use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, simulation_window);
    }
}

fn simulation_window(mut contexts: EguiContexts, diag: Res<DiagnosticsStore>) {
    let fps = diag
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(0.0);

    egui::Window::new("Simulation").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", fps))
    });
}
