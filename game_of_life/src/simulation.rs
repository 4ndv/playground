use bevy::{
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

#[derive(Resource)]
pub struct Simulation {
    index: HashMap<Position, Entity>,
    pub population: usize,
    pub generation: usize,
    pub speed: f64,
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

impl Default for Simulation {
    fn default() -> Self {
        Self {
            index: HashMap::<Position, Entity>::new(),
            population: 0,
            generation: 0,
            speed: DEFAULT_SIMULATION_SPEED_HZ,
        }
    }
}

fn simulation_tick(mut commands: Commands, mut sim: ResMut<Simulation>) {
    let mut damaged = HashSet::new();
    let mut spawn = HashSet::new();
    let mut yeet = HashSet::new();

    damaged.extend(sim.index.keys());

    for pos in sim.index.keys() {
        damaged.extend(pos.neighbours());
    }

    for pos in damaged {
        let alive = sim.state_at_position(&pos);

        let count = sim.alive_neighbours_at_position(pos);

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
        spawn_cell_at(pos, &mut commands, &mut sim);
    }

    for pos in yeet {
        despawn_cell_at(pos, &mut commands, &mut sim);
    }

    sim.generation += 1;
}

#[derive(Event)]
pub struct ToggleCell(pub Position);

#[derive(Event)]
pub struct ResetSimulation;

#[derive(Event)]
pub struct ChangeSimulationSpeed(pub f64);

fn spawn_cell_at(pos: Position, commands: &mut Commands, cells: &mut ResMut<Simulation>) {
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
    cells.population += 1;
}

fn despawn_cell_at(pos: Position, commands: &mut Commands, sim: &mut ResMut<Simulation>) {
    let cell = sim.index.get(&pos).unwrap_or_else(|| {
        panic!("Cell wasn't found in index at pos {:?}", pos);
    });

    commands.entity(*cell).despawn();

    sim.index.remove(&pos);
    sim.population -= 1;
}

fn handle_toggle_cell(
    mut ev_toggle_cell: EventReader<ToggleCell>,
    mut commands: Commands,
    mut sim: ResMut<Simulation>,
) {
    for event in ev_toggle_cell.read() {
        let pos = event.0;

        match sim.state_at_position(&pos) {
            true => {
                despawn_cell_at(pos, &mut commands, &mut sim);
            }
            false => {
                spawn_cell_at(pos, &mut commands, &mut sim);
            }
        };
    }
}

fn handle_reset_simulation(
    mut commands: Commands,
    mut ev_reset_simulation: EventReader<ResetSimulation>,
    mut sim: ResMut<Simulation>,
) {
    for _ in ev_reset_simulation.read() {
        reset_simulation(&mut commands, &mut sim);
    }
}

fn reset_simulation(commands: &mut Commands, sim: &mut ResMut<Simulation>) {
    sim.index
        .iter()
        .for_each(|(_, c)| commands.entity(*c).despawn());

    sim.index.clear();
    sim.population = 0;
    sim.generation = 0;
}

fn handle_randomize(mut commands: Commands, mut sim: ResMut<Simulation>) {
    reset_simulation(&mut commands, &mut sim);

    let mut alive = HashSet::new();

    for x in 0..BOARD_WIDTH {
        for y in 0..BOARD_HEIGHT {
            if rand::random::<f32>() > RANDOMIZE_THRESHOLD {
                alive.insert(Position { x, y });
            }
        }
    }

    for pos in alive {
        spawn_cell_at(pos, &mut commands, &mut sim);
    }
}

fn handle_change_simulation_speed(
    mut commands: Commands,
    mut ev_change_simulation_speed: EventReader<ChangeSimulationSpeed>,
    mut sim: ResMut<Simulation>,
) {
    for event in ev_change_simulation_speed.read() {
        sim.speed = event.0;

        commands.insert_resource(Time::<Fixed>::from_hz(event.0));
    }
}

fn handle_toggle_state(
    state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if *state.get() == SimulationState::Running {
        next_state.set(SimulationState::Paused)
    } else {
        next_state.set(SimulationState::Running)
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<Simulation>()
            .add_event::<ToggleCell>()
            .add_event::<ResetSimulation>()
            .add_event::<ChangeSimulationSpeed>()
            .add_systems(PostUpdate, handle_toggle_cell)
            .add_systems(PostUpdate, handle_reset_simulation)
            .add_systems(PostUpdate, handle_change_simulation_speed)
            .add_systems(
                PostUpdate,
                handle_toggle_state.run_if(input_just_pressed(KeyCode::Space)),
            )
            .add_systems(
                Update,
                simulation_tick.run_if(
                    input_just_pressed(KeyCode::Enter).and(in_state(SimulationState::Paused)),
                ),
            )
            .add_systems(
                PostUpdate,
                handle_randomize.run_if(
                    input_just_pressed(KeyCode::KeyR).and(in_state(SimulationState::Paused)),
                ),
            )
            .add_systems(
                FixedUpdate,
                simulation_tick.run_if(in_state(SimulationState::Running)),
            );
    }
}
