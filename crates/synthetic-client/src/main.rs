mod globe;
mod systems;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ClearColorConfig;
use globe::viewport::MainViewPanelMarker;
use globe::GlobePlugin;
use systems::{AstronomyPlugin, TimeWarpPlugin, UiPlugin};

#[derive(Component)]
pub struct UiCameraMarker;

#[derive(Component)]
struct FpsText;

#[derive(Resource, Default)]
pub struct TaskProfiler {
    pub tasks: std::collections::HashMap<&'static str, Vec<f64>>,
}

impl TaskProfiler {
    pub fn record_task(&mut self, name: &'static str, duration_ms: f64) {
        let history = self.tasks.entry(name).or_insert_with(|| Vec::with_capacity(60));
        history.push(duration_ms);
        if history.len() > 60 {
            history.remove(0);
        }
    }

    pub fn get_averages(&self) -> Vec<(&'static str, f64)> {
        let mut avgs = Vec::new();
        for (name, history) in &self.tasks {
            let sum: f64 = history.iter().sum();
            let avg = if history.is_empty() { 0.0 } else { sum / history.len() as f64 };
            avgs.push((*name, avg));
        }
        avgs.sort_by(|a, b| a.0.cmp(b.0));
        avgs
    }
}

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
        .add_plugins(TimeWarpPlugin)
        .add_plugins(UiPlugin)
        .init_resource::<TaskProfiler>()
        .add_systems(Startup, setup_ui)
        .add_systems(
            Update,
            update_fps_text.run_if(bevy::time::common_conditions::on_timer(
                std::time::Duration::from_millis(250),
            )),
        )
        .run();
}

pub fn setup_ui(mut commands: Commands) {
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

    // FPS counter in top-left corner
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
    profiler: Res<TaskProfiler>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        let mut display_str = String::new();
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                display_str.push_str(&format!("FPS: {value:.0}"));
            }
        }
        
        for (name, avg_ms) in profiler.get_averages() {
            display_str.push_str(&format!("\n{name}: {avg_ms:.2} ms"));
        }
        
        if !display_str.is_empty() {
            **text = display_str;
        }
    }
}

