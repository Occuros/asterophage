use bevy::prelude::*;
use std::fmt::{Debug, Formatter};

use bevy::utils::HashMap;

#[derive(Component, Reflect, Hash, Eq, PartialEq, Debug, Clone, Default, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect)]
pub enum GroundLayer {
    #[default]
    Empty,
    Blood {
        amount: i32
    },
    YellowBile {
        amount: i32,
    },
    BlackBile {
        amount: i32,
    },
    Phlegm {
        amount: i32,
    },
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect)]
pub enum SurfaceLayer {
    #[default]
    Empty,
    CiliaBelt {
        entity: Entity
    },
    Building {
        entity: Entity
    }
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect)]
pub struct  Cell {
    pub ground_layer: GroundLayer,
    pub surface_layer: SurfaceLayer,
}

// impl Debug for Cell {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Cell::EmptyCell => {
//                 write!(f, "()")
//             }
//             Cell::DebugCell { text } => {
//                 write!(f, "{}", text)
//             }
//             Cell::IntCell { number } => {
//                 write!(f, "{}", number)
//             }
//             Cell::BuildingCell {} => write!(f, "x"),
//             Cell::ResourceCell {} => write!(f, "o"),
//         }
//     }
// }

#[derive(Resource)]
pub struct GridCursor {
    pub entity: Entity,
    pub ui_position: Option<Vec2>,
    pub selected_cell: Option<Cell>,
    pub world_position: Option<Vec3>,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct WorldGrid {
    pub grid_size: f32,
    pub cells: HashMap<GridPosition, Cell>,
}

impl WorldGrid {
    pub fn new(grid_size: f32) -> WorldGrid {
        let cells = HashMap::<GridPosition, Cell>::new();
        Self {
            cells,
            grid_size,
        }
    }

    pub fn grid_to_world(&self, grid_position: &GridPosition) -> Vec3 {
        Vec3::new(
            grid_position.x as f32 * self.grid_size,
            0.0,
            grid_position.y as f32 * self.grid_size,
        )
    }

    pub fn set_cell(&mut self, cell: Cell, position: GridPosition) {
        self.cells.insert(position, cell);
    }

    pub fn set_cell_at_world_position(&mut self, position: Vec3, cell: Cell) {
        let grid_position = self.get_grid_position_from_world_position(position);
        self.cells.insert(grid_position, cell);
    }

    pub fn get_grid_position_from_world_position(&self, position: Vec3) -> GridPosition {
        let x = ((position.x + self.grid_size * 0.5) / self.grid_size).floor() as i32;
        let y = ((position.z + self.grid_size * 0.5) / self.grid_size).floor() as i32;
        GridPosition { x, y }
    }
}