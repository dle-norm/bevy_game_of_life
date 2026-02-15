mod camera;
mod game_logic;
mod input;
mod render;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

/// Sets up the Bevy application.
pub fn app() -> App {
    let mut app = App::new();

    // Ajoute les plugins par défaut de Bevy.
    let mut window_plugin = WindowPlugin {
        primary_window: Some(Window { ..default() }),
        ..default()
    };

    // Attache l'application au canvas HTML uniquement pour les builds web.
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = &mut window_plugin.primary_window {
        window.canvas = Some("#bevy".to_string());
    }

    app.add_plugins(DefaultPlugins.set(window_plugin));
    app.add_plugins(EguiPlugin)
        .init_resource::<game_logic::Universe>()
        .init_resource::<game_logic::GameSettings>()
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
        );
    app
}

#[wasm_bindgen]
pub fn run() {
    app().run();
}