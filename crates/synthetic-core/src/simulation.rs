use serde::{Deserialize, Serialize};
use crate::error::DomainError;
use crate::orbital::{propagate_orbit, OrbitalState};

/// Serializable DTO conveying simulation state across the Tauri IPC boundary to the webview.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStateDto {
    pub tick: u64,
    pub timestamp_seconds: f64,
    pub delta_time_seconds: f64,
    pub entities: Vec<OrbitalState>,
}

/// Pure simulation step function: advances simulation state by one tick.
/// Pure, deterministic, zero external I/O.
///
/// # Errors
/// Returns `DomainError` if any entity fails orbital propagation.
pub fn step_simulation(
    current_state: &SimulationStateDto,
    delta_time_seconds: f64,
) -> Result<SimulationStateDto, DomainError> {
    let mut updated_entities = Vec::with_capacity(current_state.entities.len());

    for entity in &current_state.entities {
        let updated = propagate_orbit(entity, delta_time_seconds)?;
        updated_entities.push(updated);
    }

    Ok(SimulationStateDto {
        tick: current_state.tick + 1,
        timestamp_seconds: current_state.timestamp_seconds + delta_time_seconds,
        delta_time_seconds,
        entities: updated_entities,
    })
}

/// Creates a baseline initial simulation state with standard celestial bodies.
#[must_use]
pub fn create_default_simulation_state() -> SimulationStateDto {
    // Earth: a = 1.49598e11 m, e = 0.0167086, period = 31558149.0 s (approx 365.256 days)
    let earth = OrbitalState {
        entity_id: 1,
        barycenter_id: 0,
        true_anomaly: 0.0,
        semi_major_axis: 1.495_98e11,
        eccentricity: 0.016_708_6,
        orbital_period: 31_558_149.0,
    };

    // Mars: a = 2.27939e11 m, e = 0.0934, period = 59355072.0 s (approx 686.98 days)
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
