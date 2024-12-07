use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_egui::EguiContexts;
use bevy_pancam::{PanCam, PanCamPlugin};

mod consts;
mod grid;
mod position;
mod simulation;
mod ui;

use consts::*;
use grid::*;
use position::*;
use simulation::{SimulationPlugin, SimulationState, ToggleCell};
use ui::*;

fn main() {
    let window = Some(Window {
        title: "Conway's Game of Life in Bevy".into(),
        ..default()
    });

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: window,
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
            PanCamPlugin,
            GridPlugin,
            SimulationPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            UiPlugin,
        ))
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.01))))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            handle_cell_click.run_if(in_state(SimulationState::Paused)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCam {
            min_x: -HALF_MAX_WIDTH,
            max_x: HALF_MAX_WIDTH,
            min_y: -HALF_MAX_HEIGHT,
            max_y: HALF_MAX_HEIGHT,
            max_scale: 3.0,
            min_scale: 1.0,
            grab_buttons: vec![MouseButton::Right],
            ..default()
        },
    ));
}

fn handle_cell_click(
    mut ew_toggle_cell: EventWriter<ToggleCell>,
    mut egui_ctx: EguiContexts,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if egui_ctx.ctx_mut().wants_pointer_input() {
        return;
    }

    let (camera, camera_transform) = *camera_query;

    let Ok(window) = windows.get_single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    if point.x > HALF_MAX_WIDTH
        || point.x < -HALF_MAX_WIDTH
        || point.y > HALF_MAX_HEIGHT
        || point.y < -HALF_MAX_HEIGHT
    {
        return;
    }

    let pos = Position::from_world_coords(point);

    ew_toggle_cell.send(ToggleCell(pos));
}
