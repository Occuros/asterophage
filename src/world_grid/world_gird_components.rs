use bevy::{prelude::*, utils::HashMap};
use std::{f32::consts::TAU, f32::consts::PI, ops};
use crate::building::building_components::Building;

#[derive(Component, Reflect, Hash, Eq, PartialEq, Debug, Clone, Default, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    #[allow(dead_code)]
    pub fn get_neighbour(&self, direction: GridRotation) -> GridPosition {
        match direction {
            GridRotation::N => GridPosition { x: self.x, y: self.y + 1 },
            GridRotation::S => GridPosition { x: self.x, y: self.y - 1 },
            GridRotation::W => GridPosition { x: self.x - 1, y: self.y },
            GridRotation::E => GridPosition { x: self.x + 1, y: self.y },
        }
    }

    pub fn get_all_surrounding_positions(&self) -> Vec<GridPosition> {
        vec![self.get_neighbour(GridRotation::N),
             self.get_neighbour(GridRotation::E),
             self.get_neighbour(GridRotation::S),
             self.get_neighbour(GridRotation::W)]
    }

    pub fn get_relative_forward(&self, rotation: GridRotation) -> GridPosition {
        match rotation {
            GridRotation::N => GridPosition { x: self.x, y: self.y + 1 },
            GridRotation::S => GridPosition { x: self.x, y: self.y - 1 },
            GridRotation::W => GridPosition { x: self.x + 1, y: self.y },
            GridRotation::E => GridPosition { x: self.x - 1, y: self.y },
        }
    }

    #[allow(dead_code)]
    pub fn get_relative_back(&self, rotation: GridRotation) -> GridPosition {
        match rotation {
            GridRotation::N => GridPosition { x: self.x, y: self.y - 1 },
            GridRotation::S => GridPosition { x: self.x, y: self.y + 1 },
            GridRotation::W => GridPosition { x: self.x - 1, y: self.y },
            GridRotation::E => GridPosition { x: self.x + 1, y: self.y },
        }
    }
    #[allow(dead_code)]
    pub fn get_relative_left(&self, rotation: GridRotation) -> GridPosition {
        match rotation {
            GridRotation::N => GridPosition { x: self.x + 1, y: self.y },
            GridRotation::S => GridPosition { x: self.x - 1, y: self.y },
            GridRotation::W => GridPosition { x: self.x, y: self.y - 1 },
            GridRotation::E => GridPosition { x: self.x, y: self.y + 1 },
        }
    }

    #[allow(dead_code)]
    pub fn get_relative_right(&self, rotation: GridRotation) -> GridPosition {
        match rotation {
            GridRotation::N => GridPosition { x: self.x - 1, y: self.y },
            GridRotation::S => GridPosition { x: self.x + 1, y: self.y },
            GridRotation::W => GridPosition { x: self.x, y: self.y + 1 },
            GridRotation::E => GridPosition { x: self.x, y: self.y - 1 },
        }
    }
}

impl ops::Add<GridPosition> for GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: GridPosition) -> Self::Output {
        GridPosition {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub enum GroundLayerType {
    #[default]
    Empty,
    BloodResource,
    YellowBileResource,
    BlackBileResource,
    PhlegmResource,
}


#[derive(Reflect, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridRotation {
    #[default]
    N,
    S,
    W,
    E,
}

impl GridRotation {
    pub fn difference(&self, other: GridRotation) -> i32 {
        match (self, other) {
            (GridRotation::N, GridRotation::N) => 0,
            (GridRotation::N, GridRotation::S) => 2,
            (GridRotation::N, GridRotation::W) => 1,
            (GridRotation::N, GridRotation::E) => 1,
            (GridRotation::S, GridRotation::N) => 2,
            (GridRotation::S, GridRotation::S) => 0,
            (GridRotation::S, GridRotation::W) => 1,
            (GridRotation::S, GridRotation::E) => 1,
            (GridRotation::W, GridRotation::N) => 1,
            (GridRotation::W, GridRotation::S) => 1,
            (GridRotation::W, GridRotation::W) => 0,
            (GridRotation::W, GridRotation::E) => 2,
            (GridRotation::E, GridRotation::N) => 1,
            (GridRotation::E, GridRotation::S) => 1,
            (GridRotation::E, GridRotation::W) => 2,
            (GridRotation::E, GridRotation::E) => 0,
        }
    }
    #[allow(dead_code)]
    pub fn get_relative_rotation(&self, other: GridRotation) -> GridRotation {
        match (self, other) {
            (GridRotation::N, GridRotation::N) => GridRotation::N,
            (GridRotation::N, GridRotation::S) => GridRotation::N,
            (GridRotation::N, GridRotation::W) => GridRotation::N,
            (GridRotation::N, GridRotation::E) => GridRotation::N,
            (GridRotation::S, GridRotation::N) => GridRotation::S,
            (GridRotation::S, GridRotation::S) => GridRotation::N,
            (GridRotation::S, GridRotation::W) => GridRotation::E,
            (GridRotation::S, GridRotation::E) => GridRotation::W,
            (GridRotation::W, GridRotation::N) => GridRotation::W,
            (GridRotation::W, GridRotation::S) => GridRotation::E,
            (GridRotation::W, GridRotation::W) => GridRotation::S,
            (GridRotation::W, GridRotation::E) => GridRotation::N,
            (GridRotation::E, GridRotation::N) => GridRotation::E,
            (GridRotation::E, GridRotation::S) => GridRotation::W,
            (GridRotation::E, GridRotation::W) => GridRotation::N,
            (GridRotation::E, GridRotation::E) => GridRotation::S,
        }
    }

    pub fn get_direction(&self) -> Dir3 {
        match self {
            GridRotation::N => {Dir3::Z}
            GridRotation::S => {Dir3::NEG_Z}
            GridRotation::W => {Dir3::X}
            GridRotation::E => {Dir3::NEG_X}
        }
    }
}

pub trait GridPiece {
    fn grid_rotation(&self) -> GridRotation;
    fn grid_position(&self, grid: &WorldGrid) -> GridPosition;
}

impl GridPiece for Transform {
    fn grid_rotation(&self) -> GridRotation {
        let mut y_rotation = self.rotation.to_euler(EulerRot::YXZ).0;

        y_rotation = (y_rotation + TAU) % TAU;

        const NORTH: f32 = PI / 4.0;
        const WEST: f32 = 3.0 * PI / 4.0;
        const SOUTH: f32 = 5.0 * PI / 4.0;
        const EAST: f32 = 7.0 * PI / 4.0;

        if y_rotation < NORTH || y_rotation >= EAST {
            GridRotation::N
        } else if y_rotation < WEST {
            GridRotation::W
        } else if y_rotation < SOUTH {
            GridRotation::S
        } else {
            GridRotation::E
        }
    }

    fn grid_position(&self, grid: &WorldGrid) -> GridPosition {
        grid.get_grid_position_from_world_position(self.translation)
    }
}


#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub enum SurfaceLayer {
    #[default]
    Empty,
    // CiliaBelt {
    //     entity: Entity,
    // },
    Building {
        entity: Entity,
    },
    Resource {
        entity: Entity,
    },
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub enum ItemLayer {
    #[default]
    Empty,
    YellowBileItem,
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub struct Cell {
    pub ground_layer: GroundLayerType,
    pub surface_layer: SurfaceLayer,
    pub item_layer: ItemLayer,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct ResourceNoiseSettings {
    pub zoom_level: f32,
    pub bile_level: f32,
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
        Self { cells, grid_size }
    }

    pub fn grid_to_world(&self, grid_position: &GridPosition) -> Vec3 {
        Vec3::new(
            grid_position.x as f32 * self.grid_size,
            0.0,
            grid_position.y as f32 * self.grid_size,
        )
    }

    #[allow(dead_code)]
    pub fn set_cell(&mut self, cell: Cell, position: GridPosition) {
        self.cells.insert(position, cell);
    }
    #[allow(dead_code)]
    pub fn set_cell_at_world_position(&mut self, position: Vec3, cell: Cell) {
        let grid_position = self.get_grid_position_from_world_position(position);
        self.cells.insert(grid_position, cell);
    }

    pub fn get_grid_position_from_world_position(&self, position: Vec3) -> GridPosition {
        let x = ((position.x + self.grid_size * 0.5) / self.grid_size).floor() as i32;
        let y = ((position.z + self.grid_size * 0.5) / self.grid_size).floor() as i32;
        GridPosition { x, y }
    }

    #[allow(dead_code)]
    pub fn cell(&mut self, grid_position: &GridPosition) -> &Cell {
        if !self.cells.contains_key(grid_position) {
            self.cells.insert(*grid_position, Cell::default());
        }
        &self.cells[grid_position]
    }

    pub fn get_cell(&self, grid_position: &GridPosition) -> Option<&Cell> {
        self.cells.get(grid_position)
    }

    pub fn get_building_entity(&self, grid_position: &GridPosition) -> Option<Entity> {
        self.cells.get(grid_position).and_then(|c| match c.surface_layer {
            SurfaceLayer::Building {entity } => { Some(entity)}
            _ => None
        })
    }
}
