//! Stock-and-Flow network module implemented with Bevy ECS.
//!
//! Models discrete storage accumulators, transmission edges, stoichiometric
//! converters, and dynamic transient tags (`SparseSet`) across a strict Two-Pass Cycle.

pub mod components;
pub mod resources;
pub mod systems;
pub mod types;

pub use components::{
    Converter, ConverterBindings, FlowEdge, FlowQueue, Relocating, Storage, TransitPacket,
};
pub use resources::{
    CurrentTick, EdgeAllocation, EdgeDemand, FlowAllocations, PendingDemand, Recipe,
    RecipeIngredient, RecipeRegistry, SimulationTime, TickErrorLog,
};
pub use systems::{
    advance_tick_system, execute_converters_system, poll_demand_system,
    relocating_lifecycle_system, resolve_contention_system, transit_flow_system,
};
pub use types::ResourceAmount;

use bevy_ecs::prelude::*;
use crate::error::DomainError;

/// Builds the deterministic Two-Pass Cycle schedule strictly ordered via chaining.
#[must_use]
pub fn create_simulation_schedule() -> Schedule {
    let mut schedule = Schedule::default();
    schedule.add_systems(
        (
            advance_tick_system,
            crate::orbital::propagate_orbits_system,
            poll_demand_system,
            resolve_contention_system,
            transit_flow_system,
            relocating_lifecycle_system,
            execute_converters_system,
        )
            .chain(),
    );
    schedule
}

/// Initializes the Bevy ECS World and core simulation resources.
#[must_use]
pub fn initialize_simulation_world() -> World {
    let mut world = World::new();
    world.insert_resource(CurrentTick(0));
    world.insert_resource(SimulationTime {
        tick: 0,
        delta_time_seconds: 86_400.0,
        elapsed_seconds: 0.0,
    });
    world.insert_resource(RecipeRegistry::new());
    world.insert_resource(PendingDemand::default());
    world.insert_resource(FlowAllocations::default());
    world.insert_resource(TickErrorLog::default());
    world
}

/// Core tick pipeline executing the ordered SSOT-001 phases.
///
/// # Errors
/// Returns `DomainError` if an unhandled domain error occurred during systems execution.
pub fn tick_simulation_world(world: &mut World) -> Result<(), DomainError> {
    let mut schedule = create_simulation_schedule();
    schedule.run(world);

    if let Some(mut error_log) = world.get_resource_mut::<TickErrorLog>() {
        if let Some(first_err) = error_log.take_first() {
            return Err(first_err);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_world_initialization() {
        let world = initialize_simulation_world();
        assert_eq!(world.resource::<CurrentTick>().0, 0);
        assert_eq!(world.resource::<SimulationTime>().tick, 0);
    }

    #[test]
    fn test_flow_transit_latency_and_conservation() {
        let mut world = initialize_simulation_world();

        let stock_a = world
            .spawn(Storage::new(1, ResourceAmount::from_raw(1_000), ResourceAmount::from_raw(1_000)).unwrap())
            .id();
        let stock_b = world
            .spawn(Storage::new(1, ResourceAmount::ZERO, ResourceAmount::from_raw(1_000)).unwrap())
            .id();

        let edge = world
            .spawn((
                FlowEdge {
                    source: stock_a,
                    destination: stock_b,
                    max_throughput: ResourceAmount::from_raw(200),
                    latency: 2,
                    efficiency_millionths: 1_000_000,
                },
                FlowQueue::default(),
            ))
            .id();

        // Tick 0 -> 1: Demand 200 polled, 200 allocated, dispatched with arrival_tick = 3 (1 + 2)
        tick_simulation_world(&mut world).expect("tick ok");
        assert_eq!(world.resource::<CurrentTick>().0, 1);
        assert_eq!(world.get::<Storage>(stock_a).unwrap().current_amount.to_raw(), 800);
        assert_eq!(world.get::<Storage>(stock_b).unwrap().current_amount.to_raw(), 0);
        let queue = world.get::<FlowQueue>(edge).unwrap();
        assert_eq!(queue.packets.len(), 1);
        assert_eq!(queue.packets[0].amount.to_raw(), 200);
        assert_eq!(queue.packets[0].arrival_tick, 3);

        // Tick 1 -> 2: At current_tick 2, packet is still in flight
        tick_simulation_world(&mut world).expect("tick ok");
        assert_eq!(world.resource::<CurrentTick>().0, 2);
        assert_eq!(world.get::<Storage>(stock_b).unwrap().current_amount.to_raw(), 0);

        // Tick 2 -> 3: At current_tick 3, packet 1 arrives!
        tick_simulation_world(&mut world).expect("tick ok");
        assert_eq!(world.resource::<CurrentTick>().0, 3);
        assert_eq!(world.get::<Storage>(stock_b).unwrap().current_amount.to_raw(), 200);
    }

    #[test]
    fn test_contention_resolution_pro_rata() {
        let mut world = initialize_simulation_world();

        let source = world
            .spawn(Storage::new(1, ResourceAmount::from_raw(100), ResourceAmount::from_raw(100)).unwrap())
            .id();
        let dest1 = world
            .spawn(Storage::new(1, ResourceAmount::ZERO, ResourceAmount::from_raw(100)).unwrap())
            .id();
        let dest2 = world
            .spawn(Storage::new(1, ResourceAmount::ZERO, ResourceAmount::from_raw(100)).unwrap())
            .id();

        world.spawn((
            FlowEdge {
                source,
                destination: dest1,
                max_throughput: ResourceAmount::from_raw(100),
                latency: 1,
                efficiency_millionths: 1_000_000,
            },
            FlowQueue::default(),
        ));
        world.spawn((
            FlowEdge {
                source,
                destination: dest2,
                max_throughput: ResourceAmount::from_raw(100),
                latency: 1,
                efficiency_millionths: 1_000_000,
            },
            FlowQueue::default(),
        ));

        // Demand is 200 total against 100 available stock -> pro-rata 50/50
        tick_simulation_world(&mut world).expect("tick ok");
        assert_eq!(world.get::<Storage>(source).unwrap().current_amount.to_raw(), 0);
    }

    #[test]
    fn test_converter_transformation() {
        let mut world = initialize_simulation_world();

        let mut recipes = RecipeRegistry::new();
        recipes.register(Recipe {
            id: 1,
            inputs: vec![RecipeIngredient {
                element_id: 1,
                amount: ResourceAmount::from_raw(10),
            }],
            outputs: vec![RecipeIngredient {
                element_id: 2,
                amount: ResourceAmount::from_raw(10),
            }],
        });
        world.insert_resource(recipes);

        let input_stock = world
            .spawn(Storage::new(1, ResourceAmount::from_raw(50), ResourceAmount::from_raw(100)).unwrap())
            .id();
        let output_stock = world
            .spawn(Storage::new(2, ResourceAmount::ZERO, ResourceAmount::from_raw(100)).unwrap())
            .id();

        world.spawn((
            Converter::new(1, 10_000, true),
            ConverterBindings::new(vec![input_stock], vec![output_stock]),
        ));

        tick_simulation_world(&mut world).expect("tick ok");

        assert_eq!(world.get::<Storage>(input_stock).unwrap().current_amount.to_raw(), 40);
        assert_eq!(world.get::<Storage>(output_stock).unwrap().current_amount.to_raw(), 10);
    }

    #[test]
    fn test_relocating_lifecycle() {
        let mut world = initialize_simulation_world();

        let target = world.spawn_empty().id();
        let mover = world
            .spawn(Relocating {
                destination_id: target,
                arrival_tick: 1,
            })
            .id();

        assert!(world.get::<Relocating>(mover).is_some());
        tick_simulation_world(&mut world).expect("tick ok");
        // At tick 1, Relocating is removed
        assert!(world.get::<Relocating>(mover).is_none());
    }
}


