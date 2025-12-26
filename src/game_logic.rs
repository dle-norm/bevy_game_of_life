use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub const CELL_SIZE: f32 = 10.0;
pub const DEFAULT_UPDATE_TIME: f32 = 0.1;

#[derive(Resource, Default)]
pub struct Universe {
    pub cells: HashSet<(i32, i32)>,
}

#[derive(Resource)]
pub struct GameSettings {
    pub paused: bool,
    pub speed: f32,
    pub timer: Timer,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            paused: true,
            speed: DEFAULT_UPDATE_TIME,
            timer: Timer::from_seconds(DEFAULT_UPDATE_TIME, TimerMode::Repeating),
        }
    }
}

pub fn evolution_system(
    time: Res<Time>,
    mut settings: ResMut<GameSettings>,
    mut universe: ResMut<Universe>,
) {
    if settings.paused {
        return;
    }

    settings.timer.tick(time.delta());
    if !settings.timer.finished() {
        return;
    }

    let mut neighbor_counts = HashMap::new();
    for &(x, y) in &universe.cells {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                *neighbor_counts.entry((x + dx, y + dy)).or_insert(0) += 1;
            }
        }
    }

    let mut next_gen = HashSet::default();
    for (pos, count) in neighbor_counts {
        let is_alive = universe.cells.contains(&pos);
        if count == 3 || (count == 2 && is_alive) {
            next_gen.insert(pos);
        }
    }
    universe.cells = next_gen;
}
