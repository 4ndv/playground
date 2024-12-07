use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    utils::{HashMap, HashSet},
};
use bevy_pancam::{PanCam, PanCamPlugin};

const BOARD_WIDTH: usize = 1000;
const BOARD_HEIGHT: usize = 500;

const TILE_SIZE: f32 = 20.0;

const MAX_WIDTH: f32 = BOARD_WIDTH as f32 * TILE_SIZE;
const MAX_HEIGHT: f32 = BOARD_HEIGHT as f32 * TILE_SIZE;

const HALF_MAX_WIDTH: f32 = MAX_WIDTH / 2.0;
const HALF_MAX_HEIGHT: f32 = MAX_HEIGHT / 2.0;

#[derive(Component, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn from_world_coords(point: Vec2) -> Self {
        Self {
            x: (((point.x + HALF_MAX_WIDTH) / TILE_SIZE) as usize).clamp(0, BOARD_WIDTH),
            y: (((-point.y + HALF_MAX_HEIGHT) / TILE_SIZE) as usize).clamp(0, BOARD_HEIGHT),
        }
    }

    pub fn wrap(x: i32, y: i32) -> Self {
        let mut x = x;
        let mut y = y;

        if x < 0 {
            x = BOARD_WIDTH as i32 + x;
        }

        if y < 0 {
            y = BOARD_HEIGHT as i32 + y;
        }

        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    pub fn to_world_coords(&self) -> Vec2 {
        Vec2 {
            x: (self.x as f32 * TILE_SIZE - HALF_MAX_WIDTH + TILE_SIZE * 0.5)
                .clamp(-HALF_MAX_WIDTH, HALF_MAX_WIDTH),
            y: -(self.y as f32 * TILE_SIZE - HALF_MAX_HEIGHT + TILE_SIZE * 0.5)
                .clamp(-HALF_MAX_HEIGHT, HALF_MAX_HEIGHT),
        }
    }

    pub fn to_transform(&self) -> Transform {
        let coords = self.to_world_coords();

        Transform::from_xyz(coords.x, coords.y, 0.)
    }

    pub fn neighbours(&self) -> Vec<Self> {
        let x = self.x as i32;
        let y = self.y as i32;

        vec![
            // Above
            Self::wrap(x - 1, y - 1),
            Self::wrap(x, y - 1),
            Self::wrap(x + 1, y - 1),
            // Around
            Self::wrap(x - 1, y),
            Self::wrap(x + 1, y),
            // Below
            Self::wrap(x - 1, y + 1),
            Self::wrap(x, y + 1),
            Self::wrap(x + 1, y + 1),
        ]
    }
}

#[derive(Event)]
struct ToggleCell(Position);

#[derive(Event)]
struct SpawnCell(Position);

#[derive(Event)]
struct DespawnCell(Position);

#[derive(Event, States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    Paused,
    Running,
}

#[derive(Resource, Default)]
struct Cells {
    index: HashMap<Position, Entity>,
}

impl Cells {
    pub fn at_position(&self, pos: &Position) -> bool {
        self.index.contains_key(pos)
    }

    pub fn alive_neighbours_at_position(&self, pos: Position) -> usize {
        let mut result = 0;

        let x = pos.x as i32;
        let y = pos.y as i32;

        for pos in pos.neighbours() {
            result += self.at_position(&pos) as usize;
        }

        result
    }
}

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

fn main() {
    let window = Some(Window {
        title: "Conway's Game of Life in Bevy".into(),
        ..default()
    });

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: window,
                ..default()
            }),
            PanCamPlugin,
        ))
        .insert_resource(ClearColor(Color::Srgba(Srgba::gray(0.01))))
        .init_state::<GameState>()
        .init_resource::<Cells>()
        .add_event::<ToggleCell>()
        .add_event::<SpawnCell>()
        .add_event::<DespawnCell>()
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .add_systems(
            Update,
            handle_cell_click.run_if(in_state(GameState::Paused)),
        )
        .add_systems(Update, handle_toggle_cell)
        .add_systems(Update, handle_spawn_cell)
        .add_systems(Update, handle_despawn_cell)
        .add_systems(
            Update,
            simulation_tick.run_if(input_just_pressed(KeyCode::Space)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        PanCam {
            min_x: -MAX_WIDTH / 2.0,
            max_x: MAX_WIDTH / 2.0,
            min_y: -MAX_HEIGHT / 2.0,
            max_y: MAX_HEIGHT / 2.0,
            max_scale: 3.0,
            min_scale: 1.0,
            grab_buttons: vec![MouseButton::Right],
            ..default()
        },
    ));
}

fn draw_grid(mut gizmos: Gizmos, query: Query<&OrthographicProjection>) {
    let Ok(proj) = query.get_single() else {
        return;
    };

    let mut intensity = 0.015;

    if proj.scale > 2.2 {
        intensity = 0.005;
    } else if proj.scale > 1.5 {
        intensity = 0.01;
    }

    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(BOARD_WIDTH as u32, BOARD_HEIGHT as u32),
            Vec2::new(TILE_SIZE, TILE_SIZE),
            LinearRgba::gray(intensity),
        )
        .outer_edges();
}

fn handle_cell_click(
    mut ew_toggle_cell: EventWriter<ToggleCell>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
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

    // TODO: Return specific entity id
    ew_toggle_cell.send(ToggleCell(pos));
}

// fn spawn_on_click(mut commands: Commands, mut ev_cell_clicked: EventReader<ToggleCell>) {
//     for event in ev_cell_clicked.read() {
//         let transform = event.0.to_transform();

//         commands.spawn(CellBundle {
//             sprite: Sprite {
//                 color: Color::WHITE,
//                 custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
//                 ..default()
//             },
//             transform,
//         });
//     }
// }

fn handle_spawn_cell(
    mut commands: Commands,
    mut ev_spawn_cell: EventReader<SpawnCell>,
    mut cells: ResMut<Cells>,
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
    mut cells: ResMut<Cells>,
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
    cells: Res<Cells>,
) {
    for event in ev_toggle_cell.read() {
        let pos = event.0;

        match cells.at_position(&pos) {
            true => {
                ew_despawn_cell.send(DespawnCell(pos));
            }
            false => {
                ew_spawn_cell.send(SpawnCell(pos));
            }
        };
    }
}

fn simulation_tick(
    cells: Res<Cells>,
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
        let alive = cells.at_position(&pos);

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
