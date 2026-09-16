use bevy::prelude::*;
use bevy::render::camera::Viewport;

#[derive(Component)]
pub struct MainViewPanelMarker;

#[derive(Component)]
pub struct GlobeCameraMarker;

#[allow(
    clippy::needless_pass_by_value,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn sync_globe_viewport(
    window_query: Query<&Window>,
    panel_query: Query<(&GlobalTransform, &ComputedNode), With<MainViewPanelMarker>>,
    mut camera_query: Query<&mut Camera, With<GlobeCameraMarker>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let Ok((transform, node)) = panel_query.get_single() else {
        return;
    };
    let Ok(mut camera) = camera_query.get_single_mut() else {
        return;
    };

    let win_w = window.physical_width();
    let win_h = window.physical_height();

    if win_w == 0 || win_h == 0 {
        return;
    }

    let size = node.size();
    if size.x <= 0.0 || size.y <= 0.0 {
        return;
    }

    // In Bevy UI, GlobalTransform translation is located at (node_left + size.x / 2, node_top + size.y / 2)
    let center = transform.translation().truncate();
    let top_left = center - size / 2.0;

    let x = (top_left.x.max(0.0)).round() as u32;
    let y = (top_left.y.max(0.0)).round() as u32;
    let w = size.x.round() as u32;
    let h = size.y.round() as u32;

    if x >= win_w || y >= win_h {
        return;
    }

    let max_w = win_w - x;
    let max_h = win_h - y;

    let physical_w = w.min(max_w);
    let physical_h = h.min(max_h);

    if physical_w > 0 && physical_h > 0 {
        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(x, y),
            physical_size: UVec2::new(physical_w, physical_h),
            depth: 0.0..1.0,
        });
    }
}
