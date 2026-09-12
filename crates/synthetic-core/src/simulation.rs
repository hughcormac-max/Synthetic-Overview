//! Application Adapter layer connecting the Bevy ECS World to DTOs across the IPC boundary.
//!
//! Pure domain orchestration: manages the simulation tick loop, extracts serializable
//! state snapshots, and guarantees deterministic execution without external I/O.

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::error::DomainError;
use crate::network::{
    initialize_simulation_world, tick_simulation_world, CurrentTick, SimulationTime,
};
use crate::orbital::OrbitalState;

/// Serializable DTO conveying simulation state across the Tauri IPC boundary to the webview.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStateDto {
    /// Active simulation tick.
    pub tick: u64,
    /// Total elapsed simulation time in seconds.
    pub timestamp_seconds: f64,
    /// Step interval in seconds for this tick.
    pub delta_time_seconds: f64,
    /// Active celestial bodies with orbital propagation parameters.
    pub entities: Vec<OrbitalState>,
}

impl SimulationStateDto {
    /// Extracts a serializable DTO snapshot from the Bevy ECS World.
    #[must_use]
    pub fn from_world(world: &mut World) -> Self {
        let sim_time = *world.resource::<SimulationTime>();
        let mut query = world.query::<&OrbitalState>();
        let mut entities: Vec<OrbitalState> = query.iter(world).copied().collect();
        entities.sort_by_key(|e| e.entity_id);

        Self {
            tick: sim_time.tick,
            timestamp_seconds: sim_time.elapsed_seconds,
            delta_time_seconds: sim_time.delta_time_seconds,
            entities,
        }
    }
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

    for entity in &state.entities {
        world.spawn(*entity);
    }
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
    /// Initializes a new simulation session with standard celestial baseline bodies.
    #[must_use]
    pub fn new() -> Self {
        let default_state = create_default_simulation_state();
        let world = build_simulation_world_from_dto(
            &default_state,
            default_state.delta_time_seconds,
        );
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

/// Creates a baseline initial simulation state with standard celestial bodies.
#[must_use]
pub fn create_default_simulation_state() -> SimulationStateDto {
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

    SimulationStateDto {
        tick: 0,
        timestamp_seconds: 0.0,
        delta_time_seconds: 86_400.0,
        entities: vec![earth, mars],
    }
}
