//! Tier 0 Spatial Domain: H3 hexagonal spatial indexing for astronomical bodies.
//!
//! Architectural Invariants:
//! 1. Zero external I/O or UI dependencies.
//! 2. Deterministic, pure domain representations.
//! 3. Plain-text and ASCII notation only (strict Zero-LaTeX compliance).

pub mod components;

pub use components::{AstroNode, SurfaceNode};
