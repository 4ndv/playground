use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::simulation::{ChangeSimulationSpeed, ResetSimulation, Simulation, SimulationState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, simulation_window);
    }
}

fn simulation_window(
    mut ew_reset_simulation: EventWriter<ResetSimulation>,
    mut ew_change_simulation_speed: EventWriter<ChangeSimulationSpeed>,
    mut contexts: EguiContexts,
    diag: Res<DiagnosticsStore>,
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    sim: Res<Simulation>,
) {
    let fps = diag
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(0.0);

    let frame_time = diag
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(0.0);

    egui::Window::new("Simulation").show(contexts.ctx_mut(), |ui| {
        ui.label(format!("FPS: {:.2}", fps));
        ui.label(format!("Frame Time: {:.2}ms", frame_time));
        ui.label(format!("Generation: {}", sim.generation));
        ui.label(format!("Population: {}", sim.population));

        ui.horizontal(|ui| {
            let is_running = state.get() == &SimulationState::Running;

            let run_button = egui::Button::new("Run");
            let pause_button = egui::Button::new("Pause");
            let reset_button = egui::Button::new("Reset");

            if ui.add_enabled(!is_running, run_button).clicked() {
                next_state.set(SimulationState::Running);
            }

            if ui.add_enabled(is_running, pause_button).clicked() {
                next_state.set(SimulationState::Paused);
            }

            if ui.add_enabled(!is_running, reset_button).clicked() {
                ew_reset_simulation.send(ResetSimulation);
            }
        });

        let mut speed = sim.speed;

        ui.add(
            egui::Slider::new(&mut speed, 1.0..=200.0)
                .step_by(1.0)
                .suffix("Hz"),
        );

        if speed != sim.speed {
            ew_change_simulation_speed.send(ChangeSimulationSpeed(speed));
        }
    });
}
