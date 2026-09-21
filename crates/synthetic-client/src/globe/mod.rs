pub mod camera;
pub mod material;
pub mod mesh;
pub mod viewport;

use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use camera::{orbit_camera_input_system, orbit_camera_transform_system, GlobeOrbitCamera};
use material::{GlobeMaterial, OrbitMaterial};
use viewport::{sync_globe_viewport, GlobeCameraMarker};

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<GlobeMaterial>::default())
            .add_plugins(MaterialPlugin::<OrbitMaterial>::default())
            .add_systems(Startup, setup_solar_system_camera)
            .add_systems(
                Update,
                (
                    sync_globe_viewport,
                    orbit_camera_input_system,
                    orbit_camera_transform_system,
                ),
            );
    }
}

fn setup_solar_system_camera(mut commands: Commands) {
    // 3.0 degrees FOV (~0.05236 rad) perspective projection
    let fov_three_deg = 3.0 * std::f32::consts::PI / 180.0;

    let initial_distance = 1.2e9_f32;
    let initial_pitch = 0.6_f32;
    let initial_yaw = 0.0_f32;
    let init_x = initial_distance * initial_pitch.cos() * initial_yaw.cos();
    let init_y = initial_distance * initial_pitch.cos() * initial_yaw.sin();
    let init_z = initial_distance * initial_pitch.sin();

    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.04, 0.04, 0.06)),
            viewport: None,
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: fov_three_deg,
            near: 10.0,
            far: 1e15,
            ..default()
        }),
        Transform::from_xyz(init_x, init_y, init_z).looking_at(Vec3::ZERO, Vec3::Z),
        GlobeCameraMarker,
        GlobeOrbitCamera {
            distance: initial_distance,
            target_distance: initial_distance,
            min_distance: 100.0,
            max_distance: 1e14,
            zoom_sensitivity: 0.25,
            ..default()
        },
    ));
}
