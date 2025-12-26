use crate::game_logic::{Universe, CELL_SIZE};
use bevy::prelude::*;

#[derive(Component)]
pub struct CellVisual;

pub fn render_system(
    mut commands: Commands,
    universe: Res<Universe>,
    query: Query<Entity, With<CellVisual>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    for &(x, y) in &universe.cells {
        commands.spawn((
            Sprite {
                color: Color::srgb(0.2, 0.8, 0.2),
                custom_size: Some(Vec2::splat(CELL_SIZE - 1.0)),
                ..default()
            },
            Transform::from_xyz(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, 0.0),
            CellVisual,
        ));
    }
}
