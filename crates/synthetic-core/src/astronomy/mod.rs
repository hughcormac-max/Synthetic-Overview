//! Astronomy domain module for `AstroNodes` and celestial surface topography.
//!
//! Grounded in SSOT-PHY-001 and SSOT-PHY-004.
//! Pure Tier 0 simulation logic with zero external I/O.

pub mod models;
pub mod topography;

pub use models::{AstroNode, AstroNodeConfig, SurfaceNode};
pub use topography::{
    generate_fibonacci_nodes_by_density, generate_fibonacci_surface_nodes, GOLDEN_ANGLE_RAD,
};
