use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_just_pressed,
    prelude::*,
    utils::{HashMap, HashSet},
};

use crate::consts::*;
use crate::position::*;

#[derive(Component)]
#[require(Position, Sprite, Transform)]
struct Cell;

#[derive(Bundle)]
struct CellBundle {
    cell: Cell,
    sprite: Sprite,
    transform: Transform,
    position: Position,
}

#[derive(Event, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}

#[derive(Resource, Default)]
pub struct Simulation {
    index: HashMap<Position, Entity>,
}

impl Simulation {
    pub fn state_at_position(&self, pos: &Position) -> bool {
        self.index.contains_key(pos)
    }

    pub fn alive_neighbours_at_position(&self, pos: Position) -> usize {
        let mut result = 0;

        for pos in pos.neighbours() {
            result += self.state_at_position(&pos) as usize;
        }

        result
    }
}

fn simulation_tick(
    cells: Res<Simulation>,
    mut ew_spawn_cell: EventWriter<SpawnCell>,
    mut ew_despawn_cell: EventWriter<DespawnCell>,
) {
    let mut damaged = HashSet::new();
    let mut spawn = HashSet::new();
    let mut yeet = HashSet::new();

    for pos in cells.index.keys() {
        damaged.extend(pos.neighbours());
        damaged.insert(*pos);
    }

    for pos in damaged {
        let alive = cells.state_at_position(&pos);

        let count = cells.alive_neighbours_at_position(pos);

        match (alive, count) {
            (true, ..2) | (true, 4..) => {
                yeet.insert(pos);
            }
            (false, 3) => {
                spawn.insert(pos);
            }
            _ => {}
        }
    }

    for pos in spawn {
        ew_spawn_cell.send(SpawnCell(pos));
    }

    for pos in yeet {
        ew_despawn_cell.send(DespawnCell(pos));
    }
}

#[derive(Event)]
pub struct ToggleCell(pub Position);

#[derive(Event)]
pub struct SpawnCell(pub Position);

#[derive(Event)]
pub struct DespawnCell(pub Position);

fn handle_spawn_cell(
    mut commands: Commands,
    mut ev_spawn_cell: EventReader<SpawnCell>,
    mut cells: ResMut<Simulation>,
) {
    for event in ev_spawn_cell.read() {
        let pos = event.0;

        let cell = commands
            .spawn(CellBundle {
                cell: Cell {},
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: pos.to_transform(),
                position: pos,
            })
            .id();

        cells.index.insert(pos, cell);
    }
}

fn handle_despawn_cell(
    mut commands: Commands,
    mut ev_spawn_cell: EventReader<DespawnCell>,
    mut cells: ResMut<Simulation>,
) {
    for event in ev_spawn_cell.read() {
        let pos = event.0;

        let cell = cells.index.get(&pos).unwrap();

        commands.entity(*cell).despawn();
        cells.index.remove(&pos);
    }
}

fn handle_toggle_cell(
    mut ev_toggle_cell: EventReader<ToggleCell>,
    mut ew_spawn_cell: EventWriter<SpawnCell>,
    mut ew_despawn_cell: EventWriter<DespawnCell>,
    cells: Res<Simulation>,
) {
    for event in ev_toggle_cell.read() {
        let pos = event.0;

        match cells.state_at_position(&pos) {
            true => {
                ew_despawn_cell.send(DespawnCell(pos));
            }
            false => {
                ew_spawn_cell.send(SpawnCell(pos));
            }
        };
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<Simulation>()
            .add_event::<ToggleCell>()
            .add_event::<SpawnCell>()
            .add_event::<DespawnCell>()
            .add_systems(Update, handle_toggle_cell)
            .add_systems(Update, handle_spawn_cell)
            .add_systems(Update, handle_despawn_cell)
            .add_systems(
                Update,
                simulation_tick.run_if(input_just_pressed(KeyCode::Space)),
            );
    }
}