use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;

#[allow(dead_code)]
const GOLDEN_RATIO: f32 = 1.618_034;
#[allow(dead_code)]
const TWO_PI: f32 = std::f32::consts::TAU;
#[allow(dead_code)]
const GOLDEN_ANGLE: f32 = TWO_PI * (1.0 - 1.0 / GOLDEN_RATIO);

/// Generates a spherical point-list mesh using the SSOT-PHY-004 Fibonacci spiral
#[allow(dead_code, clippy::cast_precision_loss)]
pub fn generate_fibonacci_globe_mesh(node_count: usize, radius: f32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(node_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(node_count);

    let count_f = node_count.max(2) as f32;
    for i in 0..node_count {
        let y = 1.0 - (i as f32 / (count_f - 1.0)) * 2.0;
        let radius_at_y = (1.0 - y * y).max(0.0).sqrt();
        let theta = GOLDEN_ANGLE * (i as f32);

        let x = theta.cos() * radius_at_y;
        let z = theta.sin() * radius_at_y;

        let normal = [x, y, z];
        let pos = [x * radius, y * radius, z * radius];

        positions.push(pos);
        normals.push(normal);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh
}

