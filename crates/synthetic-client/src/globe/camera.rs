use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use super::viewport::MainViewPanelMarker;

pub const DEFAULT_PITCH_LIMIT: f32 = 1.48;

/// Orbit camera component for planetary 3D navigation.
#[derive(Component, Debug, Clone)]
pub struct GlobeOrbitCamera {
    /// Focus point in world coordinates.
    pub focus: Vec3,
    /// Current radial distance from focus.
    pub distance: f32,
    /// Target radial distance for smooth damping.
    pub target_distance: f32,
    /// Current yaw angle (radians around Y-axis).
    pub yaw: f32,
    /// Current pitch angle (radians elevation from XZ plane).
    pub pitch: f32,
    /// Target pitch angle for smooth damping.
    pub target_pitch: f32,
    /// Target yaw angle for smooth damping.
    pub target_yaw: f32,
    /// Minimum allowed zoom distance.
    pub min_distance: f32,
    /// Maximum allowed zoom distance.
    pub max_distance: f32,
    /// Sensitivity factor for mouse drag rotation.
    pub rotate_sensitivity: f32,
    /// Sensitivity factor for mouse wheel zoom.
    pub zoom_sensitivity: f32,
    /// Exponential damping coefficient.
    pub damping: f32,
    /// Flag indicating whether active drag interaction is occurring.
    pub is_dragging: bool,
}

impl Default for GlobeOrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            distance: 4.0,
            target_distance: 4.0,
            yaw: 0.0,
            pitch: 0.2,
            target_yaw: 0.0,
            target_pitch: 0.2,
            min_distance: 1.8,
            max_distance: 12.0,
            rotate_sensitivity: 0.005,
            zoom_sensitivity: 0.3,
            damping: 10.0,
            is_dragging: false,
        }
    }
}

/// System to handle mouse drag rotation and mouse wheel zooming for `GlobeOrbitCamera`.
#[allow(clippy::needless_pass_by_value)]
pub fn orbit_camera_input_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    window_query: Query<&Window>,
    panel_query: Query<
        (&GlobalTransform, &ComputedNode, Option<&Interaction>),
        With<MainViewPanelMarker>,
    >,
    mut camera_query: Query<&mut GlobeOrbitCamera>,
) {
    let is_pressed = mouse_button_input.pressed(MouseButton::Left)
        || mouse_button_input.pressed(MouseButton::Right);
    let just_pressed = mouse_button_input.just_pressed(MouseButton::Left)
        || mouse_button_input.just_pressed(MouseButton::Right);

    let is_hovered = if let Ok((transform, node, interaction)) = panel_query.get_single() {
        let interaction_hovered = interaction.is_some_and(|i| *i != Interaction::None);
        let rect_hovered = window_query
            .get_single()
            .ok()
            .and_then(|win| {
                win.cursor_position().map(|cursor_pos| {
                    let center = transform.translation().truncate();
                    let size = node.size();
                    let rect = Rect::from_center_size(center, size);
                    rect.contains(cursor_pos)
                })
            })
            .unwrap_or(false);
        interaction_hovered || rect_hovered
    } else {
        false
    };

    let mut motion_delta = Vec2::ZERO;
    for motion in mouse_motion_events.read() {
        motion_delta += motion.delta;
    }

    let mut scroll_y = 0.0;
    for ev in mouse_wheel_events.read() {
        let y = match ev.unit {
            MouseScrollUnit::Line => ev.y,
            MouseScrollUnit::Pixel => ev.y * 0.05,
        };
        scroll_y += y;
    }

    for mut camera in &mut camera_query {
        if just_pressed && is_hovered {
            camera.is_dragging = true;
        } else if !is_pressed {
            camera.is_dragging = false;
        }

        if camera.is_dragging && motion_delta != Vec2::ZERO {
            camera.target_yaw += -motion_delta.x * camera.rotate_sensitivity;
            camera.target_pitch += motion_delta.y * camera.rotate_sensitivity;
            camera.target_pitch =
                camera.target_pitch.clamp(-DEFAULT_PITCH_LIMIT, DEFAULT_PITCH_LIMIT);
        }

        if (is_hovered || camera.is_dragging) && scroll_y.abs() > f32::EPSILON {
            camera.target_distance = (camera.target_distance
                * (1.0 - scroll_y * camera.zoom_sensitivity * 0.1))
                .clamp(camera.min_distance, camera.max_distance);
        }
    }
}

/// System to interpolate camera position and orientation using exponential damping.
#[allow(clippy::needless_pass_by_value)]
pub fn orbit_camera_transform_system(
    time: Res<Time>,
    mut query: Query<(&mut GlobeOrbitCamera, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (mut camera, mut transform) in &mut query {
        let blend = 1.0 - (-camera.damping * dt).exp();

        camera.distance += (camera.target_distance - camera.distance) * blend;
        camera.yaw += (camera.target_yaw - camera.yaw) * blend;
        camera.pitch += (camera.target_pitch - camera.pitch) * blend;

        let x = camera.focus.x + camera.distance * camera.pitch.cos() * camera.yaw.sin();
        let y = camera.focus.y + camera.distance * camera.pitch.sin();
        let z = camera.focus.z + camera.distance * camera.pitch.cos() * camera.yaw.cos();

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(camera.focus, Vec3::Y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_globe_orbit_camera_default() {
        let cam = GlobeOrbitCamera::default();
        assert_eq!(cam.focus, Vec3::ZERO);
        assert!((cam.distance - 4.0).abs() < f32::EPSILON);
        assert!((cam.target_distance - 4.0).abs() < f32::EPSILON);
        assert!((cam.pitch - 0.2).abs() < f32::EPSILON);
        assert!((cam.yaw - 0.0).abs() < f32::EPSILON);
        assert!(!cam.is_dragging);
    }

    #[test]
    fn test_spherical_to_cartesian() {
        let distance = 4.0;
        let pitch = 0.0_f32;
        let yaw = 0.0_f32;
        let x = distance * pitch.cos() * yaw.sin();
        let y = distance * pitch.sin();
        let z = distance * pitch.cos() * yaw.cos();
        assert!((x - 0.0).abs() < 1e-5);
        assert!((y - 0.0).abs() < 1e-5);
        assert!((z - 4.0).abs() < 1e-5);
    }
}
