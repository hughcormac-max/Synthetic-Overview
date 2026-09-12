use std::f64::consts::PI;
use serde::{Deserialize, Serialize};
use crate::error::DomainError;

const TWO_PI: f64 = 2.0 * PI;
const MAX_KEPLER_ITERATIONS: usize = 50;
const KEPLER_CONVERGENCE_EPSILON: f64 = 1e-13;

/// Core deterministic Keplerian orbital state model.
/// Managed strictly in the Rust Tier 0 domain layer.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OrbitalState {
    pub entity_id: u64,
    pub barycenter_id: u64,
    pub true_anomaly: f64,
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub orbital_period: f64,
}

impl OrbitalState {
    /// Constructs and validates a new orbital state instance.
    ///
    /// # Errors
    /// Returns `DomainError::InvalidEccentricity` if `eccentricity < 0.0` or `eccentricity >= 1.0`.
    /// Returns `DomainError::InvalidOrbitalPeriod` if `orbital_period <= 0.0`.
    /// Returns `DomainError::InvalidSemiMajorAxis` if `semi_major_axis <= 0.0`.
    pub fn new(
        entity_id: u64,
        barycenter_id: u64,
        true_anomaly: f64,
        semi_major_axis: f64,
        eccentricity: f64,
        orbital_period: f64,
    ) -> Result<Self, DomainError> {
        let state = Self {
            entity_id,
            barycenter_id,
            true_anomaly: normalize_angle(true_anomaly),
            semi_major_axis,
            eccentricity,
            orbital_period,
        };
        state.validate()?;
        Ok(state)
    }

    /// Validates orbital boundary parameters according to physical invariants.
    ///
    /// # Errors
    /// Returns `DomainError::InvalidEccentricity` if `eccentricity < 0.0` or `eccentricity >= 1.0`.
    /// Returns `DomainError::InvalidOrbitalPeriod` if `orbital_period <= 0.0`.
    /// Returns `DomainError::InvalidSemiMajorAxis` if `semi_major_axis <= 0.0`.
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.eccentricity < 0.0 || self.eccentricity >= 1.0 {
            return Err(DomainError::InvalidEccentricity(format!(
                "{}",
                self.eccentricity
            )));
        }
        if self.orbital_period <= 0.0 {
            return Err(DomainError::InvalidOrbitalPeriod(format!(
                "{}",
                self.orbital_period
            )));
        }
        if self.semi_major_axis <= 0.0 {
            return Err(DomainError::InvalidSemiMajorAxis(format!(
                "{}",
                self.semi_major_axis
            )));
        }
        Ok(())
    }
}

/// Normalizes any angle into the half-open interval [0.0, 2.0 * PI).
#[must_use]
pub fn normalize_angle(angle: f64) -> f64 {
    let remainder = angle % TWO_PI;
    if remainder < 0.0 {
        remainder + TWO_PI
    } else {
        remainder
    }
}

/// Converts true anomaly (nu) to eccentric anomaly (E) using atan2.
/// Formulation:
///   sin(E) = sqrt(1 - e^2) * sin(nu) / (1 + e * cos(nu))
///   cos(E) = (e + cos(nu)) / (1 + e * cos(nu))
///   E = atan2(sqrt(1 - e^2) * sin(nu), e + cos(nu))
#[must_use]
pub fn true_anomaly_to_eccentric_anomaly(true_anomaly: f64, eccentricity: f64) -> f64 {
    let sin_nu = true_anomaly.sin();
    let cos_nu = true_anomaly.cos();
    let beta = (1.0 - eccentricity * eccentricity).sqrt();
    let y = beta * sin_nu;
    let x = eccentricity + cos_nu;
    normalize_angle(y.atan2(x))
}

/// Converts eccentric anomaly (E) to mean anomaly (M) using Kepler equation:
///   M = E - e * sin(E)
#[must_use]
pub fn eccentric_anomaly_to_mean_anomaly(eccentric_anomaly: f64, eccentricity: f64) -> f64 {
    let mean_anomaly = eccentric_anomaly - eccentricity * eccentric_anomaly.sin();
    normalize_angle(mean_anomaly)
}

/// Solves Kepler equation for eccentric anomaly (E) given mean anomaly (M) and eccentricity:
///   M = E - e * sin(E)
/// Utilizes the Newton-Raphson method with quadratic convergence.
///
/// # Errors
/// Returns `DomainError::ConvergenceFailure` if the iteration fails to converge within 50 cycles.
pub fn mean_anomaly_to_eccentric_anomaly(
    mean_anomaly: f64,
    eccentricity: f64,
) -> Result<f64, DomainError> {
    let normalized_m = normalize_angle(mean_anomaly);

    // Initial estimate for Newton-Raphson solver
    let mut e_est = if eccentricity < 0.8 {
        normalized_m
    } else {
        PI
    };

    for _ in 0..MAX_KEPLER_ITERATIONS {
        let f = e_est - eccentricity * e_est.sin() - normalized_m;
        let f_prime = 1.0 - eccentricity * e_est.cos();
        let delta = f / f_prime;
        e_est -= delta;

        if delta.abs() < KEPLER_CONVERGENCE_EPSILON {
            return Ok(normalize_angle(e_est));
        }
    }

    Err(DomainError::ConvergenceFailure(format!(
        "Mean anomaly: {mean_anomaly}, Eccentricity: {eccentricity}"
    )))
}

/// Converts eccentric anomaly (E) to true anomaly (nu) using atan2:
///   sin(nu) = sqrt(1 - e^2) * sin(E) / (1 - e * cos(E))
///   cos(nu) = (cos(E) - e) / (1 - e * cos(E))
///   nu = atan2(sqrt(1 - e^2) * sin(E), cos(E) - e)
#[must_use]
pub fn eccentric_anomaly_to_true_anomaly(eccentric_anomaly: f64, eccentricity: f64) -> f64 {
    let sin_e = eccentric_anomaly.sin();
    let cos_e = eccentric_anomaly.cos();
    let beta = (1.0 - eccentricity * eccentricity).sqrt();
    let y = beta * sin_e;
    let x = cos_e - eccentricity;
    normalize_angle(y.atan2(x))
}

/// Computes 2D planar position (x, y) relative to the focal barycenter.
/// Formula:
///   r = a * (1 - e^2) / (1 + e * cos(nu))
///   x = r * cos(nu)
///   y = r * sin(nu)
#[must_use]
pub fn calculate_orbital_position(state: &OrbitalState) -> (f64, f64) {
    let e = state.eccentricity;
    let a = state.semi_major_axis;
    let nu = state.true_anomaly;

    let semi_latus_rectum = a * (1.0 - e * e);
    let radius = semi_latus_rectum / (1.0 + e * nu.cos());
    let x = radius * nu.cos();
    let y = radius * nu.sin();
    (x, y)
}

/// Propagates an orbital state forward by `delta_time_seconds` using deterministic Keplerian motion.
/// Pure function: produces a new `OrbitalState` without mutating the input instance.
///
/// # Errors
/// Returns `DomainError` if the state fails validation or if Kepler solver fails to converge.
pub fn propagate_orbit(
    state: &OrbitalState,
    delta_time_seconds: f64,
) -> Result<OrbitalState, DomainError> {
    state.validate()?;

    // Mean angular motion: n = 2.0 * PI / orbital_period
    let mean_motion = TWO_PI / state.orbital_period;

    // Convert current true anomaly to eccentric and mean anomalies
    let current_e = true_anomaly_to_eccentric_anomaly(state.true_anomaly, state.eccentricity);
    let current_m = eccentric_anomaly_to_mean_anomaly(current_e, state.eccentricity);

    // Advance mean anomaly linearly by mean motion * delta_time
    let next_m = current_m + mean_motion * delta_time_seconds;

    // Solve Kepler equation for next eccentric anomaly
    let next_e = mean_anomaly_to_eccentric_anomaly(next_m, state.eccentricity)?;

    // Map next eccentric anomaly back to true anomaly
    let next_true_anomaly = eccentric_anomaly_to_true_anomaly(next_e, state.eccentricity);

    Ok(OrbitalState {
        entity_id: state.entity_id,
        barycenter_id: state.barycenter_id,
        true_anomaly: next_true_anomaly,
        semi_major_axis: state.semi_major_axis,
        eccentricity: state.eccentricity,
        orbital_period: state.orbital_period,
    })
}
