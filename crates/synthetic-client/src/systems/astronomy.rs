//! Solar system configuration loading, floating-origin translation, and surface node rendering.
//!
//! Grounded in SSOT-PHY-001, SSOT-PHY-004, and SSOT-UIX-001.

use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::primitives::Aabb;
use bevy::render::render_asset::RenderAssetUsages;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use synthetic_core::astronomy::{
    calculate_kepler_position, generate_fibonacci_nodes_by_density,
    AstroNodeConfig, GlobalPosition, SurfaceNode,
};

use crate::globe::camera::GlobeOrbitCamera;
use crate::globe::material::{AstroDotMaterial, GlobeMaterial, OrbitMaterial};
use crate::systems::time_warp::TimeWarp;

/// Default surface node density in inverse meters: 1 node per kilometer of radius (1e-3 m^-1).
pub const KM_SURFACE_NODE_DENSITY: f64 = 1e-3;

/// Off-white color for celestial and surface dots.
pub const OFF_WHITE_COLOR: LinearRgba = LinearRgba::new(0.92, 0.92, 0.90, 1.0);

/// Translucent cyan tint for subtle orbital tracks.
pub const ORBIT_LINE_COLOR: LinearRgba = LinearRgba::new(0.25, 0.78, 0.88, 0.35);

/// Screen radius of the `AstroNode` billboard dot in pixels.
pub const ASTRO_DOT_RADIUS_PX: f32 = 6.0;

/// Minimum screen-space pick radius in pixels for celestial body selection.
pub const MIN_PICK_RADIUS_PX: f32 = 10.0;

/// Maximum cursor displacement in pixels allowed between mouse press and release to register as a click.
pub const MAX_CLICK_DRAG_DISTANCE_PX: f32 = 5.0;

/// Number of hours into the past and future to project the orbital path.
pub const ORBIT_DOT_WINDOW_HOURS: i32 = 200;

/// Number of total points in the dot orbital path (past + current + future).
pub const ORBIT_DOT_COUNT: usize = (ORBIT_DOT_WINDOW_HOURS * 2 + 1) as usize;

/// Number of seconds in a single orbital dot time step.
pub const ORBIT_DOT_INTERVAL_S: f64 = 3600.0;

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

/// Attached to the child orbit entity rendering the dynamic time-step points.
#[derive(Component, Debug, Clone)]
#[allow(dead_code)]
pub struct DynamicOrbitDots {
    pub mesh_handle: Handle<Mesh>,
    pub body_entity: Entity,
}

#[allow(dead_code)]
impl DynamicOrbitDots {
    #[must_use]
    pub const fn new(mesh_handle: Handle<Mesh>, body_entity: Entity) -> Self {
        Self {
            mesh_handle,
            body_entity,
        }
    }
}

/// Current simulation time tracking in seconds elapsed since epoch.
#[derive(Resource, Debug, Clone, Copy, Default, PartialEq)]
#[allow(dead_code)]
pub struct SimulationTime {
    pub elapsed_seconds: f64,
}

#[allow(dead_code)]
impl SimulationTime {
    #[must_use]
    pub const fn new(elapsed_seconds: f64) -> Self {
        Self { elapsed_seconds }
    }
}

/// System set for systems that advance or update simulation time.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SimulationTimeSystem;

/// Advances simulation time based on real frame delta time and current warp multiplier.
#[allow(clippy::needless_pass_by_value)]
pub fn advance_simulation_time(
    time: Res<Time>,
    time_warp: Res<TimeWarp>,
    mut sim_time: ResMut<SimulationTime>,
) {
    sim_time.elapsed_seconds += time.delta_secs_f64() * time_warp.current_multiplier();
}

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
            .init_resource::<SimulationTime>()
            .add_systems(Startup, setup_solar_system)
            .add_systems(
                Update,
                (
                    advance_simulation_time.in_set(SimulationTimeSystem),
                    update_celestial_positions_system.after(SimulationTimeSystem),
                    update_orbital_dots_system.after(SimulationTimeSystem),
                    solar_system_camera_focus_system,
                    mouse_pick_astronode_system,
                    update_floating_origin_transforms.after(update_celestial_positions_system),
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

/// Resolves all global coordinates at a specific simulation time `t_s` via parent hierarchy.
#[must_use]
pub fn compute_all_positions_at_time(
    configs: &[AstroNodeConfig],
    t_s: f64,
) -> HashMap<String, GlobalPosition> {
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
                let pos = calculate_kepler_position(config, parent_pos, t_s);
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

/// Resolves all global coordinates at epoch t=0 via parent hierarchy.
#[must_use]
pub fn compute_all_epoch_positions(configs: &[AstroNodeConfig]) -> HashMap<String, GlobalPosition> {
    compute_all_positions_at_time(configs, 0.0)
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

    // Spawn dynamic orbital dots as children of their respective parent celestial bodies
    for config in &configs {
        if config.semi_major_axis_m > 0.0 {
            if let Some(parent_name) = &config.parent_name {
                if let Some(&parent_entity) = body_entities.get(parent_name) {
                    if let Some(&body_entity) = body_entities.get(&config.name) {
                        let mut mesh = Mesh::new(
                            PrimitiveTopology::PointList,
                            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
                        );
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_POSITION,
                            vec![[0.0_f32, 0.0_f32, 0.0_f32]; ORBIT_DOT_COUNT],
                        );
                        mesh.insert_attribute(
                            Mesh::ATTRIBUTE_NORMAL,
                            vec![[0.0_f32, 0.0_f32, 1.0_f32]; ORBIT_DOT_COUNT],
                        );
                        let mesh_handle = meshes.add(mesh);
                        let orbit_entity = commands
                            .spawn((
                                Mesh3d(mesh_handle.clone()),
                                MeshMaterial3d(orbit_material.clone()),
                                Transform::IDENTITY,
                                Visibility::Inherited,
                                Aabb::default(),
                                DynamicOrbitDots {
                                    mesh_handle,
                                    body_entity,
                                },
                            ))
                            .id();
                        commands.entity(parent_entity).add_child(orbit_entity);
                    }
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

/// Dynamically updates the orbital point mesh for each celestial body based on current simulation time.
///
/// Grounded in SSOT-PHY-001 and PLAN-019.
/// Evaluates `calculate_kepler_position` across [`T_sim` - 200h, `T_sim` + 200h] in parent local space.
#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless
)]
pub fn update_orbital_dots_system(
    mut commands: Commands,
    sim_time: Res<SimulationTime>,
    mut orbit_query: Query<(Entity, &DynamicOrbitDots, Option<&mut Aabb>)>,
    body_query: Query<&CelestialBody>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let t_sim = sim_time.elapsed_seconds;

    for (orbit_entity, orbit_dots, maybe_aabb) in &mut orbit_query {
        let Ok(body) = body_query.get(orbit_dots.body_entity) else {
            continue;
        };

        let Some(mesh) = meshes.get_mut(&orbit_dots.mesh_handle) else {
            continue;
        };

        let Some(bevy::render::mesh::VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        else {
            continue;
        };

        if positions.len() != ORBIT_DOT_COUNT {
            positions.resize(ORBIT_DOT_COUNT, [0.0, 0.0, 0.0]);
        }

        let mut min_pt = Vec3::splat(f32::INFINITY);
        let mut max_pt = Vec3::splat(f32::NEG_INFINITY);

        for (i, pos_out) in positions.iter_mut().enumerate() {
            let offset_hours = (i as i32) - ORBIT_DOT_WINDOW_HOURS;
            let t_point = t_sim + f64::from(offset_hours) * ORBIT_DOT_INTERVAL_S;
            let kepler_pos = calculate_kepler_position(&body.config, GlobalPosition::ZERO, t_point);
            let pt = Vec3::new(kepler_pos.x as f32, kepler_pos.y as f32, kepler_pos.z as f32);
            *pos_out = [pt.x, pt.y, pt.z];
            min_pt = min_pt.min(pt);
            max_pt = max_pt.max(pt);
        }

        if min_pt.x.is_finite() && max_pt.x.is_finite() {
            let center = (min_pt + max_pt) * 0.5;
            let half_extents = ((max_pt - min_pt) * 0.5).max(Vec3::splat(1.0));
            let new_aabb = Aabb {
                center: Vec3A::from(center),
                half_extents: Vec3A::from(half_extents),
            };

            if let Some(mut aabb) = maybe_aabb {
                *aabb = new_aabb;
            } else {
                commands.entity(orbit_entity).insert(new_aabb);
            }
        }
    }
}

/// Propagates all celestial body global positions along their Keplerian orbits at current simulation time.
#[allow(clippy::needless_pass_by_value)]
pub fn update_celestial_positions_system(
    sim_time: Res<SimulationTime>,
    mut origin: ResMut<FloatingOrigin>,
    mut body_query: Query<&mut CelestialBody>,
) {
    let configs: Vec<AstroNodeConfig> = body_query.iter().map(|b| b.config.clone()).collect();
    let positions = compute_all_positions_at_time(&configs, sim_time.elapsed_seconds);

    for mut body in &mut body_query {
        if let Some(&new_pos) = positions.get(&body.config.name) {
            body.global_position = new_pos;
            if body.index == origin.focused_index {
                origin.focused_position = new_pos;
            }
        }
    }
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
pub fn focus_on_body<'a, I>(
    index: usize,
    origin: &mut FloatingOrigin,
    bodies: I,
    camera_query: &mut Query<&mut GlobeOrbitCamera>,
) -> bool
where
    I: IntoIterator<Item = &'a CelestialBody>,
{
    let total = origin.body_names.len();
    if index >= total || index == origin.focused_index {
        return false;
    }

    origin.focused_index = index;
    origin.focused_name = origin.body_names[index].clone();

    for body in bodies {
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

/// Screen-to-world raycasting and picking system for celestial bodies.
///
/// Clicking directly on a celestial body focuses the camera and floating origin on it.
#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation
)]
pub fn mouse_pick_astronode_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<GlobeOrbitCamera>>,
    body_query: Query<(&CelestialBody, &GlobalTransform)>,
    mut origin: ResMut<FloatingOrigin>,
    mut globe_camera: Query<&mut GlobeOrbitCamera>,
    mut click_start_pos: Local<Option<Vec2>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Ok(window) = window_query.get_single() {
            *click_start_pos = window.cursor_position();
        }
    }

    if !mouse_button_input.just_released(MouseButton::Left) {
        if mouse_button_input.pressed(MouseButton::Left) {
            if let (Some(start_pos), Ok(window)) = (*click_start_pos, window_query.get_single()) {
                if let Some(cursor_pos) = window.cursor_position() {
                    if start_pos.distance_squared(cursor_pos)
                        > MAX_CLICK_DRAG_DISTANCE_PX * MAX_CLICK_DRAG_DISTANCE_PX
                    {
                        *click_start_pos = None;
                    }
                }
            }
        }
        return;
    }

    let Some(start_pos) = click_start_pos.take() else {
        return;
    };

    let Ok(window) = window_query.get_single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    if start_pos.distance_squared(cursor_pos)
        > MAX_CLICK_DRAG_DISTANCE_PX * MAX_CLICK_DRAG_DISTANCE_PX
    {
        return;
    }

    let Ok((camera, camera_transform, projection)) = camera_query.get_single() else {
        return;
    };

    let fov = match projection {
        Projection::Perspective(p) => p.fov,
        Projection::Orthographic(_) => 3.0 * std::f32::consts::PI / 180.0,
    };

    let viewport_height = window.height().max(1.0);
    let cam_pos = camera_transform.translation();

    let mut closest_body_index = None;
    let mut closest_distance_sq = f32::INFINITY;

    for (body, body_transform) in &body_query {
        let body_pos = body_transform.translation();
        let Some(screen_pos) = camera.world_to_viewport(camera_transform, body_pos).ok() else {
            continue;
        };

        let cam_to_body = body_pos - cam_pos;
        let distance_3d_sq = cam_to_body.length_squared();
        let distance_3d = distance_3d_sq.sqrt().max(1.0);

        let projected_radius = calculate_screen_radius_px(
            body.config.radius_m,
            distance_3d,
            fov,
            viewport_height,
        );
        let pick_radius = projected_radius.max(MIN_PICK_RADIUS_PX);
        let pick_radius_sq = pick_radius * pick_radius;

        let screen_dist_sq = cursor_pos.distance_squared(screen_pos);
        if screen_dist_sq <= pick_radius_sq && distance_3d_sq < closest_distance_sq {
            closest_distance_sq = distance_3d_sq;
            closest_body_index = Some(body.index);
        }
    }

    if let Some(target_index) = closest_body_index {
        focus_on_body(
            target_index,
            &mut origin,
            body_query.iter().map(|(b, _)| b),
            &mut globe_camera,
        );
    }
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
        Projection::Orthographic(_) => 3.0 * std::f32::consts::PI / 180.0,
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
                inclination_rad: 0.0,
                longitude_of_ascending_node_rad: 0.0,
                argument_of_periapsis_rad: 0.0,
                true_anomaly_epoch_rad: 0.0,
                mean_motion_rad_s: 0.0,
            },
            AstroNodeConfig {
                name: "Earth".into(),
                parent_name: Some("Sun".into()),
                mass_kg: 5.972e24,
                radius_m: 6.371e6,
                semi_major_axis_m: 1.495_978_7e11,
                eccentricity: 0.0,
                inclination_rad: 0.0,
                longitude_of_ascending_node_rad: 0.0,
                argument_of_periapsis_rad: 0.0,
                true_anomaly_epoch_rad: 0.0,
                mean_motion_rad_s: 1.991e-7,
            },
            AstroNodeConfig {
                name: "Luna".into(),
                parent_name: Some("Earth".into()),
                mass_kg: 7.342e22,
                radius_m: 1.7374e6,
                semi_major_axis_m: 3.844e8,
                eccentricity: 0.0,
                inclination_rad: 0.0,
                longitude_of_ascending_node_rad: 0.0,
                argument_of_periapsis_rad: 0.0,
                true_anomaly_epoch_rad: 0.0,
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
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::float_cmp,
        clippy::cast_lossless
    )]
    fn test_orbital_local_space_invariance() {
        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.insert_resource(SimulationTime::new(10_000.0));

        let parent_entity = app
            .world_mut()
            .spawn((
                Transform::from_xyz(1.0e11, 2.0e11, 3.0e11),
                GlobalTransform::from(Transform::from_xyz(1.0e11, 2.0e11, 3.0e11)),
            ))
            .id();

        let earth_config = AstroNodeConfig {
            name: "Earth".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 5.972e24,
            radius_m: 6.371e6,
            semi_major_axis_m: 1.495_978_7e11,
            eccentricity: 0.0167,
            inclination_rad: 0.0,
            longitude_of_ascending_node_rad: 0.0,
            argument_of_periapsis_rad: 0.0,
            true_anomaly_epoch_rad: 0.0,
            mean_motion_rad_s: 1.991e-7,
        };

        let body_entity = app
            .world_mut()
            .spawn(CelestialBody {
                config: earth_config,
                global_position: GlobalPosition::new(1.495_978_7e11, 0.0, 0.0),
                index: 1,
            })
            .id();

        let mut initial_mesh = Mesh::new(
            PrimitiveTopology::PointList,
            RenderAssetUsages::default(),
        );
        initial_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0_f32, 0.0_f32, 0.0_f32]; ORBIT_DOT_COUNT],
        );
        let mesh_handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(initial_mesh);

        let orbit_entity = app
            .world_mut()
            .spawn(DynamicOrbitDots {
                mesh_handle: mesh_handle.clone(),
                body_entity,
            })
            .id();
        app.world_mut().entity_mut(parent_entity).add_child(orbit_entity);

        app.add_systems(Update, update_orbital_dots_system);
        app.update();

        let first_positions: Vec<[f32; 3]> = {
            let meshes = app.world().resource::<Assets<Mesh>>();
            let mesh = meshes.get(&mesh_handle).unwrap();
            let pos_attr = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) = pos_attr else {
                panic!("Expected Float32x3 positions");
            };
            pts.clone()
        };

        // Mutate parent transform significantly to simulate camera translation or parent movement
        {
            let mut parent_transform = app
                .world_mut()
                .get_mut::<Transform>(parent_entity)
                .unwrap();
            parent_transform.translation = Vec3::new(-9.9e11, 4.4e11, -1.2e11);
        }
        app.update();

        let second_positions: Vec<[f32; 3]> = {
            let meshes = app.world().resource::<Assets<Mesh>>();
            let mesh = meshes.get(&mesh_handle).unwrap();
            let pos_attr = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) = pos_attr else {
                panic!("Expected Float32x3 positions");
            };
            pts.clone()
        };

        assert_eq!(first_positions.len(), ORBIT_DOT_COUNT);
        for (p1, p2) in first_positions.iter().zip(&second_positions) {
            assert_eq!(
                p1, p2,
                "Orbital points must be strictly invariant to parent transform in local space"
            );
            let radius = (p1[0] * p1[0] + p1[1] * p1[1] + p1[2] * p1[2]).sqrt();
            assert!(
                f64::from(radius) < 1.495_978_7e11 * 1.05 && f64::from(radius) > 1.495_978_7e11 * 0.95,
                "Point must be centered around parent local origin"
            );
        }
    }

    #[test]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_lossless
    )]
    fn test_orbital_time_window_bounds() {
        let t_sim = 72_000.0_f64;
        let config = AstroNodeConfig {
            name: "TestBody".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 1.0,
            radius_m: 1000.0,
            semi_major_axis_m: 1.0e10,
            eccentricity: 0.1,
            inclination_rad: 0.05,
            longitude_of_ascending_node_rad: 0.1,
            argument_of_periapsis_rad: 0.2,
            true_anomaly_epoch_rad: 0.3,
            mean_motion_rad_s: 1.0e-5,
        };

        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.insert_resource(SimulationTime::new(t_sim));

        let body_entity = app
            .world_mut()
            .spawn(CelestialBody {
                config: config.clone(),
                global_position: GlobalPosition::ZERO,
                index: 0,
            })
            .id();

        let parent_entity = app.world_mut().spawn(Transform::IDENTITY).id();

        let mut mesh = Mesh::new(
            PrimitiveTopology::PointList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0.0_f32, 0.0_f32, 0.0_f32]; ORBIT_DOT_COUNT],
        );
        let mesh_handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);

        let orbit_entity = app
            .world_mut()
            .spawn(DynamicOrbitDots {
                mesh_handle: mesh_handle.clone(),
                body_entity,
            })
            .id();
        app.world_mut().entity_mut(parent_entity).add_child(orbit_entity);

        app.add_systems(Update, update_orbital_dots_system);
        app.update();

        let meshes = app.world().resource::<Assets<Mesh>>();
        let mesh = meshes.get(&mesh_handle).unwrap();
        let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
        else {
            panic!("Expected Float32x3 positions");
        };

        assert_eq!(pts.len(), ORBIT_DOT_COUNT);

        // Verify index 0: T_points[0] strictly equals T_sim - ORBIT_DOT_WINDOW_HOURS * 3600
        let expected_t_start = t_sim - f64::from(ORBIT_DOT_WINDOW_HOURS) * 3600.0;
        let expected_pos_start = calculate_kepler_position(&config, GlobalPosition::ZERO, expected_t_start);
        assert!((pts[0][0] - expected_pos_start.x as f32).abs() < 1e-1);
        assert!((pts[0][1] - expected_pos_start.y as f32).abs() < 1e-1);
        assert!((pts[0][2] - expected_pos_start.z as f32).abs() < 1e-1);

        // Verify index 200 (current time T_sim)
        let mid_idx = ORBIT_DOT_WINDOW_HOURS as usize;
        let expected_pos_mid = calculate_kepler_position(&config, GlobalPosition::ZERO, t_sim);
        assert!((pts[mid_idx][0] - expected_pos_mid.x as f32).abs() < 1e-1);
        assert!((pts[mid_idx][1] - expected_pos_mid.y as f32).abs() < 1e-1);
        assert!((pts[mid_idx][2] - expected_pos_mid.z as f32).abs() < 1e-1);

        // Verify index 400: T_points[400] strictly equals T_sim + ORBIT_DOT_WINDOW_HOURS * 3600
        let end_idx = ORBIT_DOT_COUNT - 1;
        let expected_t_end = t_sim + f64::from(ORBIT_DOT_WINDOW_HOURS) * 3600.0;
        let expected_pos_end = calculate_kepler_position(&config, GlobalPosition::ZERO, expected_t_end);
        assert!((pts[end_idx][0] - expected_pos_end.x as f32).abs() < 1e-1);
        assert!((pts[end_idx][1] - expected_pos_end.y as f32).abs() < 1e-1);
        assert!((pts[end_idx][2] - expected_pos_end.z as f32).abs() < 1e-1);
    }

    #[test]
    fn test_dynamic_orbital_dots_aabb_update() {
        let mut app = App::new();
        app.add_plugins(bevy::asset::AssetPlugin::default());
        app.init_asset::<Mesh>();
        app.insert_resource(SimulationTime::new(0.0));

        let mut config = create_test_body_config("Earth");
        config.parent_name = Some("Sun".into());
        config.semi_major_axis_m = 1.496e11;
        config.mean_motion_rad_s = 1.991e-7;

        let body_entity = app
            .world_mut()
            .spawn(CelestialBody {
                config,
                global_position: GlobalPosition::ZERO,
                index: 0,
            })
            .id();
        let parent = app.world_mut().spawn(Transform::IDENTITY).id();

        let mut mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0_f32; 3]; ORBIT_DOT_COUNT]);
        let mesh_handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);

        let orbit_entity = app
            .world_mut()
            .spawn((
                DynamicOrbitDots {
                    mesh_handle,
                    body_entity,
                },
                Aabb::default(),
            ))
            .id();
        app.world_mut().entity_mut(parent).add_child(orbit_entity);

        app.add_systems(Update, update_orbital_dots_system);
        app.update();

        let aabb_t0 = *app.world().get::<Aabb>(orbit_entity).unwrap();
        assert!(aabb_t0.center.x > 1.4e11, "AABB at t=0 must be near +1.5e11");

        // Advance 6 months (half orbit ~ 1.578e7 s)
        app.world_mut().resource_mut::<SimulationTime>().elapsed_seconds = 1.578e7;
        app.update();

        let aabb_t6m = *app.world().get::<Aabb>(orbit_entity).unwrap();
        assert!(aabb_t6m.center.x < -1.4e11, "AABB at t=6m must be near -1.5e11");
        assert!((aabb_t6m.center - aabb_t0.center).length() > 2.8e11);
    }

    #[test]
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_lossless
    )]
    fn test_eccentricity_edge_cases() {
        let test_eccentricities = [0.0, 0.001, 0.5, 0.9, 0.99, 0.999_999];
        let test_sim_times = [-1_000_000.0, 0.0, 100.0, 500_000.0, 10_000_000.0];

        for &e in &test_eccentricities {
            let config = AstroNodeConfig {
                name: format!("Body_e_{e}"),
                parent_name: Some("Sun".into()),
                mass_kg: 1.0e24,
                radius_m: 5.0e6,
                semi_major_axis_m: 2.0e11,
                eccentricity: e,
                inclination_rad: 0.1,
                longitude_of_ascending_node_rad: 0.2,
                argument_of_periapsis_rad: 0.3,
                true_anomaly_epoch_rad: 0.0,
                mean_motion_rad_s: 1.0e-7,
            };

            for &t in &test_sim_times {
                let mut app = App::new();
                app.add_plugins(bevy::asset::AssetPlugin::default());
                app.init_asset::<Mesh>();
                app.insert_resource(SimulationTime::new(t));

                let body_entity = app
                    .world_mut()
                    .spawn(CelestialBody {
                        config: config.clone(),
                        global_position: GlobalPosition::ZERO,
                        index: 0,
                    })
                    .id();

                let parent_entity = app.world_mut().spawn(Transform::IDENTITY).id();

                let mut mesh = Mesh::new(
                    PrimitiveTopology::PointList,
                    RenderAssetUsages::default(),
                );
                mesh.insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![[0.0_f32, 0.0_f32, 0.0_f32]; ORBIT_DOT_COUNT],
                );
                let mesh_handle = app.world_mut().resource_mut::<Assets<Mesh>>().add(mesh);

                let orbit_entity = app
                    .world_mut()
                    .spawn(DynamicOrbitDots {
                        mesh_handle: mesh_handle.clone(),
                        body_entity,
                    })
                    .id();
                app.world_mut().entity_mut(parent_entity).add_child(orbit_entity);

                app.add_systems(Update, update_orbital_dots_system);
                app.update();

                let meshes = app.world().resource::<Assets<Mesh>>();
                let mesh = meshes.get(&mesh_handle).unwrap();
                let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) =
                    mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap()
                else {
                    panic!("Expected Float32x3 positions");
                };

                assert_eq!(pts.len(), ORBIT_DOT_COUNT);
                for pt in pts {
                    assert!(!pt[0].is_nan() && !pt[0].is_infinite(), "X must be finite for e={e}");
                    assert!(!pt[1].is_nan() && !pt[1].is_infinite(), "Y must be finite for e={e}");
                    assert!(!pt[2].is_nan() && !pt[2].is_infinite(), "Z must be finite for e={e}");

                    let r = f64::from(pt[0] * pt[0] + pt[1] * pt[1] + pt[2] * pt[2]).sqrt();
                    let min_expected = config.semi_major_axis_m * (1.0 - e) * 0.999;
                    let max_expected = config.semi_major_axis_m * (1.0 + e) * 1.001;
                    assert!(
                        r >= min_expected && r <= max_expected,
                        "Radius {r} out of bounds [{min_expected}, {max_expected}] for e={e}"
                    );
                }
            }
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

    fn create_test_body_config(name: &str) -> AstroNodeConfig {
        AstroNodeConfig {
            name: name.into(),
            parent_name: None,
            mass_kg: 1.0,
            radius_m: 0.01,
            semi_major_axis_m: 0.0,
            eccentricity: 0.0,
            inclination_rad: 0.0,
            longitude_of_ascending_node_rad: 0.0,
            argument_of_periapsis_rad: 0.0,
            true_anomaly_epoch_rad: 0.0,
            mean_motion_rad_s: 0.0,
        }
    }

    fn setup_test_pick_app() -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins((
            bevy::window::WindowPlugin {
                primary_window: None,
                ..default()
            },
            bevy::asset::AssetPlugin::default(),
            bevy::render::camera::CameraPlugin,
        ));
        app.init_asset::<Image>();
        app.init_resource::<ButtonInput<MouseButton>>();

        let win = app
            .world_mut()
            .spawn((
                Window {
                    resolution: (800.0_f32, 600.0_f32).into(),
                    ..default()
                },
                bevy::window::PrimaryWindow,
            ))
            .id();

        let cam_transform = Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y);
        app.world_mut().spawn((
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(win),
                ),
                ..default()
            },
            Projection::Perspective(PerspectiveProjection::default()),
            cam_transform,
            GlobalTransform::from(cam_transform),
            GlobeOrbitCamera::default(),
        ));

        // Body 0: Far body along Z axis at (0, 0, 0) -> distance 100 to cam
        app.world_mut().spawn((
            CelestialBody {
                config: create_test_body_config("FarBody"),
                global_position: GlobalPosition::ZERO,
                index: 0,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        ));

        // Body 1: Near body along Z axis at (0, 0, 50) -> distance 50 to cam
        app.world_mut().spawn((
            CelestialBody {
                config: create_test_body_config("NearBody"),
                global_position: GlobalPosition { x: 50.0, y: 0.0, z: 0.0 },
                index: 1,
            },
            Transform::from_xyz(0.0, 0.0, 50.0),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 50.0)),
        ));

        // Body 2: Behind camera at (0, 0, 150) -> camera is at 100 looking towards -Z
        app.world_mut().spawn((
            CelestialBody {
                config: create_test_body_config("BehindBody"),
                global_position: GlobalPosition { x: 150.0, y: 0.0, z: 0.0 },
                index: 2,
            },
            Transform::from_xyz(0.0, 0.0, 150.0),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 150.0)),
        ));

        app.insert_resource(FloatingOrigin {
            focused_index: 0,
            focused_name: "FarBody".into(),
            focused_position: GlobalPosition::ZERO,
            body_names: vec!["FarBody".into(), "NearBody".into(), "BehindBody".into()],
        });

        app.add_systems(
            Update,
            (
                bevy::render::camera::camera_system::<Projection>,
                mouse_pick_astronode_system,
            ).chain(),
        );

        (app, win)
    }

    #[test]
    fn test_mouse_pick_z_depth_priority() {
        let (mut app, win) = setup_test_pick_app();
        app.update();

        // Frame 1: Press at center (400, 300)
        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.press(MouseButton::Left);
        }
        app.update();

        // Frame 2: Release at center (400, 300) without drag
        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.clear_just_pressed(MouseButton::Left);
            mouse.release(MouseButton::Left);
        }
        app.update();

        let origin = app.world().resource::<FloatingOrigin>();
        assert_eq!(
            origin.focused_index, 1,
            "Picking should prioritize the closer body in 3D distance"
        );
        assert_eq!(origin.focused_name, "NearBody");
    }

    #[test]
    fn test_mouse_pick_click_vs_drag() {
        let (mut app, win) = setup_test_pick_app();
        app.update();

        // Case 1: Drag displacement > 5px should NOT trigger pick
        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.press(MouseButton::Left);
        }
        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(420.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.clear_just_pressed(MouseButton::Left);
            mouse.release(MouseButton::Left);
        }
        app.update();

        let origin = app.world().resource::<FloatingOrigin>();
        assert_eq!(
            origin.focused_index, 0,
            "Mouse drag exceeding displacement threshold must not trigger picking"
        );

        // Case 2: Sub-threshold movement (<= 5px) DOES trigger pick
        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.press(MouseButton::Left);
        }
        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(402.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.clear_just_pressed(MouseButton::Left);
            mouse.release(MouseButton::Left);
        }
        app.update();

        let origin = app.world().resource::<FloatingOrigin>();
        assert_eq!(
            origin.focused_index, 1,
            "Small mouse movement within threshold must register as a valid click"
        );
    }

    #[test]
    fn test_mouse_pick_behind_camera_filtered() {
        let mut app = App::new();
        app.add_plugins((
            bevy::window::WindowPlugin {
                primary_window: None,
                ..default()
            },
            bevy::asset::AssetPlugin::default(),
            bevy::render::camera::CameraPlugin,
        ));
        app.init_asset::<Image>();
        app.init_resource::<ButtonInput<MouseButton>>();

        let win = app
            .world_mut()
            .spawn((
                Window {
                    resolution: (800.0_f32, 600.0_f32).into(),
                    ..default()
                },
                bevy::window::PrimaryWindow,
            ))
            .id();

        let cam_transform = Transform::from_xyz(0.0, 0.0, 100.0).looking_at(Vec3::ZERO, Vec3::Y);
        app.world_mut().spawn((
            Camera {
                target: bevy::render::camera::RenderTarget::Window(
                    bevy::window::WindowRef::Entity(win),
                ),
                ..default()
            },
            Projection::Perspective(PerspectiveProjection::default()),
            cam_transform,
            GlobalTransform::from(cam_transform),
            GlobeOrbitCamera::default(),
        ));

        // Only spawn a body behind the camera at Z = 150
        app.world_mut().spawn((
            CelestialBody {
                config: create_test_body_config("BehindOnly"),
                global_position: GlobalPosition { x: 150.0, y: 0.0, z: 0.0 },
                index: 0,
            },
            Transform::from_xyz(0.0, 0.0, 150.0),
            GlobalTransform::from(Transform::from_xyz(0.0, 0.0, 150.0)),
        ));

        app.insert_resource(FloatingOrigin {
            focused_index: 99,
            focused_name: "None".into(),
            focused_position: GlobalPosition::ZERO,
            body_names: vec!["BehindOnly".into()],
        });

        app.add_systems(
            Update,
            (
                bevy::render::camera::camera_system::<Projection>,
                mouse_pick_astronode_system,
            ).chain(),
        );

        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.press(MouseButton::Left);
        }
        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(400.0, 300.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.clear_just_pressed(MouseButton::Left);
            mouse.release(MouseButton::Left);
        }
        app.update();

        let origin = app.world().resource::<FloatingOrigin>();
        assert_eq!(
            origin.focused_index, 99,
            "Behind-camera bodies returning None for viewport coordinates must not be picked"
        );
    }

    #[test]
    fn test_mouse_pick_empty_space() {
        let (mut app, win) = setup_test_pick_app();
        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(10.0, 10.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.press(MouseButton::Left);
        }
        app.update();

        {
            let mut window = app.world_mut().get_mut::<Window>(win).unwrap();
            window.set_cursor_position(Some(Vec2::new(10.0, 10.0)));
            let mut mouse = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
            mouse.clear_just_pressed(MouseButton::Left);
            mouse.release(MouseButton::Left);
        }
        app.update();

        let origin = app.world().resource::<FloatingOrigin>();
        assert_eq!(
            origin.focused_index, 0,
            "Clicking empty space must not change celestial focus"
        );
    }

    #[test]
    fn test_advance_simulation_time_scaled_by_warp() {
        let mut app = App::new();
        app.add_plugins(bevy::time::TimePlugin);
        app.insert_resource(SimulationTime::default());

        let mut warp = TimeWarp::default();
        warp.set_level(crate::systems::time_warp::WarpLevel::HourPerSec); // 3600x
        app.insert_resource(warp);

        app.add_systems(Update, advance_simulation_time);

        // Advance simulation time
        app.update();

        let sim_time = app.world().resource::<SimulationTime>();
        assert!(sim_time.elapsed_seconds >= 0.0);
    }

    #[test]
    fn test_celestial_body_positions_advance_with_time() {
        let earth_config = AstroNodeConfig {
            name: "Earth".into(),
            parent_name: Some("Sun".into()),
            mass_kg: 5.972e24,
            radius_m: 6.371e6,
            semi_major_axis_m: 1.495_978_7e11,
            eccentricity: 0.0167,
            inclination_rad: 0.0,
            longitude_of_ascending_node_rad: 0.0,
            argument_of_periapsis_rad: 0.0,
            true_anomaly_epoch_rad: 0.0,
            mean_motion_rad_s: 1.991e-7,
        };

        let configs = vec![
            AstroNodeConfig {
                name: "Sun".into(),
                parent_name: None,
                mass_kg: 1.989e30,
                radius_m: 6.96e8,
                semi_major_axis_m: 0.0,
                eccentricity: 0.0,
                inclination_rad: 0.0,
                longitude_of_ascending_node_rad: 0.0,
                argument_of_periapsis_rad: 0.0,
                true_anomaly_epoch_rad: 0.0,
                mean_motion_rad_s: 0.0,
            },
            earth_config,
        ];

        let pos_t0 = compute_all_positions_at_time(&configs, 0.0);
        let pos_t_half_year = compute_all_positions_at_time(&configs, std::f64::consts::PI / 1.991e-7);

        let earth_t0 = pos_t0.get("Earth").unwrap();
        let earth_half_year = pos_t_half_year.get("Earth").unwrap();

        // After half an orbit, Earth's position should be on opposite side of the Sun
        assert!((earth_t0.x + earth_half_year.x).abs() < 1e10);
    }
}
