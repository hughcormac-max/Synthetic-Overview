mod globe;
mod systems;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use globe::viewport::MainViewPanelMarker;
use globe::GlobePlugin;
use systems::AstronomyPlugin;

#[derive(Component)]
pub struct UiCameraMarker;

#[derive(Component)]
struct FpsText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Synthetic Overview".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(GlobePlugin)
        .add_plugins(AstronomyPlugin)
        .add_systems(Startup, setup_ui)
        .add_systems(Update, update_fps_text)
        .run();
}

fn setup_ui(mut commands: Commands) {
    let ui_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            UiCameraMarker,
        ))
        .id();

    // Single full-window UI viewport
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        TargetCamera(ui_camera),
        BackgroundColor(Color::NONE),
        Interaction::default(),
        MainViewPanelMarker,
    ));

    // FPS counter in top-right corner
    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.85, 0.85)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            right: Val::Px(16.0),
            ..default()
        },
        TargetCamera(ui_camera),
        FpsText,
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                **text = format!("FPS: {value:.0}");
            }
        }
    }
}

