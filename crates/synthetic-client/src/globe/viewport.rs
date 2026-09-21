use bevy::prelude::*;

#[derive(Component)]
pub struct MainViewPanelMarker;

#[derive(Component)]
pub struct GlobeCameraMarker;

/// Ensures the 3D globe camera renders across the full window without subpanel constraints.
#[allow(clippy::needless_pass_by_value)]
pub fn sync_globe_viewport(
    mut camera_query: Query<&mut Camera, With<GlobeCameraMarker>>,
) {
    for mut camera in &mut camera_query {
        if camera.viewport.is_some() {
            camera.viewport = None;
        }
    }
}
