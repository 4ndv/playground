use crate::consts::*;
use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct GridMaterial {
    #[uniform(0)]
    tile_size: f32,
    #[uniform(1)]
    half_max_width: f32,
    #[uniform(2)]
    half_max_height: f32,
    alpha_mode: AlphaMode2d,
}

impl Material2d for GridMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/grid.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        self.alpha_mode
    }
}

impl Default for GridMaterial {
    fn default() -> Self {
        Self {
            tile_size: TILE_SIZE,
            alpha_mode: AlphaMode2d::Blend,
            half_max_width: HALF_MAX_WIDTH,
            half_max_height: HALF_MAX_HEIGHT,
        }
    }
}

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle {
            half_size: Vec2::new(HALF_MAX_WIDTH, HALF_MAX_HEIGHT),
        })),
        MeshMaterial2d(materials.add(GridMaterial::default())),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<GridMaterial>::default())
            .add_systems(Startup, setup_grid);
    }
}
