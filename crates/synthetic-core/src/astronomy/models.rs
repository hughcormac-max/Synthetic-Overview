//! `AstroNode` and `SurfaceNode` core data models.
//!
//! Grounded in SSOT-PHY-004 (`AstroNode` Architecture) and SSOT-SYS-000 (SI Units).
//! Uses strict Zero-LaTeX plain text notation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstroNodeConfig {
    pub name: String,
    pub parent_name: Option<String>,
    pub mass_kg: f64,
    pub radius_m: f64,
    pub semi_major_axis_m: f64,
    pub eccentricity: f64,
    pub true_anomaly_epoch_rad: f64,
    pub longitude_of_periapsis_rad: f64,
    pub mean_motion_rad_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SurfaceNode {
    pub local_x: f64,
    pub local_y: f64,
    pub local_z: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AstroNode {
    pub config: AstroNodeConfig,
    pub surface_nodes: Vec<SurfaceNode>,
}

