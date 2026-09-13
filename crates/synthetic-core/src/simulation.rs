//! Application Adapter layer connecting the Bevy ECS World to DTOs across the IPC boundary.
//!
//! Pure domain orchestration: manages the simulation tick loop, extracts serializable
//! state snapshots, and guarantees deterministic execution without external I/O.

use std::collections::HashMap;
use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};

pub use crate::dto::*;
use crate::error::DomainError;
use crate::network::{
    initialize_simulation_world, tick_simulation_world, Converter, CurrentTick, FlowEdge,
    FlowQueue, SimulationTime, Storage,
};
use crate::orbital::OrbitalState;
use crate::spatial::{AstroNode, SurfaceNode};

/// Global directory mapping resource IDs to metadata in the Bevy ECS World.
#[derive(Resource, Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDirectory {
    /// Ordered resource definitions.
    pub resources: Vec<ResourceDto>,
}

impl ResourceDirectory {
    /// Creates a new resource directory with the specified resource definitions.
    #[must_use]
    pub fn new(resources: Vec<ResourceDto>) -> Self {
        Self { resources }
    }
}

impl SimulationStateDto {
    /// Extracts a serializable DTO snapshot from the Bevy ECS World.
    #[must_use]
    pub fn from_world(world: &mut World) -> Self {
        let sim_time = *world.resource::<SimulationTime>();
        let mut orbital_query = world.query::<&OrbitalState>();
        let mut entities: Vec<OrbitalState> = orbital_query.iter(world).copied().collect();
        entities.sort_by_key(|e| e.entity_id);

        let resources = world
            .get_resource::<ResourceDirectory>()
            .map(|rd| rd.resources.clone())
            .unwrap_or_default();

        let mut storage_query = world.query::<(Entity, &Storage)>();
        let mut storages: Vec<StorageDto> = storage_query
            .iter(world)
            .map(|(entity, storage)| StorageDto {
                entity_id: entity.index(),
                resource_id: storage.element_id,
                amount: storage.current_amount.to_raw(),
                capacity: storage.capacity.to_raw(),
            })
            .collect();
        storages.sort_by_key(|s| s.entity_id);

        let mut converter_query = world.query::<(Entity, &Converter)>();
        let mut converters: Vec<ConverterDto> = converter_query
            .iter(world)
            .map(|(entity, converter)| ConverterDto {
                entity_id: entity.index(),
                recipe_id: converter.recipe_id,
            })
            .collect();
        converters.sort_by_key(|c| c.entity_id);

        let mut edge_query = world.query::<(Entity, &FlowEdge, Option<&FlowQueue>)>();
        let mut edges: Vec<FlowEdgeDto> = edge_query
            .iter(world)
            .map(|(entity, edge, queue)| {
                let in_transit = queue.map_or(0, |q| {
                    q.packets.iter().map(|p| p.amount.to_raw()).sum()
                });
                FlowEdgeDto {
                    edge_id: entity.index(),
                    source_id: edge.source.index(),
                    destination_id: edge.destination.index(),
                    in_transit,
                }
            })
            .collect();
        edges.sort_by_key(|e| e.edge_id);

        let mut astro_query = world.query::<(Entity, &AstroNode)>();
        let mut astro_nodes: Vec<AstroNodeDto> = Vec::new();
        let mut astro_body_map = HashMap::new();
        for (entity, astro) in astro_query.iter(world) {
            astro_body_map.insert(entity, astro.body_id);
            astro_nodes.push(AstroNodeDto {
                entity_id: entity.index(),
                body_id: astro.body_id,
                radius_km: astro.radius_km,
                h3_resolution: astro.h3_resolution,
            });
        }
        astro_nodes.sort_by_key(|a| a.entity_id);

        let mut surface_query = world.query::<(Entity, &SurfaceNode)>();
        let mut surface_nodes: Vec<SurfaceNodeDto> = surface_query
            .iter(world)
            .map(|(entity, surface)| {
                let parent_body_id = astro_body_map
                    .get(&surface.parent_body)
                    .copied()
                    .unwrap_or_else(|| surface.parent_body.index());
                SurfaceNodeDto {
                    entity_id: entity.index(),
                    parent_body_id,
                    h3_cell_index: surface.cell.to_string(),
                }
            })
            .collect();
        surface_nodes.sort_by_key(|s| s.entity_id);

        Self {
            tick: sim_time.tick,
            timestamp_seconds: sim_time.elapsed_seconds,
            delta_time_seconds: sim_time.delta_time_seconds,
            entities,
            resources,
            storages,
            converters,
            edges,
            astro_nodes,
            surface_nodes,
        }
    }
}

/// Spawns default celestial bodies into the given world.
fn spawn_default_orbital_entities(world: &mut World) -> Vec<OrbitalState> {
    let earth = OrbitalState {
        entity_id: 1,
        barycenter_id: 0,
        true_anomaly: 0.0,
        semi_major_axis: 1.495_98e11,
        eccentricity: 0.016_708_6,
        orbital_period: 31_558_149.0,
    };

    let mars = OrbitalState {
        entity_id: 2,
        barycenter_id: 0,
        true_anomaly: 0.5,
        semi_major_axis: 2.279_39e11,
        eccentricity: 0.0934,
        orbital_period: 59_355_072.0,
    };

    let earth_entity = world.spawn((earth, AstroNode::new(1, 6371, 3))).id();
    let mars_entity = world.spawn((mars, AstroNode::new(2, 3389, 2))).id();

    // Spawn sample surface facilities (London, Tokyo on Earth; Olympus Mons on Mars)
    if let (Ok(coord_london), Ok(coord_tokyo)) = (
        h3o::LatLng::new(51.5074, -0.1278),
        h3o::LatLng::new(35.6762, 139.6503),
    ) {
        let cell_london = coord_london.to_cell(h3o::Resolution::Three);
        let cell_tokyo = coord_tokyo.to_cell(h3o::Resolution::Three);
        world.spawn(SurfaceNode::new(earth_entity, cell_london));
        world.spawn(SurfaceNode::new(earth_entity, cell_tokyo));
    }

    if let Ok(coord_olympus) = h3o::LatLng::new(18.65, -133.8) {
        let cell_olympus = coord_olympus.to_cell(h3o::Resolution::Two);
        world.spawn(SurfaceNode::new(mars_entity, cell_olympus));
    }

    vec![earth, mars]
}

/// Constructs and populates a Bevy ECS World from an existing simulation state DTO.
#[must_use]
pub fn build_simulation_world_from_dto(
    state: &SimulationStateDto,
    delta_time_seconds: f64,
) -> World {
    let mut world = initialize_simulation_world();
    world.insert_resource(CurrentTick(state.tick));
    world.insert_resource(SimulationTime {
        tick: state.tick,
        delta_time_seconds,
        elapsed_seconds: state.timestamp_seconds,
    });

    let mut astro_entity_map = HashMap::new();
    if state.astro_nodes.is_empty() {
        for entity in &state.entities {
            world.spawn(*entity);
        }
    } else {
        for astro in &state.astro_nodes {
            let matching_orbital = state.entities.iter().find(|e| e.entity_id == u64::from(astro.body_id));
            let ent = if let Some(orbital) = matching_orbital {
                world.spawn((*orbital, AstroNode::new(astro.body_id, astro.radius_km, astro.h3_resolution))).id()
            } else {
                world.spawn(AstroNode::new(astro.body_id, astro.radius_km, astro.h3_resolution)).id()
            };
            astro_entity_map.insert(astro.body_id, ent);
        }
        for entity in &state.entities {
            if !state.astro_nodes.iter().any(|a| u64::from(a.body_id) == entity.entity_id) {
                world.spawn(*entity);
            }
        }
    }

    for surface in &state.surface_nodes {
        if let Ok(cell) = surface.h3_cell_index.parse::<h3o::CellIndex>() {
            let parent_entity = astro_entity_map
                .get(&surface.parent_body_id)
                .copied()
                .unwrap_or(Entity::PLACEHOLDER);
            world.spawn(SurfaceNode::new(parent_entity, cell));
        }
    }

    crate::generator::generate_economic_scenario(&mut world);
    world
}

/// Pure simulation step function: executes the Bevy ECS World schedule for one tick.
///
/// # Errors
/// Returns `DomainError` if any domain validation, orbital calculation, or network system fails.
pub fn step_simulation(
    current_state: &SimulationStateDto,
    delta_time_seconds: f64,
) -> Result<SimulationStateDto, DomainError> {
    let mut world = build_simulation_world_from_dto(current_state, delta_time_seconds);
    tick_simulation_world(&mut world)?;
    Ok(SimulationStateDto::from_world(&mut world))
}

/// Persistent simulation session holding the active Bevy ECS World across ticks.
pub struct SimulationSession {
    /// Internal Bevy ECS World.
    pub world: World,
}

impl SimulationSession {
    /// Initializes a new simulation session with standard celestial baseline bodies
    /// and the deterministic 200-node economic network.
    #[must_use]
    pub fn new() -> Self {
        let mut world = initialize_simulation_world();
        spawn_default_orbital_entities(&mut world);
        crate::generator::generate_economic_scenario(&mut world);
        Self { world }
    }

    /// Initializes a session from an explicit simulation state DTO.
    #[must_use]
    pub fn from_dto(state: &SimulationStateDto) -> Self {
        let world = build_simulation_world_from_dto(state, state.delta_time_seconds);
        Self { world }
    }

    /// Advances the internal simulation session by one tick.
    ///
    /// # Errors
    /// Returns `DomainError` if any domain calculation fails during the tick.
    pub fn step(&mut self, delta_time_seconds: f64) -> Result<SimulationStateDto, DomainError> {
        if let Some(mut sim_time) = self.world.get_resource_mut::<SimulationTime>() {
            sim_time.delta_time_seconds = delta_time_seconds;
        }
        tick_simulation_world(&mut self.world)?;
        Ok(SimulationStateDto::from_world(&mut self.world))
    }

    /// Extracts the active state DTO snapshot.
    pub fn extract_dto(&mut self) -> SimulationStateDto {
        SimulationStateDto::from_world(&mut self.world)
    }
}

impl Default for SimulationSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a baseline initial simulation state with standard celestial bodies and economic network.
#[must_use]
pub fn create_default_simulation_state() -> SimulationStateDto {
    let mut session = SimulationSession::new();
    session.extract_dto()
}

