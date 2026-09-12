#![allow(clippy::needless_pass_by_value)]
//! Systems executing the SSOT-001 Two-Pass Cycle across discrete tick phases.

use std::collections::HashMap;
use bevy_ecs::prelude::*;

use super::components::{Converter, ConverterBindings, FlowEdge, FlowQueue, Relocating, Storage, TransitPacket};
use super::resources::{
    CurrentTick, EdgeAllocation, EdgeDemand, FlowAllocations, PendingDemand, Recipe,
    RecipeRegistry, SimulationTime, TickErrorLog,
};
use super::types::ResourceAmount;

/// Phase 1: Polling and Demand Registration.
/// Gathers resource demand from downstream storage capacities along active flow edges,
/// strictly accounting for in-flight packets to enforce pipeline backpressure.
pub fn poll_demand_system(
    edge_query: Query<(Entity, &FlowEdge, &FlowQueue)>,
    storage_query: Query<&Storage>,
    mut pending_demand: ResMut<PendingDemand>,
) {
    pending_demand.demands.clear();

    let mut in_flight_map: HashMap<Entity, ResourceAmount> = HashMap::new();
    for (_, _, queue) in &edge_query {
        for packet in &queue.packets {
            *in_flight_map.entry(packet.destination).or_default() += packet.amount;
        }
    }

    for (edge_entity, edge, _) in &edge_query {
        let Ok(dest_storage) = storage_query.get(edge.destination) else {
            continue;
        };

        let in_flight = in_flight_map.get(&edge.destination).copied().unwrap_or(ResourceAmount::ZERO);
        let available_space = dest_storage.available_capacity().saturating_sub(in_flight);

        let requested = edge.max_throughput.min(available_space);
        if requested.is_positive() {
            *in_flight_map.entry(edge.destination).or_default() += requested;
            pending_demand.demands.push(EdgeDemand {
                edge_entity,
                source: edge.source,
                destination: edge.destination,
                requested_amount: requested,
            });
        }
    }
}

/// Helper function allocating available source inventory across competing edge demands.
fn allocate_demand_group(
    available: ResourceAmount,
    demands: &[&EdgeDemand],
    allocations: &mut Vec<EdgeAllocation>,
) {
    let mut total_demand = ResourceAmount::ZERO;
    for d in demands {
        total_demand += d.requested_amount;
    }

    if total_demand <= available {
        for d in demands {
            allocations.push(EdgeAllocation {
                edge_entity: d.edge_entity,
                allocated_amount: d.requested_amount,
            });
        }
        return;
    }

    let mut allocated_sum = ResourceAmount::ZERO;
    let count = demands.len();
    for (idx, d) in demands.iter().enumerate() {
        if idx == count - 1 {
            let remainder = available.saturating_sub(allocated_sum);
            allocations.push(EdgeAllocation {
                edge_entity: d.edge_entity,
                allocated_amount: remainder.min(d.requested_amount),
            });
        } else {
            let share = available
                .checked_mul_ratio(d.requested_amount.to_raw(), total_demand.to_raw())
                .unwrap_or(ResourceAmount::ZERO);
            allocated_sum += share;
            allocations.push(EdgeAllocation {
                edge_entity: d.edge_entity,
                allocated_amount: share,
            });
        }
    }
}

/// Phase 2: Contention Resolution and Allocation.
/// Rationing available source stocks pro-rata when aggregate demand exceeds capacity.
pub fn resolve_contention_system(
    pending_demand: Res<PendingDemand>,
    storage_query: Query<&Storage>,
    mut allocations: ResMut<FlowAllocations>,
) {
    allocations.allocations.clear();

    let mut grouped: HashMap<Entity, Vec<&EdgeDemand>> = HashMap::new();
    for demand in &pending_demand.demands {
        grouped.entry(demand.source).or_default().push(demand);
    }

    for (source, demands) in grouped {
        let available = storage_query
            .get(source)
            .map_or(ResourceAmount::ZERO, |s| s.current_amount);
        allocate_demand_group(available, &demands, &mut allocations.allocations);
    }
}

/// Dispatches allocated resource packets from source stocks into edge queues.
fn dispatch_transit_packets(
    allocations: &[EdgeAllocation],
    current_tick: u64,
    edge_query: &mut Query<(&FlowEdge, &mut FlowQueue)>,
    storage_query: &mut Query<&mut Storage>,
    error_log: &mut TickErrorLog,
) {
    for alloc in allocations {
        if !alloc.allocated_amount.is_positive() {
            continue;
        }
        let Ok((edge, mut queue)) = edge_query.get_mut(alloc.edge_entity) else {
            continue;
        };
        let Ok(mut src_storage) = storage_query.get_mut(edge.source) else {
            continue;
        };

        if let Err(e) = src_storage.withdraw(alloc.allocated_amount) {
            error_log.push(e);
            continue;
        }

        let packet_amount = alloc
            .allocated_amount
            .checked_mul_ratio(i64::from(edge.efficiency_millionths), 1_000_000)
            .unwrap_or(alloc.allocated_amount);

        queue.packets.push(TransitPacket {
            element_id: src_storage.element_id,
            amount: packet_amount,
            arrival_tick: current_tick + edge.latency,
            destination: edge.destination,
        });
    }
}

/// Delivers arrived packets from edge queues into destination storages with buffering.
fn deliver_arrived_packets(
    current_tick: u64,
    edge_query: &mut Query<(&FlowEdge, &mut FlowQueue)>,
    storage_query: &mut Query<&mut Storage>,
) {
    for (_, mut queue) in edge_query.iter_mut() {
        let mut remaining = Vec::with_capacity(queue.packets.len());
        for mut packet in queue.packets.drain(..) {
            if packet.arrival_tick <= current_tick {
                if let Ok(mut dest) = storage_query.get_mut(packet.destination) {
                    let available = dest.available_capacity();
                    let to_deposit = available.min(packet.amount);
                    if to_deposit.is_positive() {
                        let _ = dest.deposit(to_deposit);
                        packet.amount = packet.amount.saturating_sub(to_deposit);
                    }
                    if packet.amount.is_positive() {
                        packet.arrival_tick = current_tick + 1;
                        remaining.push(packet);
                    }
                } else {
                    remaining.push(packet);
                }
            } else {
                remaining.push(packet);
            }
        }
        queue.packets = remaining;
    }
}

/// Phase 3: Physical Flow and Latency Transit.
/// Executes withdrawals, packet dispatch with latency tags, and packet arrivals.
pub fn transit_flow_system(
    current_tick: Res<CurrentTick>,
    allocations: Res<FlowAllocations>,
    mut edge_query: Query<(&FlowEdge, &mut FlowQueue)>,
    mut storage_query: Query<&mut Storage>,
    mut error_log: ResMut<TickErrorLog>,
) {
    deliver_arrived_packets(
        current_tick.0,
        &mut edge_query,
        &mut storage_query,
    );
    dispatch_transit_packets(
        &allocations.allocations,
        current_tick.0,
        &mut edge_query,
        &mut storage_query,
        &mut error_log,
    );
}

/// Cleans up finished relocations from the sparse set.
pub fn relocating_lifecycle_system(
    mut commands: Commands,
    current_tick: Res<CurrentTick>,
    query: Query<(Entity, &Relocating)>,
) {
    for (entity, relocating) in &query {
        if relocating.arrival_tick <= current_tick.0 {
            commands.entity(entity).remove::<Relocating>();
        }
    }
}

/// Helper executing a single stoichiometric recipe transformation.
fn execute_single_converter(
    converter: &Converter,
    bindings: &ConverterBindings,
    recipe: &Recipe,
    storage_query: &mut Query<&mut Storage>,
    error_log: &mut TickErrorLog,
) {
    // Check input availability
    for input_req in &recipe.inputs {
        let has_input = bindings.inputs.iter().any(|&e| {
            storage_query
                .get(e)
                .is_ok_and(|s| s.element_id == input_req.element_id && s.current_amount >= input_req.amount)
        });
        if !has_input {
            return;
        }
    }

    // Check output capacity
    for output_req in &recipe.outputs {
        let has_capacity = bindings.outputs.iter().any(|&e| {
            storage_query
                .get(e)
                .is_ok_and(|s| s.element_id == output_req.element_id && s.available_capacity() >= output_req.amount)
        });
        if !has_capacity {
            return;
        }
    }

    // Withdraw inputs
    for input_req in &recipe.inputs {
        for &e in &bindings.inputs {
            if let Ok(mut s) = storage_query.get_mut(e) {
                if s.element_id == input_req.element_id && s.current_amount >= input_req.amount {
                    if let Err(err) = s.withdraw(input_req.amount) {
                        error_log.push(err);
                    }
                    break;
                }
            }
        }
    }

    // Deposit outputs with health scaling
    for output_req in &recipe.outputs {
        let yield_amt = output_req
            .amount
            .checked_mul_ratio(i64::from(converter.health), 10_000)
            .unwrap_or(output_req.amount);

        for &e in &bindings.outputs {
            if let Ok(mut s) = storage_query.get_mut(e) {
                if s.element_id == output_req.element_id && s.available_capacity() >= yield_amt {
                    if let Err(err) = s.deposit(yield_amt) {
                        error_log.push(err);
                    }
                    break;
                }
            }
        }
    }
}

/// Phase 4: Production Integration and Converter Execution.
pub fn execute_converters_system(
    recipe_registry: Res<RecipeRegistry>,
    converter_query: Query<(&Converter, &ConverterBindings)>,
    mut storage_query: Query<&mut Storage>,
    mut error_log: ResMut<TickErrorLog>,
) {
    for (converter, bindings) in &converter_query {
        if !converter.is_operational() {
            continue;
        }
        if let Some(recipe) = recipe_registry.get(converter.recipe_id) {
            execute_single_converter(
                converter,
                bindings,
                recipe,
                &mut storage_query,
                &mut error_log,
            );
        }
    }
}

/// Advances simulation tick and clock time.
pub fn advance_tick_system(
    mut current_tick: ResMut<CurrentTick>,
    mut sim_time: ResMut<SimulationTime>,
) {
    current_tick.0 += 1;
    sim_time.tick += 1;
    sim_time.elapsed_seconds += sim_time.delta_time_seconds;
}
