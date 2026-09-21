//! 2D Keplerian orbital kinematics and ephemeris calculations.
//!
//! Grounded in SSOT-PHY-001 Section 2.1 and SSOT-PHY-004 Section 2.2.
//! Evaluates closed-form positions for celestial bodies (`AstroNodes`) at tick `t` or epoch `t = 0`.
//! Adheres strictly to Zero-LaTeX plain text formatting and pure function decoupling.

use super::models::AstroNodeConfig;
use serde::{Deserialize, Serialize};

/// Global 3D Cartesian position in meters.
///
/// While orbital trajectories are constrained to the ecliptic plane (z = 0.0),
/// the state representation supports 3D coordinates for surface nodes and spatial queries.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GlobalPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GlobalPosition {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for GlobalPosition {
    fn default() -> Self {
        Self::ZERO
    }
}

/// Calculates the global position of an `AstroNode` at time `time_s` seconds since epoch.
///
/// Grounded in SSOT-PHY-004 Section 2.2:
/// - Mean anomaly: `M = (nu_0 + n * t) mod TWO_PI`
/// - Solves Kepler's equation `M = E - e * sin(E)` via Newton-Raphson iteration.
/// - Computes true anomaly `nu` and orbital radius `r`.
/// - Resolves parent-relative 2D coordinates (`X_rel`, `Y_rel`, `0.0`) with longitude of periapsis `varpi`.
/// - Offsets by parent body global coordinates.
#[must_use]
pub fn calculate_kepler_position(
    config: &AstroNodeConfig,
    parent_pos: GlobalPosition,
    time_s: f64,
) -> GlobalPosition {
    if config.semi_major_axis_m <= 0.0 {
        return parent_pos;
    }

    let two_pi = 2.0 * std::f64::consts::PI;
    let e = config.eccentricity.clamp(0.0, 0.999_999);
    let m = (config.true_anomaly_epoch_rad + config.mean_motion_rad_s * time_s).rem_euclid(two_pi);

    // Solve Kepler's equation M = E - e * sin(E)
    let mut eccentric_anomaly = m;
    for _ in 0..15 {
        let delta = (eccentric_anomaly - e * eccentric_anomaly.sin() - m)
            / (1.0 - e * eccentric_anomaly.cos());
        eccentric_anomaly -= delta;
        if delta.abs() < 1e-12 {
            break;
        }
    }

    let sin_half_e = (eccentric_anomaly * 0.5).sin();
    let cos_half_e = (eccentric_anomaly * 0.5).cos();
    let true_anomaly = 2.0 * ((1.0 + e).sqrt() * sin_half_e).atan2((1.0 - e).sqrt() * cos_half_e);

    let radius = config.semi_major_axis_m * (1.0 - e * eccentric_anomaly.cos());
    let angle = true_anomaly + config.longitude_of_periapsis_rad;

    let x_rel = radius * angle.cos();
    let y_rel = radius * angle.sin();

    GlobalPosition {
        x: parent_pos.x + x_rel,
        y: parent_pos.y + y_rel,
        z: parent_pos.z,
    }
}

/// Calculates the global position of an `AstroNode` at epoch `t = 0`.
///
/// Grounded in SSOT-PHY-001 and SSOT-PHY-004.
#[must_use]
pub fn calculate_epoch_position(
    config: &AstroNodeConfig,
    parent_pos: GlobalPosition,
) -> GlobalPosition {
    calculate_kepler_position(config, parent_pos, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_ast_01_earth_epoch_position() {
        let earth_config = AstroNodeConfig {
            name: "Earth".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 5.972e24,
            radius_m: 6.371e6,
            semi_major_axis_m: 1.495_978_7e11,
            eccentricity: 0.0,
            true_anomaly_epoch_rad: 0.0,
            longitude_of_periapsis_rad: 0.0,
            mean_motion_rad_s: 1.991e-7,
        };

        let pos = calculate_epoch_position(&earth_config, GlobalPosition::ZERO);
        assert!((pos.x - 1.495_978_7e11).abs() < 1.0);
        assert!(pos.y.abs() < 1.0);
        assert!(pos.z.abs() < f64::EPSILON);
    }

    #[test]
    fn test_vec_ast_02_quarter_orbit() {
        let orbital_period_s = 31_556_940.0;
        let mean_motion = 2.0 * std::f64::consts::PI / orbital_period_s;

        let earth_config = AstroNodeConfig {
            name: "Earth".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 5.972e24,
            radius_m: 6.371e6,
            semi_major_axis_m: 1.495_978_7e11,
            eccentricity: 0.0,
            true_anomaly_epoch_rad: 0.0,
            longitude_of_periapsis_rad: 0.0,
            mean_motion_rad_s: mean_motion,
        };

        let t_quarter = orbital_period_s / 4.0;
        let pos = calculate_kepler_position(&earth_config, GlobalPosition::ZERO, t_quarter);
        assert!(pos.x.abs() < 1000.0, "Expected x approx 0, got {}", pos.x);
        assert!(
            (pos.y - 1.495_978_7e11).abs() < 1000.0,
            "Expected y approx 1.4959787e11, got {}",
            pos.y
        );
        assert!(pos.z.abs() < f64::EPSILON);
    }

    #[test]
    fn test_moon_relative_to_parent() {
        let earth_pos = GlobalPosition::new(1.495_978_7e11, 0.0, 0.0);
        let luna_config = AstroNodeConfig {
            name: "Luna".into(),
            parent_name: Some("Earth".into()),
            mass_kg: 7.342e22,
            radius_m: 1.7374e6,
            semi_major_axis_m: 3.844e8,
            eccentricity: 0.0,
            true_anomaly_epoch_rad: 0.0,
            longitude_of_periapsis_rad: 0.0,
            mean_motion_rad_s: 2.662e-6,
        };

        let luna_pos = calculate_epoch_position(&luna_config, earth_pos);
        assert!((luna_pos.x - (1.495_978_7e11 + 3.844e8)).abs() < 1.0);
        assert!(luna_pos.y.abs() < 1.0);
        assert!(luna_pos.z.abs() < f64::EPSILON);
    }

    #[test]
    fn test_root_body_sun() {
        let sun_config = AstroNodeConfig {
            name: "Sun".into(),
            parent_name: None,
            mass_kg: 1.989e30,
            radius_m: 6.9634e8,
            semi_major_axis_m: 0.0,
            eccentricity: 0.0,
            true_anomaly_epoch_rad: 0.0,
            longitude_of_periapsis_rad: 0.0,
            mean_motion_rad_s: 0.0,
        };

        let sun_pos = calculate_epoch_position(&sun_config, GlobalPosition::ZERO);
        assert_eq!(sun_pos, GlobalPosition::ZERO);
    }
}
