mod camera;
mod game_logic;
mod input;
mod render;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin) // Plugin pour les menus
        // Resources
        .init_resource::<game_logic::Universe>()
        .init_resource::<game_logic::GameSettings>()
        // Systems
        .add_systems(Startup, camera::setup_camera)
        .add_systems(
            Update,
            (
                input::handle_input,
                camera::zoom_system,
                game_logic::evolution_system,
                render::render_system,
                ui::ui_controls,
            ),
        )
        .run();
}
