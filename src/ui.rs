use crate::game_logic::{GameSettings, Universe};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn ui_controls(
    mut contexts: EguiContexts,
    mut settings: ResMut<GameSettings>,
    mut universe: ResMut<Universe>,
) {
    // 1. Store the value in a local variable to avoid
    // borrowing from 'settings' inside the closure
    let mut current_speed = settings.speed;

    egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        ui.label("Space: Play/Pause | R: Clear");

        if ui
            .button(if settings.paused {
                "▶ Play"
            } else {
                "⏸ Pause"
            })
            .clicked()
        {
            settings.paused = !settings.paused;
        }

        if ui.button("Clear Grid").clicked() {
            universe.cells.clear();
        }

        // 2. Use the local variable for the slider
        if ui
            .add(egui::Slider::new(&mut current_speed, 0.01..=1.0).text("Update Interval"))
            .changed()
        {
            // 3. Apply the change back to the resource if it changed
            settings.speed = current_speed;
            settings
                .timer
                .set_duration(std::time::Duration::from_secs_f32(current_speed));
        }
    });
}
