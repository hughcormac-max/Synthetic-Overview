//! UI systems for solar ecliptic orientation widget and astrobody selection.
//!
//! Grounded in SSOT-UIX-001 and PLAN-015.

use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::{ClearColorConfig, Viewport};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;

use crate::globe::camera::GlobeOrbitCamera;
use crate::globe::material::OrbitMaterial;
use crate::systems::astronomy::{focus_on_body, CelestialBody, FloatingOrigin};

/// Color for the ecliptic circle loop.
pub const ECLIPTIC_CIRCLE_COLOR: LinearRgba = LinearRgba::new(0.25, 0.78, 0.88, 0.65);

/// Color for the +Z ecliptic pole indicator.
pub const ECLIPTIC_POLE_COLOR: LinearRgba = LinearRgba::new(0.40, 0.90, 1.00, 0.95);

/// Color for ecliptic cardinal tick marks.
pub const ECLIPTIC_TICK_COLOR: LinearRgba = LinearRgba::new(0.60, 0.75, 0.85, 0.50);

/// Normal button background tint.
pub const BUTTON_NORMAL_BG: Color = Color::srgba(0.10, 0.10, 0.15, 0.75);

/// Normal button border color.
pub const BUTTON_NORMAL_BORDER: Color = Color::srgba(0.20, 0.22, 0.28, 0.50);

/// Hovered button background tint.
pub const BUTTON_HOVER_BG: Color = Color::srgba(0.20, 0.28, 0.38, 0.85);

/// Hovered button border color.
pub const BUTTON_HOVER_BORDER: Color = Color::srgba(0.35, 0.55, 0.75, 0.80);

/// Active/focused button background tint.
pub const BUTTON_ACTIVE_BG: Color = Color::srgba(0.15, 0.45, 0.60, 0.90);

/// Active/focused button border color.
pub const BUTTON_ACTIVE_BORDER: Color = Color::srgba(0.40, 0.80, 1.00, 0.95);

/// Number of discrete segments used for the orientation widget circle.
pub const WIDGET_CIRCLE_SEGMENTS: usize = 64;

/// Radius of the ecliptic orientation widget circle.
pub const WIDGET_CIRCLE_RADIUS: f32 = 1.0;

/// Length of the +Z ecliptic pole indicator line.
pub const WIDGET_POLE_HEIGHT: f32 = 1.2;

/// Marker for the 3D secondary camera rendering the ecliptic orientation widget.
#[derive(Component, Debug, Clone)]
pub struct EclipticOrientationCameraMarker;

/// Marker for entities belonging to the ecliptic orientation widget.
#[derive(Component, Debug, Clone)]
pub struct EclipticOrientationWidget;

/// Marker for the floating astrobody selector container.
#[derive(Component, Debug, Clone)]
pub struct AstrobodySelectorUI;

/// Marker for the scrollable list container inside the astrobody selector.
#[derive(Component, Debug, Clone)]
pub struct AstrobodyScrollArea;

/// Component attached to each celestial body selection button.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct AstrobodyButton {
    pub index: usize,
}

/// Marker component attached to the root entity of the keybinding help modal overlay.
#[derive(Component, Debug, Clone)]
pub struct KeybindingHelpModal;

/// Generates a closed line loop mesh in the XY plane representing the ecliptic circle.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn create_circle_mesh(radius: f32, segments: usize) -> Mesh {
    let segments = segments.max(3);
    let step = std::f32::consts::TAU / (segments as f32);

    let mut positions = Vec::with_capacity(segments + 1);
    let mut normals = Vec::with_capacity(segments + 1);

    for i in 0..=segments {
        let angle = (i % segments) as f32 * step;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let z = 0.0_f32;

        positions.push([x, y, z]);
        normals.push([0.0, 0.0, 1.0]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh
}

/// Generates a line strip mesh connecting the specified sequence of 3D points.
#[must_use]
pub fn create_line_strip_mesh(points: &[Vec3]) -> Mesh {
    let positions: Vec<[f32; 3]> = points.iter().map(|p| [p.x, p.y, p.z]).collect();
    let normals = vec![[0.0, 0.0, 1.0]; positions.len()];

    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh
}

/// Spawns the secondary 3D camera and geometry for the ecliptic orientation widget.
#[allow(clippy::needless_pass_by_value)]
pub fn setup_orientation_widget(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut orbit_materials: ResMut<Assets<OrbitMaterial>>,
    windows: Query<&Window>,
) {
    let widget_layer = RenderLayers::layer(1);

    let initial_width = windows
        .get_single()
        .map_or(1280, Window::physical_width);

    let initial_viewport = Viewport {
        physical_position: UVec2::new(initial_width.saturating_sub(130), 40),
        physical_size: UVec2::new(100, 100),
        depth: 0.0..1.0,
    };

    // Secondary 3D camera layered on top (order: 2) rendering RenderLayers::layer(1)
    commands.spawn((
        Camera3d::default(),
        Camera {
            order: 2,
            clear_color: ClearColorConfig::None,
            viewport: Some(initial_viewport),
            ..default()
        },
        Projection::Perspective(PerspectiveProjection {
            fov: 38.0_f32.to_radians(),
            near: 0.1,
            far: 50.0,
            ..default()
        }),
        Transform::from_xyz(0.0, -3.8, 2.0).looking_at(Vec3::ZERO, Vec3::Z),
        widget_layer.clone(),
        EclipticOrientationCameraMarker,
    ));

    // Shared materials
    let circle_material = orbit_materials.add(OrbitMaterial {
        color: ECLIPTIC_CIRCLE_COLOR,
    });
    let pole_material = orbit_materials.add(OrbitMaterial {
        color: ECLIPTIC_POLE_COLOR,
    });
    let tick_material = orbit_materials.add(OrbitMaterial {
        color: ECLIPTIC_TICK_COLOR,
    });

    // 1. Ecliptic circle mesh (radius 1.0 in XY plane)
    let circle_mesh = meshes.add(create_circle_mesh(WIDGET_CIRCLE_RADIUS, WIDGET_CIRCLE_SEGMENTS));
    commands.spawn((
        Mesh3d(circle_mesh),
        MeshMaterial3d(circle_material),
        Transform::IDENTITY,
        widget_layer.clone(),
        EclipticOrientationWidget,
    ));

    // 2. Z-axis pole line from (0,0,0) to (0,0,1.2)
    let pole_mesh = meshes.add(create_line_strip_mesh(&[
        Vec3::ZERO,
        Vec3::new(0.0, 0.0, WIDGET_POLE_HEIGHT),
    ]));
    commands.spawn((
        Mesh3d(pole_mesh),
        MeshMaterial3d(pole_material.clone()),
        Transform::IDENTITY,
        widget_layer.clone(),
        EclipticOrientationWidget,
    ));

    // 3. Arrowhead at top of +Z pole
    let arrow_mesh_x = meshes.add(create_line_strip_mesh(&[
        Vec3::new(-0.06, 0.0, 1.12),
        Vec3::new(0.0, 0.0, WIDGET_POLE_HEIGHT),
        Vec3::new(0.06, 0.0, 1.12),
    ]));
    commands.spawn((
        Mesh3d(arrow_mesh_x),
        MeshMaterial3d(pole_material.clone()),
        Transform::IDENTITY,
        widget_layer.clone(),
        EclipticOrientationWidget,
    ));

    let arrow_mesh_y = meshes.add(create_line_strip_mesh(&[
        Vec3::new(0.0, -0.06, 1.12),
        Vec3::new(0.0, 0.0, WIDGET_POLE_HEIGHT),
        Vec3::new(0.0, 0.06, 1.12),
    ]));
    commands.spawn((
        Mesh3d(arrow_mesh_y),
        MeshMaterial3d(pole_material),
        Transform::IDENTITY,
        widget_layer.clone(),
        EclipticOrientationWidget,
    ));

    // 4. Subtle cardinal tick marks along +/- X and +/- Y
    let cardinal_ticks = [
        [Vec3::new(0.90, 0.0, 0.0), Vec3::new(1.08, 0.0, 0.0)],
        [Vec3::new(-0.90, 0.0, 0.0), Vec3::new(-1.08, 0.0, 0.0)],
        [Vec3::new(0.0, 0.90, 0.0), Vec3::new(0.0, 1.08, 0.0)],
        [Vec3::new(0.0, -0.90, 0.0), Vec3::new(0.0, -1.08, 0.0)],
    ];

    for tick in &cardinal_ticks {
        let tick_mesh = meshes.add(create_line_strip_mesh(tick));
        commands.spawn((
            Mesh3d(tick_mesh),
            MeshMaterial3d(tick_material.clone()),
            Transform::IDENTITY,
            widget_layer.clone(),
            EclipticOrientationWidget,
        ));
    }
}

/// Keeps the orientation widget viewport anchored in the top-right corner on window resize.
#[allow(clippy::needless_pass_by_value)]
pub fn update_orientation_widget_viewport(
    windows: Query<&Window>,
    mut camera_query: Query<&mut Camera, With<EclipticOrientationCameraMarker>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let width = window.physical_width();
    let new_pos = UVec2::new(width.saturating_sub(130), 40);
    let new_size = UVec2::new(100, 100);

    if let Some(ref mut vp) = camera.viewport {
        if vp.physical_position != new_pos || vp.physical_size != new_size {
            vp.physical_position = new_pos;
            vp.physical_size = new_size;
        }
    } else {
        camera.viewport = Some(Viewport {
            physical_position: new_pos,
            physical_size: new_size,
            depth: 0.0..1.0,
        });
    }
}

/// Synchronizes the widget camera transform with the main `GlobeOrbitCamera` orientation.
#[allow(clippy::needless_pass_by_value)]
pub fn sync_orientation_widget_camera(
    orbit_camera_query: Query<&GlobeOrbitCamera>,
    mut widget_camera_query: Query<&mut Transform, With<EclipticOrientationCameraMarker>>,
) {
    let Ok(orbit_cam) = orbit_camera_query.get_single() else {
        return;
    };
    let Ok(mut widget_transform) = widget_camera_query.get_single_mut() else {
        return;
    };

    let distance = 3.8_f32;
    let cos_pitch = orbit_cam.pitch.cos();
    let sin_pitch = orbit_cam.pitch.sin();
    let cos_yaw = orbit_cam.yaw.cos();
    let sin_yaw = orbit_cam.yaw.sin();

    let x = distance * cos_pitch * cos_yaw;
    let y = distance * cos_pitch * sin_yaw;
    let z = distance * sin_pitch;

    *widget_transform = Transform::from_xyz(x, y, z).looking_at(Vec3::ZERO, Vec3::Z);
}

/// Builds the floating astrobody selector UI overlay listing all available celestial bodies.
#[allow(clippy::needless_pass_by_value)]
pub fn spawn_astrobody_selector(
    mut commands: Commands,
    origin: Res<FloatingOrigin>,
    ui_camera_query: Query<Entity, With<crate::UiCameraMarker>>,
) {
    let ui_camera = ui_camera_query.get_single().ok();

    // Floating overlay container in top-left
    let mut root_cmd = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            width: Val::Px(150.0),
            max_height: Val::Vh(75.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            row_gap: Val::Px(6.0),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BorderRadius::all(Val::Px(4.0)),
        BackgroundColor(Color::srgba(0.04, 0.05, 0.07, 0.85)),
        BorderColor(Color::srgba(0.20, 0.25, 0.35, 0.60)),
        AstrobodySelectorUI,
    ));

    if let Some(cam) = ui_camera {
        root_cmd.insert(TargetCamera(cam));
    }

    let root_entity = root_cmd.id();

    // Compact header
    let header_entity = commands
        .spawn((
            Text::new("CELESTIAL BODIES"),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(0.55, 0.70, 0.85)),
            Node {
                margin: UiRect::bottom(Val::Px(2.0)),
                ..default()
            },
        ))
        .id();

    // Scrollable list container
    let scroll_container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(2.0),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollPosition::default(),
            Interaction::default(),
            AstrobodyScrollArea,
        ))
        .id();

    // Add buttons for each celestial body
    let mut button_entities = Vec::with_capacity(origin.body_names.len());

    for (index, name) in origin.body_names.iter().enumerate() {
        let is_active = index == origin.focused_index;
        let bg_color = if is_active {
            BUTTON_ACTIVE_BG
        } else {
            BUTTON_NORMAL_BG
        };
        let border_color = if is_active {
            BUTTON_ACTIVE_BORDER
        } else {
            BUTTON_NORMAL_BORDER
        };

        let button_entity = commands
            .spawn((
                Button,
                AstrobodyButton { index },
                Interaction::default(),
                Node {
                    padding: UiRect::axes(Val::Px(6.0), Val::Px(3.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BorderRadius::all(Val::Px(3.0)),
                BackgroundColor(bg_color),
                BorderColor(border_color),
            ))
            .with_child((
                Text::new(name.clone()),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.90, 0.92, 0.94)),
            ))
            .id();

        button_entities.push(button_entity);
    }

    commands.entity(scroll_container).add_children(&button_entities);
    commands.entity(root_entity).add_children(&[header_entity, scroll_container]);
}

/// Handles clicking and hovering on astrobody selector buttons, updating visuals and focus.
#[allow(clippy::needless_pass_by_value)]
pub fn handle_astrobody_selector_interaction(
    mut button_query: Query<
        (
            &Interaction,
            &AstrobodyButton,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        With<Button>,
    >,
    mut origin: ResMut<FloatingOrigin>,
    body_query: Query<&CelestialBody>,
    mut camera_query: Query<&mut GlobeOrbitCamera>,
) {
    let mut clicked_index = None;
    for (interaction, button, _, _) in &button_query {
        if *interaction == Interaction::Pressed {
            clicked_index = Some(button.index);
            break;
        }
    }

    if let Some(idx) = clicked_index {
        focus_on_body(idx, &mut origin, &body_query, &mut camera_query);
    }

    let current_focus = origin.focused_index;

    for (interaction, button, mut bg, mut border) in &mut button_query {
        let is_active = button.index == current_focus;
        let (target_bg, target_border) = if is_active {
            (BUTTON_ACTIVE_BG, BUTTON_ACTIVE_BORDER)
        } else {
            match *interaction {
                Interaction::Hovered => (BUTTON_HOVER_BG, BUTTON_HOVER_BORDER),
                Interaction::Pressed => (BUTTON_ACTIVE_BG, BUTTON_ACTIVE_BORDER),
                Interaction::None => (BUTTON_NORMAL_BG, BUTTON_NORMAL_BORDER),
            }
        };

        if bg.0 != target_bg {
            bg.0 = target_bg;
        }
        if border.0 != target_border {
            border.0 = target_border;
        }
    }
}

/// Handles mouse wheel scrolling over the astrobody selector list.
#[allow(clippy::needless_pass_by_value)]
pub fn handle_astrobody_selector_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    button_query: Query<&Interaction, With<AstrobodyButton>>,
    mut scroll_query: Query<
        (&Interaction, &mut ScrollPosition),
        With<AstrobodyScrollArea>,
    >,
) {
    let mut scroll_y = 0.0;
    for ev in mouse_wheel_events.read() {
        let y = match ev.unit {
            MouseScrollUnit::Line => ev.y * 24.0,
            MouseScrollUnit::Pixel => ev.y,
        };
        scroll_y += y;
    }

    if scroll_y.abs() > f32::EPSILON {
        let any_button_hovered = button_query.iter().any(|i| *i != Interaction::None);

        for (container_interaction, mut scroll_pos) in &mut scroll_query {
            if *container_interaction != Interaction::None || any_button_hovered {
                scroll_pos.offset_y = (scroll_pos.offset_y - scroll_y).max(0.0);
            }
        }
    }
}

/// Toggles keybinding help modal visibility when H is pressed, and hides it when Escape is pressed.
#[allow(clippy::needless_pass_by_value)]
pub fn toggle_keybinding_help_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Visibility, With<KeybindingHelpModal>>,
) {
    let toggle_h = keyboard.just_pressed(KeyCode::KeyH);
    let close_esc = keyboard.just_pressed(KeyCode::Escape);

    if !toggle_h && !close_esc {
        return;
    }

    for mut vis in &mut query {
        if toggle_h {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Inherited,
                Visibility::Inherited | Visibility::Visible => Visibility::Hidden,
            };
        } else if close_esc && *vis != Visibility::Hidden {
            *vis = Visibility::Hidden;
        }
    }
}

fn spawn_section_header(commands: &mut Commands, parent: Entity, title: &str) {
    let header = commands
        .spawn((
            Text::new(title),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.55, 0.65, 0.75)),
            Node {
                margin: UiRect::top(Val::Px(4.0)),
                ..default()
            },
        ))
        .id();
    commands.entity(parent).add_child(header);
}

fn spawn_keybinding_row(
    commands: &mut Commands,
    parent: Entity,
    key_label: &str,
    description: &str,
) {
    let row = commands
        .spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            padding: UiRect::vertical(Val::Px(1.5)),
            ..default()
        })
        .id();

    let badge = commands
        .spawn((
            Text::new(key_label),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(0.40, 0.85, 1.0)),
            Node {
                padding: UiRect::axes(Val::Px(6.0), Val::Px(2.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BorderRadius::all(Val::Px(3.0)),
            BackgroundColor(Color::srgba(0.10, 0.18, 0.28, 0.80)),
            BorderColor(Color::srgba(0.25, 0.45, 0.65, 0.60)),
        ))
        .id();

    let desc = commands
        .spawn((
            Text::new(description),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(0.85, 0.88, 0.92)),
        ))
        .id();

    commands.entity(row).add_child(badge);
    commands.entity(row).add_child(desc);
    commands.entity(parent).add_child(row);
}

/// Spawns the centered keybinding help modal overlay (hidden by default).
#[allow(clippy::needless_pass_by_value)]
pub fn spawn_keybinding_help_modal(
    mut commands: Commands,
    ui_camera_query: Query<Entity, With<crate::UiCameraMarker>>,
) {
    let ui_camera = ui_camera_query.get_single().ok();

    let mut root_cmd = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        KeybindingHelpModal,
    ));

    if let Some(cam) = ui_camera {
        root_cmd.insert(TargetCamera(cam));
    }
    let root_entity = root_cmd.id();

    let card = commands
        .spawn((
            Node {
                width: Val::Px(380.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                row_gap: Val::Px(8.0),
                border: UiRect::all(Val::Px(1.5)),
                ..default()
            },
            BorderRadius::all(Val::Px(8.0)),
            BackgroundColor(Color::srgba(0.04, 0.06, 0.10, 0.94)),
            BorderColor(Color::srgba(0.25, 0.55, 0.80, 0.75)),
        ))
        .id();
    commands.entity(root_entity).add_child(card);

    let header = commands
        .spawn((
            Text::new("KEYBOARD CONTROLS & SHORTCUTS"),
            TextFont {
                font_size: 13.0,
                ..default()
            },
            TextColor(Color::srgb(0.40, 0.90, 1.0)),
            Node {
                margin: UiRect::bottom(Val::Px(4.0)),
                ..default()
            },
        ))
        .id();
    commands.entity(card).add_child(header);

    // Section 1: Simulation Speed
    spawn_section_header(&mut commands, card, "SIMULATION SPEED");
    spawn_keybinding_row(&mut commands, card, "SPACE", "Pause / Resume simulation");
    spawn_keybinding_row(&mut commands, card, "0 - 5", "Jump: Pause, 1s, 1m, 1h, 1d, 1mo");
    spawn_keybinding_row(&mut commands, card, "[ / ]", "Step warp speed down / up");

    // Section 2: Camera Navigation
    spawn_section_header(&mut commands, card, "CAMERA NAVIGATION");
    spawn_keybinding_row(&mut commands, card, "LMB Drag", "Orbit camera around focus");
    spawn_keybinding_row(&mut commands, card, "RMB Drag", "Pan camera offset");
    spawn_keybinding_row(&mut commands, card, "Scroll", "Zoom camera distance");

    // Section 3: Navigation & Dismiss
    spawn_section_header(&mut commands, card, "SELECTION & HELP");
    spawn_keybinding_row(&mut commands, card, "LMB Click", "Select & focus celestial body");
    spawn_keybinding_row(&mut commands, card, "H / ESC", "Toggle / close this help modal");
}

/// UI plugin managing the ecliptic orientation widget and celestial body selection UI.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_orientation_widget)
            .add_systems(
                Startup,
                (
                    spawn_astrobody_selector
                        .after(crate::systems::astronomy::setup_solar_system)
                        .after(crate::setup_ui),
                    spawn_keybinding_help_modal.after(crate::setup_ui),
                ),
            )
            .add_systems(
                Update,
                (
                    update_orientation_widget_viewport,
                    sync_orientation_widget_camera,
                    handle_astrobody_selector_interaction,
                    handle_astrobody_selector_scroll,
                    toggle_keybinding_help_system,
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_circle_mesh() {
        let mesh = create_circle_mesh(1.0, 32);
        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::LineStrip);

        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Position attribute must exist");

        if let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) = positions {
            assert_eq!(pts.len(), 33);

            for pt in pts {
                let dist = (pt[0] * pt[0] + pt[1] * pt[1]).sqrt();
                assert!((dist - 1.0).abs() < 1e-4);
                assert!(pt[2].abs() < 1e-6);
            }

            assert!((pts[0][0] - pts[32][0]).abs() < 1e-4);
            assert!((pts[0][1] - pts[32][1]).abs() < 1e-4);
        } else {
            panic!("Expected Float32x3 format");
        }
    }

    #[test]
    fn test_create_line_strip_mesh() {
        let points = vec![
            Vec3::ZERO,
            Vec3::new(0.0, 0.0, 1.2),
            Vec3::new(0.1, 0.0, 1.2),
        ];
        let mesh = create_line_strip_mesh(&points);
        assert_eq!(mesh.primitive_topology(), PrimitiveTopology::LineStrip);

        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .expect("Position attribute must exist");

        if let bevy::render::mesh::VertexAttributeValues::Float32x3(pts) = positions {
            assert_eq!(pts.len(), 3);
            for (pt, expected) in pts.iter().zip([[0.0, 0.0, 0.0], [0.0, 0.0, 1.2], [0.1, 0.0, 1.2]]) {
                assert!((pt[0] - expected[0]).abs() < 1e-5);
                assert!((pt[1] - expected[1]).abs() < 1e-5);
                assert!((pt[2] - expected[2]).abs() < 1e-5);
            }
        } else {
            panic!("Expected Float32x3 format");
        }
    }

    #[test]
    fn test_toggle_keybinding_help_system() {
        let mut app = App::new();
        app.init_resource::<ButtonInput<KeyCode>>();

        let entity = app
            .world_mut()
            .spawn((KeybindingHelpModal, Visibility::Hidden))
            .id();

        app.add_systems(Update, toggle_keybinding_help_system);

        // Frame 1: No key pressed -> remains Hidden
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(entity).unwrap(),
            Visibility::Hidden
        );

        // Frame 2: Press KeyH -> becomes Inherited (visible)
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::KeyH);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(entity).unwrap(),
            Visibility::Inherited
        );

        // Frame 3: Press Escape -> becomes Hidden
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .clear();
        app.world_mut()
            .resource_mut::<ButtonInput<KeyCode>>()
            .press(KeyCode::Escape);
        app.update();
        assert_eq!(
            *app.world().get::<Visibility>(entity).unwrap(),
            Visibility::Hidden
        );
    }
}
