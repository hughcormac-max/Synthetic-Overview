//! Tier 0 Domain Layer: Pure Rust simulation engine and Keplerian orbital mechanics.
//!
//! Architectural Invariants:
//! 1. Zero external I/O or UI dependencies.
//! 2. Deterministic, pure calculation pipelines without side effects.
//! 3. Plain-text and ASCII math notation only (strict Zero-LaTeX compliance).

pub mod error;
pub mod network;
pub mod orbital;
pub mod simulation;

pub use error::DomainError;
pub use network::{
    Converter, CurrentTick, FlowEdge, FlowQueue, Recipe, RecipeIngredient, RecipeRegistry,
    Relocating, ResourceAmount, SimulationTime, Storage, TransitPacket,
};
pub use orbital::{
    calculate_orbital_position, eccentric_anomaly_to_mean_anomaly,
    eccentric_anomaly_to_true_anomaly, mean_anomaly_to_eccentric_anomaly, normalize_angle,
    propagate_orbit, true_anomaly_to_eccentric_anomaly, OrbitalState,
};
pub use simulation::{build_simulation_world_from_dto, create_default_simulation_state, step_simulation, SimulationSession, SimulationStateDto};

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-11;

    #[test]
    fn test_orbital_state_validation_valid() {
        let state = OrbitalState::new(1, 0, 0.0, 1.496e11, 0.0167, 31_558_149.0);
        assert!(state.is_ok());
    }

    #[test]
    fn test_orbital_state_validation_invalid_eccentricity() {
        let parabolic = OrbitalState::new(1, 0, 0.0, 1.496e11, 1.0, 31_558_149.0);
        assert!(matches!(parabolic, Err(DomainError::InvalidEccentricity(_))));

        let negative = OrbitalState::new(1, 0, 0.0, 1.496e11, -0.1, 31_558_149.0);
        assert!(matches!(negative, Err(DomainError::InvalidEccentricity(_))));
    }

    #[test]
    fn test_orbital_state_validation_invalid_period() {
        let zero_period = OrbitalState::new(1, 0, 0.0, 1.496e11, 0.05, 0.0);
        assert!(matches!(zero_period, Err(DomainError::InvalidOrbitalPeriod(_))));
    }

    #[test]
    fn test_kepler_solver_roundtrip() {
        let eccentricity = 0.25;
        let test_mean_anomalies = [0.0, 0.25 * PI, 0.5 * PI, PI, 1.5 * PI, 1.9 * PI];

        for &m_orig in &test_mean_anomalies {
            let e = mean_anomaly_to_eccentric_anomaly(m_orig, eccentricity).expect("solver converged");
            let m_recovered = eccentric_anomaly_to_mean_anomaly(e, eccentricity);
            let diff = (m_recovered - m_orig).abs();
            assert!(
                diff < EPSILON,
                "Mean anomaly roundtrip failed for m = {m_orig}: diff = {diff}"
            );
        }
    }

    #[test]
    fn test_anomaly_conversion_roundtrip() {
        let eccentricity = 0.15;
        let test_true_anomalies = [0.0, 0.3 * PI, 0.7 * PI, PI, 1.4 * PI, 1.8 * PI];

        for &nu_orig in &test_true_anomalies {
            let e = true_anomaly_to_eccentric_anomaly(nu_orig, eccentricity);
            let nu_recovered = eccentric_anomaly_to_true_anomaly(e, eccentricity);
            let diff = (nu_recovered - nu_orig).abs();
            assert!(
                diff < EPSILON,
                "True anomaly roundtrip failed for nu = {nu_orig}: diff = {diff}"
            );
        }
    }

    #[test]
    fn test_propagation_zero_drift_full_orbit() {
        // Propagating for exactly one orbital period must return true anomaly to initial value
        let period = 31_558_149.0;
        let initial_state = OrbitalState::new(10, 0, 0.42, 1.496e11, 0.05, period)
            .expect("valid orbital state");

        let final_state = propagate_orbit(&initial_state, period)
            .expect("propagation succeeded");

        let diff = (final_state.true_anomaly - initial_state.true_anomaly).abs();
        assert!(
            diff < EPSILON,
            "Full orbit propagation drifted: initial = {}, final = {}, diff = {}",
            initial_state.true_anomaly,
            final_state.true_anomaly,
            diff
        );
    }

    #[test]
    fn test_propagation_half_orbit_symmetry() {
        // Starting at periapsis (nu = 0), a half period must reach apoapsis (nu = PI)
        let period = 100_000.0;
        let periapsis = OrbitalState::new(1, 0, 0.0, 5e7, 0.1, period)
            .expect("valid orbital state");

        let apoapsis = propagate_orbit(&periapsis, period / 2.0)
            .expect("propagation succeeded");

        let diff = (apoapsis.true_anomaly - PI).abs();
        assert!(
            diff < EPSILON,
            "Half orbit did not arrive at PI: arrived at {}, diff = {}",
            apoapsis.true_anomaly,
            diff
        );
    }

    #[test]
    fn test_simulation_stepping() {
        let default_state = create_default_simulation_state();
        assert_eq!(default_state.tick, 0);
        assert_eq!(default_state.entities.len(), 2);

        let stepped = step_simulation(&default_state, 86400.0).expect("step ok");
        assert_eq!(stepped.tick, 1);
        assert!((stepped.timestamp_seconds - 86_400.0).abs() < f64::EPSILON);
        assert_eq!(stepped.entities.len(), 2);

        // Ensure entities advanced deterministically
        assert!(stepped.entities[0].true_anomaly > default_state.entities[0].true_anomaly);
    }
}



