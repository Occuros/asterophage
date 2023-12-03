use bevy::prelude::*;
use std::fmt::{Debug, Formatter};

use bevy::utils::HashMap;

#[derive(Component, Reflect, Hash, Eq, PartialEq, Debug, Clone, Default, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Reflect, Hash, Eq, PartialEq, Default, Clone)]
pub enum Cell {
    #[default]
    EmptyCell,
    DebugCell {
        text: String,
    },
    IntCell {
        number: i32,
    },
    BuildingCell {},
    ResourceCell {}
}

impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::EmptyCell => {
                write!(f, "()")
            }
            Cell::DebugCell { text } => {
                write!(f, "{}", text)
            }
            Cell::IntCell { number } => {
                write!(f, "{}", number)
            }
            Cell::BuildingCell {} => write!(f, "x"),
            Cell::ResourceCell {} => write!(f, "o"),
        }
    }
}

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
    pub width: i32,
    pub height: i32,
    pub grid_size: f32,
    pub cells: HashMap<GridPosition, Cell>,
}

#[derive(Component)]
pub struct GridChunk {
    pub origin: Vec3,
    pub width: i32,
    pub height: i32,
    pub cell_size: f32,
    pub cells: HashMap<GridPosition, Cell>,
}

impl WorldGrid {
    pub fn new(width: i32, height: i32, grid_size: f32) -> WorldGrid {
        let mut cells = HashMap::<GridPosition, Cell>::new();
        for x in 0..width {
            for y in 0..height {
                let position = GridPosition { x, y };
                cells.insert(position.clone(), Cell::EmptyCell {});
            }
        }
        Self {
            width,
            height,
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

impl<'a> IntoIterator for &'a WorldGrid {
    type Item = (GridPosition, &'a Cell);
    type IntoIter = WorldGridIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        WorldGridIterator {
            world_grid: self,
            position: GridPosition { x: 0, y: 0 },
        }
    }
}

pub struct WorldGridIterator<'a> {
    world_grid: &'a WorldGrid,
    position: GridPosition,
}

impl<'a> Iterator for WorldGridIterator<'a> {
    type Item = (GridPosition, &'a Cell);
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.world_grid.cells.get(&self.position);
        let position = self.position;

        self.position.x += 1;

        if self.position.x >= self.world_grid.width {
            self.position.y += 1;
            self.position.x = 0;
        }

        result.map(move |cell| (position, cell))
    }
}

#[allow(dead_code)]
impl GridChunk {
    pub fn new(origin: Vec3, width: i32, height: i32, cell_size: f32) -> Self {
        let mut cells = HashMap::<GridPosition, Cell>::new();
        for x in 0..width {
            for y in 0..height {
                let position = GridPosition { x, y };
                cells.insert(position, Cell::IntCell { number: x + y });
            }
        }
        Self {
            origin,
            width,
            height,
            cells,
            cell_size,
        }
    }

    pub fn grid_to_world(&self, grid_position: &GridPosition) -> Vec3 {
        Vec3::new(
            grid_position.x as f32 * self.cell_size,
            0.0,
            grid_position.y as f32 * self.cell_size,
        ) + self.origin
    }

    pub fn set_cell(&mut self, cell: Cell, position: GridPosition) {
        self.cells.insert(position, cell);
    }
    pub fn get_cell(&self, position: &GridPosition) -> Option<&Cell> {
        self.cells.get(position)
    }
    pub fn get_cell_mut(&mut self, position: &GridPosition) -> Option<&mut Cell> {
        self.cells.get_mut(position)
    }

    pub fn get_cell_from_world(&self, position: Vec3) -> Option<&Cell> {
        let grid_position = self.get_grid_position_from_world_position(position);
        self.cells.get(&grid_position)
    }

    pub fn get_cell_mut_from_world(&mut self, position: Vec3) -> Option<&mut Cell> {
        let grid_position = self.get_grid_position_from_world_position(position);
        self.cells.get_mut(&grid_position)
    }

    pub fn set_cell_at_world_position(&mut self, position: Vec3, cell: Cell) {
        let grid_position = self.get_grid_position_from_world_position(position);
        self.cells.insert(grid_position, cell);
    }

    pub fn get_grid_position_from_world_position(&self, position: Vec3) -> GridPosition {
        let x = ((position.x + self.cell_size * 0.5) / self.cell_size).floor() as i32;
        let y = ((position.z + self.cell_size * 0.5) / self.cell_size).floor() as i32;
        GridPosition { x, y }
    }
}
