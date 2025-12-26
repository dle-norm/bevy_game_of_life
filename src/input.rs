use crate::game_logic::{GameSettings, Universe, CELL_SIZE};
use bevy::prelude::*;

pub fn handle_input(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<GameSettings>,
    mut universe: ResMut<Universe>,
    q_windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        settings.paused = !settings.paused;
    }
    if keys.just_pressed(KeyCode::KeyR) {
        universe.cells.clear();
    }

    if buttons.pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
        let window = q_windows.single();

        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
        {
            let grid_pos = (
                (world_pos.x / CELL_SIZE).round() as i32,
                (world_pos.y / CELL_SIZE).round() as i32,
            );
            universe.cells.insert(grid_pos);
        }
    }
}
