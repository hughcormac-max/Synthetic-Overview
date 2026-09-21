//! Spherical Fibonacci surface node generation for `AstroNodes`.
//!
//! Grounded in SSOT-PHY-004 Section 2.3.
//! Distributes surface nodes deterministically across a sphere of radius `radius_m`
//! using a closed-form spherical Fibonacci lattice (O(N) time complexity).
//! Uses strict Zero-LaTeX plain text notation.

use super::models::SurfaceNode;

/// Golden angle step in radians: `pi * (3.0 - sqrt(5.0))`.
pub const GOLDEN_ANGLE_RAD: f64 = 2.399_963_229_728_653;

/// Generates Spherical Fibonacci surface nodes for an `AstroNode` based on physical radius and count.
///
/// Grounded in SSOT-PHY-004 Section 2.3.
/// If `radius_m <= 0.0` or `node_count == 0`, returns an empty vector.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn generate_fibonacci_surface_nodes(radius_m: f64, node_count: usize) -> Vec<SurfaceNode> {
    if radius_m <= 0.0 || node_count == 0 {
        return Vec::new();
    }

    let mut nodes = Vec::with_capacity(node_count);
    let n = node_count as f64;
    let two_pi = 2.0 * std::f64::consts::PI;

    for i in 0..node_count {
        let i_f64 = i as f64;
        let z_frac = 1.0 - (2.0 * i_f64 + 1.0) / n;
        let phi = (i_f64 * GOLDEN_ANGLE_RAD).rem_euclid(two_pi);
        let r_xy_frac = (1.0 - z_frac * z_frac).max(0.0).sqrt();

        let local_x = radius_m * r_xy_frac * phi.cos();
        let local_y = radius_m * r_xy_frac * phi.sin();
        let local_z = radius_m * z_frac;

        nodes.push(SurfaceNode {
            local_x,
            local_y,
            local_z,
        });
    }

    nodes
}

/// Generates Spherical Fibonacci surface nodes for an `AstroNode` based on physical radius and density.
///
/// Grounded in SSOT-PHY-004 Section 2.3.
/// If `radius_m <= 0.0` or `k_density <= 0.0`, returns an empty vector.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn generate_fibonacci_nodes_by_density(radius_m: f64, k_density: f64) -> Vec<SurfaceNode> {
    if radius_m <= 0.0 || k_density <= 0.0 {
        return Vec::new();
    }
    let count = (radius_m * k_density).floor() as usize;
    generate_fibonacci_surface_nodes(radius_m, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_top_01_north_pole_apex() {
        let nodes = generate_fibonacci_surface_nodes(6_371_000.0, 10_000);
        assert_eq!(nodes.len(), 10_000);
        let first_node = &nodes[0];
        assert!(
            (first_node.local_z - 6_370_362.9).abs() <= 1.0,
            "Expected local_z near 6370362.9, got {}",
            first_node.local_z
        );
        let r = (first_node.local_x * first_node.local_x
            + first_node.local_y * first_node.local_y
            + first_node.local_z * first_node.local_z)
            .sqrt();
        assert!(
            (r - 6_371_000.0).abs() <= 1.0,
            "Radius violation at apex: {r}"
        );
    }

    #[test]
    fn test_vec_top_02_south_pole_apex() {
        let nodes = generate_fibonacci_surface_nodes(6_371_000.0, 10_000);
        let node_last = &nodes[9_999];
        assert!(
            (node_last.local_z - (-6_370_362.9)).abs() <= 1.0,
            "Expected local_z near -6370362.9, got {}",
            node_last.local_z
        );
        let r = (node_last.local_x * node_last.local_x
            + node_last.local_y * node_last.local_y
            + node_last.local_z * node_last.local_z)
            .sqrt();
        assert!(
            (r - 6_371_000.0).abs() <= 1.0,
            "Radius violation at south apex: {r}"
        );
    }

    #[test]
    fn test_vec_top_03_equator_radius_consistency() {
        let nodes = generate_fibonacci_surface_nodes(6_371_000.0, 10_000);
        let node_eq = &nodes[5_000];
        let r = (node_eq.local_x * node_eq.local_x
            + node_eq.local_y * node_eq.local_y
            + node_eq.local_z * node_eq.local_z)
            .sqrt();
        assert!(
            (r - 6_371_000.0).abs() <= 1.0,
            "Radius violation near equator: {r}"
        );
    }

    #[test]
    fn test_vec_top_04_golden_angle() {
        let phi_1 = (1.0 * GOLDEN_ANGLE_RAD).rem_euclid(2.0 * std::f64::consts::PI);
        assert!(
            (phi_1 - 2.399_963).abs() < 0.0001,
            "Expected phi_1 approx 2.399963 rad, got {phi_1}"
        );
    }

    #[test]
    fn test_edge_cases() {
        assert!(generate_fibonacci_surface_nodes(0.0, 100).is_empty());
        assert!(generate_fibonacci_surface_nodes(-1_000.0, 100).is_empty());
        assert!(generate_fibonacci_surface_nodes(1_000_000.0, 0).is_empty());
        assert!(generate_fibonacci_nodes_by_density(0.0, 1e-6).is_empty());
        assert!(generate_fibonacci_nodes_by_density(-1_000.0, 1e-6).is_empty());
        assert!(generate_fibonacci_nodes_by_density(1_000_000.0, 0.0).is_empty());
        assert!(generate_fibonacci_nodes_by_density(1_000_000.0, -1e-6).is_empty());
    }

    #[test]
    fn test_perf_100k_nodes() {
        let nodes = generate_fibonacci_surface_nodes(6_371_000.0, 100_000);
        assert_eq!(nodes.len(), 100_000);
    }
}
