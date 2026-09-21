//! Solar system configuration loading, floating-origin translation, and surface node rendering.
//!
//! Grounded in SSOT-PHY-001, SSOT-PHY-004, and SSOT-UIX-001.

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use synthetic_core::astronomy::{
    calculate_epoch_position, generate_fibonacci_nodes_by_density, AstroNodeConfig, GlobalPosition,
    SurfaceNode,
};

use crate::globe::camera::GlobeOrbitCamera;
use crate::globe::material::{AstroDotMaterial, GlobeMaterial, OrbitMaterial};

/// Default surface node density in inverse meters: 1 node per kilometer of radius (1e-3 m^-1).
pub const KM_SURFACE_NODE_DENSITY: f64 = 1e-3;

/// Off-white color for celestial and surface dots.
pub const OFF_WHITE_COLOR: LinearRgba = LinearRgba::new(0.92, 0.92, 0.90, 1.0);

/// Translucent cyan tint for subtle orbital tracks.
pub const ORBIT_LINE_COLOR: LinearRgba = LinearRgba::new(0.25, 0.78, 0.88, 0.35);

/// Screen radius of the `AstroNode` billboard dot in pixels.
pub const ASTRO_DOT_RADIUS_PX: f32 = 6.0;

/// Number of discrete line segments used to approximate an orbital ellipse.
pub const ORBIT_SEGMENTS: usize = 128;

#[derive(Component, Debug, Clone)]
pub struct CelestialBody {
    pub config: AstroNodeConfig,
    pub global_position: GlobalPosition,
    pub index: usize,
}

#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct SurfaceNodesComponent {
    pub nodes: Vec<SurfaceNode>,
    pub mesh_handle: Handle<Mesh>,
}

#[derive(Component, Debug, Clone)]
pub struct AstroDotMarker;

#[derive(Component, Debug, Clone)]
pub struct OrbitCurveMarker;

#[derive(Resource, Debug, Clone)]
pub struct FloatingOrigin {
    pub focused_index: usize,
    pub focused_name: String,
    pub focused_position: GlobalPosition,
    pub body_names: Vec<String>,
}

#[derive(Resource, Debug, Clone)]
#[allow(dead_code)]
pub struct SolarSystemSharedAssets {
    pub astro_dot_mesh: Handle<Mesh>,
    pub astro_dot_material: Handle<AstroDotMaterial>,
    pub surface_material: Handle<GlobeMaterial>,
    pub orbit_material: Handle<OrbitMaterial>,
}

pub struct AstronomyPlugin;

impl Plugin for AstronomyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<AstroDotMaterial>::default())
            .add_systems(Startup, setup_solar_system)
            .add_systems(
                Update,
                (
                    solar_system_camera_focus_system,
                    update_floating_origin_transforms,
                    update_surface_node_visibility,
                ),
            );
    }
}

/// Resolves candidate paths for locating the solar system JSON dataset.
fn resolve_config_path(primary_path: &Path) -> Result<PathBuf, String> {
    if primary_path.exists() {
        return Ok(primary_path.to_path_buf());
    }

    let fallback_candidates = [
        PathBuf::from("assets/data/solar_system.json"),
        PathBuf::from("crates/synthetic-client/assets/data/solar_system.json"),
        PathBuf::from("../crates/synthetic-client/assets/data/solar_system.json"),
        PathBuf::from("../../assets/data/solar_system.json"),
    ];

    for candidate in &fallback_candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    Err(format!(
        "Unable to locate solar system configuration file at {} or fallback paths",
        primary_path.display()
    ))
}

/// Loads and deserializes solar system celestial configuration from JSON.
///
/// # Errors
///
/// Returns an error if the file cannot be found, read, or parsed.
pub fn load_solar_system_config(
    path: &Path,
) -> Result<Vec<AstroNodeConfig>, Box<dyn std::error::Error>> {
    let resolved_path = resolve_config_path(path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
    let content = fs::read_to_string(resolved_path)?;
    let configs: Vec<AstroNodeConfig> = serde_json::from_str(&content)?;
    Ok(configs)
}

/// Resolves all global coordinates at epoch t=0 via parent hierarchy.
#[must_use]
pub fn compute_all_epoch_positions(configs: &[AstroNodeConfig]) -> HashMap<String, GlobalPosition> {
    let mut positions: HashMap<String, GlobalPosition> = HashMap::new();

    for config in configs {
        if config.parent_name.is_none() {
            positions.insert(config.name.clone(), GlobalPosition::ZERO);
        }
    }

    let mut remaining: Vec<&AstroNodeConfig> =
        configs.iter().filter(|c| c.parent_name.is_some()).collect();

    while !remaining.is_empty() {
        let mut resolved_any = false;
        remaining.retain(|config| {
            let parent_name = config.parent_name.as_ref().unwrap();
            if let Some(&parent_pos) = positions.get(parent_name) {
                let pos = calculate_epoch_position(config, parent_pos);
                positions.insert(config.name.clone(), pos);
                resolved_any = true;
                false
            } else {
                true
            }
        });

        if !resolved_any {
            for config in remaining {
                positions.insert(config.name.clone(), GlobalPosition::ZERO);
            }
            break;
        }
    }

    positions
}

/// Builds a unit billboard quad mesh for `AstroNode` screen dots.
fn create_billboard_quad_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    let positions = vec![
        [-1.0, -1.0, 0.0],
        [1.0, -1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 1.0, 0.0],
    ];
    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}

/// Generates a closed line loop mesh approximating the 2D Keplerian orbit ellipse in the ecliptic plane.
#[must_use]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::many_single_char_names
)]
pub fn create_orbit_curve_mesh(
    semi_major_axis_m: f64,
    eccentricity: f64,
    longitude_of_periapsis_rad: f64,
) -> Mesh {
    let two_pi = std::f64::consts::TAU;
    let step_rad = two_pi / (ORBIT_SEGMENTS as f64);
    let e = eccentricity.clamp(0.0, 0.999_999);
    let a = semi_major_axis_m;
    let varpi = longitude_of_periapsis_rad;

    let mut positions = Vec::with_capacity(ORBIT_SEGMENTS + 1);
    let mut normals = Vec::with_capacity(ORBIT_SEGMENTS + 1);

    for i in 0..=ORBIT_SEGMENTS {
        let nu = (i % ORBIT_SEGMENTS) as f64 * step_rad;
        let r = a * (1.0 - e * e) / (1.0 + e * nu.cos());
        let angle = nu + varpi;
        let x = (r * angle.cos()) as f32;
        let y = (r * angle.sin()) as f32;
        let z = 0.0_f32;

        positions.push([x, y, z]);
        normals.push([0.0, 0.0, 1.0]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh
}

/// Calculates the screen-space projected radius of a spherical celestial body in pixels.
#[must_use]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn calculate_screen_radius_px(
    body_radius_m: f64,
    camera_distance_m: f32,
    fov_rad: f32,
    viewport_height_px: f32,
) -> f32 {
    let distance = camera_distance_m.max(1.0);
    let angular_radius = (body_radius_m as f32) / distance;
    let half_fov = fov_rad * 0.5;
    (angular_radius / half_fov.tan()) * (viewport_height_px * 0.5)
}

/// Bevy startup system that populates the ECS with celestial bodies, surface meshes, and orbital curves.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_lines
)]
pub fn setup_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut astro_materials: ResMut<Assets<AstroDotMaterial>>,
    mut globe_materials: ResMut<Assets<GlobeMaterial>>,
    mut orbit_materials: ResMut<Assets<OrbitMaterial>>,
) {
    let config_path = Path::new("assets/data/solar_system.json");
    let configs = match load_solar_system_config(config_path) {
        Ok(c) => c,
        Err(err) => {
            error!("Failed to load solar system configuration: {}", err);
            return;
        }
    };

    let positions = compute_all_epoch_positions(&configs);
    let mut body_names = Vec::with_capacity(configs.len());

    let astro_dot_mesh = meshes.add(create_billboard_quad_mesh());
    let astro_dot_material = astro_materials.add(AstroDotMaterial {
        color: OFF_WHITE_COLOR,
        size_px: ASTRO_DOT_RADIUS_PX,
    });
    let surface_material = globe_materials.add(GlobeMaterial {
        base_color: OFF_WHITE_COLOR,
        atmosphere_color: OFF_WHITE_COLOR,
    });
    let orbit_material = orbit_materials.add(OrbitMaterial {
        color: ORBIT_LINE_COLOR,
    });

    commands.insert_resource(SolarSystemSharedAssets {
        astro_dot_mesh: astro_dot_mesh.clone(),
        astro_dot_material: astro_dot_material.clone(),
        surface_material: surface_material.clone(),
        orbit_material: orbit_material.clone(),
    });

    // Default focus: Earth if available, else first body
    let default_focus_index = configs
        .iter()
        .position(|b| b.name == "Earth")
        .unwrap_or(0);

    let default_focus_name = configs[default_focus_index].name.clone();
    let default_focus_pos = *positions.get(&default_focus_name).unwrap_or(&GlobalPosition::ZERO);

    let mut body_entities: HashMap<String, Entity> = HashMap::new();

    for (index, config) in configs.iter().enumerate() {
        body_names.push(config.name.clone());
        let global_pos = *positions.get(&config.name).unwrap_or(&GlobalPosition::ZERO);
        let nodes = generate_fibonacci_nodes_by_density(config.radius_m, KM_SURFACE_NODE_DENSITY);

        // Pre-populate all surface nodes into the point mesh
        let mut positions_vec = Vec::with_capacity(nodes.len());
        let mut normals_vec = Vec::with_capacity(nodes.len());
        for node in &nodes {
            let pos = [node.local_x as f32, node.local_y as f32, node.local_z as f32];
            let len = (pos[0] * pos[0] + pos[1] * pos[1] + pos[2] * pos[2]).sqrt().max(1e-6);
            normals_vec.push([pos[0] / len, pos[1] / len, pos[2] / len]);
            positions_vec.push(pos);
        }

        let mut surface_mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default());
        surface_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions_vec);
        surface_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals_vec);
        let surface_mesh_handle = meshes.add(surface_mesh);

        let body_entity = commands
            .spawn((
                CelestialBody {
                    config: config.clone(),
                    global_position: global_pos,
                    index,
                },
                Transform::from_translation(Vec3::ZERO),
                Visibility::Inherited,
            ))
            .id();

        body_entities.insert(config.name.clone(), body_entity);

        // Spawn AstroNode Dot (visible initially)
        let astro_dot = commands
            .spawn((
                Mesh3d(astro_dot_mesh.clone()),
                MeshMaterial3d(astro_dot_material.clone()),
                Transform::IDENTITY,
                Visibility::Inherited,
                AstroDotMarker,
            ))
            .id();

        // Spawn Surface Nodes (hidden initially until body screen radius > dot radius)
        let surface_nodes = commands
            .spawn((
                Mesh3d(surface_mesh_handle.clone()),
                MeshMaterial3d(surface_material.clone()),
                Transform::IDENTITY,
                Visibility::Hidden,
                SurfaceNodesComponent {
                    nodes,
                    mesh_handle: surface_mesh_handle,
                },
            ))
            .id();

        commands.entity(body_entity).add_children(&[astro_dot, surface_nodes]);
    }

    // Spawn orbital curve loops as children of their respective parent celestial bodies
    for config in &configs {
        if config.semi_major_axis_m > 0.0 {
            if let Some(parent_name) = &config.parent_name {
                if let Some(&parent_entity) = body_entities.get(parent_name) {
                    let orbit_mesh = meshes.add(create_orbit_curve_mesh(
                        config.semi_major_axis_m,
                        config.eccentricity,
                        config.longitude_of_periapsis_rad,
                    ));
                    let orbit_entity = commands
                        .spawn((
                            Mesh3d(orbit_mesh),
                            MeshMaterial3d(orbit_material.clone()),
                            Transform::IDENTITY,
                            Visibility::Inherited,
                            OrbitCurveMarker,
                        ))
                        .id();
                    commands.entity(parent_entity).add_child(orbit_entity);
                }
            }
        }
    }

    commands.insert_resource(FloatingOrigin {
        focused_index: default_focus_index,
        focused_name: default_focus_name,
        focused_position: default_focus_pos,
        body_names,
    });
}

/// Floating-origin translation system: offsets all celestial bodies relative to focused body.
#[allow(clippy::needless_pass_by_value, clippy::cast_possible_truncation)]
pub fn update_floating_origin_transforms(
    origin: Res<FloatingOrigin>,
    mut query: Query<(&CelestialBody, &mut Transform)>,
) {
    let focus = origin.focused_position;
    for (body, mut transform) in &mut query {
        let rel_x = (body.global_position.x - focus.x) as f32;
        let rel_y = (body.global_position.y - focus.y) as f32;
        let rel_z = (body.global_position.z - focus.z) as f32;
        transform.translation = Vec3::new(rel_x, rel_y, rel_z);
    }
}

/// Updates the focused celestial body, synchronizing floating origin and camera distance.
///
/// Returns true if the focus successfully changed to a new body index.
#[allow(clippy::cast_possible_truncation)]
pub fn focus_on_body(
    index: usize,
    origin: &mut FloatingOrigin,
    body_query: &Query<&CelestialBody>,
    camera_query: &mut Query<&mut GlobeOrbitCamera>,
) -> bool {
    let total = origin.body_names.len();
    if index >= total || index == origin.focused_index {
        return false;
    }

    origin.focused_index = index;
    origin.focused_name = origin.body_names[index].clone();

    for body in body_query {
        if body.index == index {
            origin.focused_position = body.global_position;
            // Scale camera target distance comfortably to the new body radius
            if let Ok(mut cam) = camera_query.get_single_mut() {
                let rad = body.config.radius_m as f32;
                cam.target_distance = (rad * 200.0).clamp(cam.min_distance, cam.max_distance);
            }
            return true;
        }
    }

    true
}

/// Keyboard shortcuts to cycle or jump celestial focus (Tab, Shift-Tab, 1-9).
#[allow(clippy::needless_pass_by_value)]
pub fn solar_system_camera_focus_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut origin: ResMut<FloatingOrigin>,
    body_query: Query<&CelestialBody>,
    mut camera_query: Query<&mut GlobeOrbitCamera>,
) {
    let total = origin.body_names.len();
    if total == 0 {
        return;
    }

    let mut new_index = None;

    let shift = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
    if keyboard.just_pressed(KeyCode::Tab) {
        new_index = if shift {
            Some((origin.focused_index + total - 1) % total)
        } else {
            Some((origin.focused_index + 1) % total)
        };
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowDown) {
        new_index = Some((origin.focused_index + 1) % total);
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowUp) {
        new_index = Some((origin.focused_index + total - 1) % total);
    } else if keyboard.just_pressed(KeyCode::Digit1) && total > 0 {
        new_index = Some(0); // Sun
    } else if keyboard.just_pressed(KeyCode::Digit2) && total > 1 {
        new_index = Some(1); // Mercury
    } else if keyboard.just_pressed(KeyCode::Digit3) && total > 2 {
        new_index = Some(2); // Venus
    } else if keyboard.just_pressed(KeyCode::Digit4) && total > 3 {
        new_index = Some(3); // Earth
    } else if keyboard.just_pressed(KeyCode::Digit5) && total > 4 {
        new_index = Some(4); // Luna
    } else if keyboard.just_pressed(KeyCode::Digit6) && total > 5 {
        new_index = Some(5); // Mars
    } else if keyboard.just_pressed(KeyCode::Digit7) && total > 6 {
        new_index = Some(6); // Jupiter
    } else if keyboard.just_pressed(KeyCode::Digit8) && total > 7 {
        new_index = Some(7); // Saturn
    } else if keyboard.just_pressed(KeyCode::Digit9) && total > 8 {
        new_index = Some(8); // Uranus
    }

    if let Some(idx) = new_index {
        focus_on_body(idx, &mut origin, &body_query, &mut camera_query);
    }
}

/// Surface node threshold system: renders ALL surface nodes when screen-space radius > dot radius;
/// displays only the `AstroNode` dot when smaller than or equal.
#[allow(
    clippy::needless_pass_by_value,
    clippy::match_wildcard_for_single_variants,
    clippy::cast_precision_loss
)]
pub fn update_surface_node_visibility(
    camera_query: Query<(&Transform, &Projection), With<GlobeOrbitCamera>>,
    window_query: Query<&Window>,
    body_query: Query<(&CelestialBody, &Transform, &Children)>,
    mut surface_query: Query<&mut Visibility, (With<SurfaceNodesComponent>, Without<AstroDotMarker>)>,
    mut dot_query: Query<&mut Visibility, (With<AstroDotMarker>, Without<SurfaceNodesComponent>)>,
) {
    let Ok((cam_transform, projection)) = camera_query.get_single() else {
        return;
    };

    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        _ => 3.0 * std::f32::consts::PI / 180.0,
    };

    let viewport_height = window_query
        .get_single()
        .map_or(1080.0, |win| win.physical_height().max(1) as f32);

    let cam_pos = cam_transform.translation;

    for (body, body_transform, children) in &body_query {
        let distance = cam_pos.distance(body_transform.translation).max(1.0);
        let radius_screen_px = calculate_screen_radius_px(
            body.config.radius_m,
            distance,
            fov,
            viewport_height,
        );

        let show_surface_nodes = radius_screen_px > ASTRO_DOT_RADIUS_PX;

        for &child in children {
            if let Ok(mut vis) = surface_query.get_mut(child) {
                let target_vis = if show_surface_nodes {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                };
                if *vis != target_vis {
                    *vis = target_vis;
                }
            } else if let Ok(mut vis) = dot_query.get_mut(child) {
                let target_vis = if show_surface_nodes {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
                if *vis != target_vis {
                    *vis = target_vis;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_all_epoch_positions() {
        let configs = vec![
            AstroNodeConfig {
                name: "Sun".into(),
                parent_name: None,
                mass_kg: 1.989e30,
                radius_m: 6.9634e8,
                semi_major_axis_m: 0.0,
                eccentricity: 0.0,
                true_anomaly_epoch_rad: 0.0,
                longitude_of_periapsis_rad: 0.0,
                mean_motion_rad_s: 0.0,
            },
            AstroNodeConfig {
                name: "Earth".into(),
                parent_name: Some("Sun".into()),
                mass_kg: 5.972e24,
                radius_m: 6.371e6,
                semi_major_axis_m: 1.495_978_7e11,
                eccentricity: 0.0,
                true_anomaly_epoch_rad: 0.0,
                longitude_of_periapsis_rad: 0.0,
                mean_motion_rad_s: 1.991e-7,
            },
            AstroNodeConfig {
                name: "Luna".into(),
                parent_name: Some("Earth".into()),
                mass_kg: 7.342e22,
                radius_m: 1.7374e6,
                semi_major_axis_m: 3.844e8,
                eccentricity: 0.0,
                true_anomaly_epoch_rad: 0.0,
                longitude_of_periapsis_rad: 0.0,
                mean_motion_rad_s: 2.662e-6,
            },
        ];

        let positions = compute_all_epoch_positions(&configs);
        assert_eq!(positions.len(), 3);

        let sun = positions.get("Sun").unwrap();
        assert_eq!(*sun, GlobalPosition::ZERO);

        let earth = positions.get("Earth").unwrap();
        assert!((earth.x - 1.495_978_7e11).abs() < 1.0);
        assert!(earth.y.abs() < 1.0);

        let luna = positions.get("Luna").unwrap();
        assert!((luna.x - (1.495_978_7e11 + 3.844e8)).abs() < 1.0);
        assert!(luna.y.abs() < 1.0);
    }

    #[test]
    fn test_load_solar_system_config() {
        let test_path = Path::new("assets/data/solar_system.json");
        let result = load_solar_system_config(test_path);
        assert!(
            result.is_ok(),
            "Expected successful config load, got error: {:?}",
            result.err()
        );

        let configs = result.unwrap();
        assert!(!configs.is_empty(), "Solar system config must not be empty");

        let find_body = |name: &str| configs.iter().find(|b| b.name == name);

        let sun = find_body("Sun").expect("Sun must be present");
        assert!(sun.parent_name.is_none(), "Sun must have no parent");
        assert!(sun.mass_kg > 1.9e30);

        let earth = find_body("Earth").expect("Earth must be present");
        assert_eq!(earth.parent_name.as_deref(), Some("Sun"));

        let luna = find_body("Luna").expect("Luna must be present");
        assert_eq!(luna.parent_name.as_deref(), Some("Earth"));
    }

    #[test]
    fn test_km_surface_node_generation() {
        let earth_radius = 6_371_000.0;
        let earth_nodes =
            generate_fibonacci_nodes_by_density(earth_radius, KM_SURFACE_NODE_DENSITY);
        assert_eq!(earth_nodes.len(), 6_371);

        let jupiter_radius = 69_911_000.0;
        let jupiter_nodes =
            generate_fibonacci_nodes_by_density(jupiter_radius, KM_SURFACE_NODE_DENSITY);
        assert_eq!(jupiter_nodes.len(), 69_911);
    }

    #[test]
    #[allow(clippy::cast_possible_truncation)]
    fn test_create_orbit_curve_mesh() {
        let a = 1.0e11;
        let e = 0.1;
        let varpi = 0.0;
        let mesh = create_orbit_curve_mesh(a, e, varpi);
        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::LineStrip);
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Positions attribute must exist");
        if let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) = positions {
            assert_eq!(pts.len(), ORBIT_SEGMENTS + 1);
            // Periapsis at nu = 0, angle = 0: x = a * (1 - e), y = 0
            let periapsis_x = pts[0][0];
            let expected_peri = (a * (1.0 - e)) as f32;
            assert!((periapsis_x - expected_peri).abs() < 1e3);
            assert!(pts[0][1].abs() < 1e3);

            // Apoapsis at nu = PI (sample index 64): x = -a * (1 + e), y = 0
            let apoapsis_x = pts[64][0];
            let expected_apo = (-a * (1.0 + e)) as f32;
            assert!((apoapsis_x - expected_apo).abs() < 1e3);
            assert!(pts[64][1].abs() < 1e3);

            // Loop closure: index 0 equals index 128
            assert!((pts[0][0] - pts[ORBIT_SEGMENTS][0]).abs() < 1e-4);
            assert!((pts[0][1] - pts[ORBIT_SEGMENTS][1]).abs() < 1e-4);
            assert!((pts[0][2] - pts[ORBIT_SEGMENTS][2]).abs() < 1e-4);
        } else {
            panic!("Expected Float32x3 positions format");
        }
    }

    #[test]
    fn test_calculate_screen_radius_px() {
        let fov = 3.0 * std::f32::consts::PI / 180.0;
        let win_h = 1080.0;

        // Earth close observation (1.2e9 m) -> larger than dot radius (6.0 px)
        let r_close = calculate_screen_radius_px(6.371e6, 1.2e9, fov, win_h);
        assert!(r_close > ASTRO_DOT_RADIUS_PX);

        // Distant Earth from Sun focus (1.5e11 m) -> smaller than dot radius (6.0 px)
        let r_distant = calculate_screen_radius_px(6.371e6, 1.5e11, fov, win_h);
        assert!(r_distant < ASTRO_DOT_RADIUS_PX);
    }

    #[test]
    fn test_focus_on_body_boundary() {
        #[allow(clippy::needless_pass_by_value)]
        fn test_system(
            mut origin: ResMut<FloatingOrigin>,
            body_query: Query<&CelestialBody>,
            mut camera_query: Query<&mut GlobeOrbitCamera>,
        ) {
            assert!(!focus_on_body(0, &mut origin, &body_query, &mut camera_query));
            assert!(!focus_on_body(5, &mut origin, &body_query, &mut camera_query));
        }

        let mut app = App::new();
        app.insert_resource(FloatingOrigin {
            focused_index: 0,
            focused_name: "Sun".to_string(),
            focused_position: GlobalPosition::ZERO,
            body_names: vec!["Sun".to_string(), "Earth".to_string()],
        });

        app.add_systems(Update, test_system);
        app.update();
    }
}
