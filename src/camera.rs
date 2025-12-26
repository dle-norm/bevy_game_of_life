use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn zoom_system(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut projection = query.single_mut();
    for event in mouse_wheel_events.read() {
        // Zoom factor: 0.9 (in) or 1.1 (out)
        let factor = if event.y > 0.0 { 0.9 } else { 1.1 };
        projection.scale *= factor;
        projection.scale = projection.scale.clamp(0.1, 10.0);
    }
}
