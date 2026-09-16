pub mod camera;
pub mod material;
pub mod mesh;
pub mod viewport;

use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use camera::{orbit_camera_input_system, orbit_camera_transform_system, GlobeOrbitCamera};
use material::GlobeMaterial;
use mesh::generate_fibonacci_globe_mesh;
use viewport::{sync_globe_viewport, GlobeCameraMarker};

#[derive(Component)]
pub struct GlobeMarker;

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<GlobeMaterial>::default())
            .add_systems(Startup, setup_globe_scene)
            .add_systems(
                Update,
                (
                    sync_globe_viewport,
                    rotate_globe,
                    orbit_camera_input_system,
                    orbit_camera_transform_system,
                ),
            );
    }
}

fn setup_globe_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GlobeMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 0,
            clear_color: ClearColorConfig::Custom(Color::srgb(0.08, 0.08, 0.10)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobeCameraMarker,
        GlobeOrbitCamera::default(),
    ));

    let globe_mesh = generate_fibonacci_globe_mesh(10_000, 1.5);
    let globe_material = GlobeMaterial {
        base_color: LinearRgba::new(0.3, 0.75, 1.0, 1.0),
        atmosphere_color: LinearRgba::new(0.1, 0.4, 0.8, 1.0),
    };

    commands.spawn((
        Mesh3d(meshes.add(globe_mesh)),
        MeshMaterial3d(materials.add(globe_material)),
        Transform::IDENTITY,
        GlobeMarker,
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn rotate_globe(time: Res<Time>, mut query: Query<&mut Transform, With<GlobeMarker>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.05 * time.delta_secs());
    }
}
