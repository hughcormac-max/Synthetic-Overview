use bevy::prelude::*;

const BORDER_COLOR: Color = Color::srgb(0.22, 0.24, 0.28);
const TEXT_COLOR: Color = Color::srgb(0.85, 0.88, 0.92);
const SIDEBAR_BG: Color = Color::srgb(0.12, 0.12, 0.14);
const MAIN_VIEW_BG: Color = Color::srgb(0.08, 0.08, 0.10);
const BOTTOM_BAR_BG: Color = Color::srgb(0.14, 0.14, 0.16);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Synthetic Overview".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_ui)
        .run();
}

fn spawn_label(parent: &mut ChildBuilder, text: &str, font_size: f32) {
    parent.spawn((
        Text::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(TEXT_COLOR),
    ));
}

fn spawn_sidebar(parent: &mut ChildBuilder, title: &str) {
    parent
        .spawn((
            Node {
                width: Val::Percent(20.0),
                height: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(SIDEBAR_BG),
            BorderColor(BORDER_COLOR),
        ))
        .with_children(|panel| {
            spawn_label(panel, title, 14.0);
        });
}

fn spawn_view_panel(
    parent: &mut ChildBuilder,
    title: &str,
    height_pct: f32,
    bg: Color,
    font_size: f32,
) {
    parent
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(height_pct),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor(BORDER_COLOR),
        ))
        .with_children(|panel| {
            spawn_label(panel, title, font_size);
        });
}

fn spawn_center_column(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.06, 0.08)),
        ))
        .with_children(|center| {
            spawn_view_panel(center, "2: Main View", 70.0, MAIN_VIEW_BG, 16.0);
            spawn_view_panel(center, "3: Bottom Bar", 30.0, BOTTOM_BAR_BG, 14.0);
        });
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
        ))
        .with_children(|root| {
            spawn_sidebar(root, "1: Left Sidebar");
            spawn_center_column(root);
            spawn_sidebar(root, "4: Right Sidebar");
        });
}
