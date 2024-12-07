use crate::consts::*;
use bevy::prelude::*;

#[derive(Component, Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Position {
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

        x = x % BOARD_WIDTH as i32;
        y = y % BOARD_HEIGHT as i32;

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
