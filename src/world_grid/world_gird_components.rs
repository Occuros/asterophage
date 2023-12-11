use bevy::{prelude::*, utils::HashMap};
use bevy_vector_shapes::prelude::*;
use std::{f32::consts::TAU, f32::consts::PI, ops};

#[derive(Component, Reflect, Hash, Eq, PartialEq, Debug, Clone, Default, Copy)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
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

#[derive(Component, Default)]
pub struct YellowBile {
    pub amount: i32,
}

#[derive(Reflect, Default, Debug)]
pub enum GridRotation {
    #[default]
    N,
    S,
    W,
    E,
}

pub trait GridPiece {
    fn grid_rotation(&self) -> GridRotation;
}

impl GridPiece for Transform {
    fn grid_rotation(&self) -> GridRotation {

        let mut y_rotation = self.rotation.to_euler(EulerRot::YXZ).0;

        y_rotation = (y_rotation + TAU) % TAU;

        const NORTH: f32 = PI / 4.0;
        const WEST: f32 = 3.0 * PI / 4.0;
        const SOUTH: f32 = 5.0 * PI / 4.0;
        const EAST: f32 = 7.0 * PI / 4.0;

        if  y_rotation < NORTH || y_rotation >= EAST {
            GridRotation::N
        } else if y_rotation < WEST {
            GridRotation::W
        } else if y_rotation < SOUTH {
            GridRotation::S
        } else {
            GridRotation::E
        }
    }
}

impl YellowBile {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        amount: i32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        mut _shapes: &mut ShapeCommands,
    ) -> Entity {
        // shapes.reset = true;
        // shapes.transform = Transform::from_translation(position + Vec3::Y * 0.01);
        // shapes.color = Color::YELLOW;
        // shapes.rotate_x(TAU * 0.25);
        // shapes.rotate(rotation);
        // shapes.circle(0.10).insert(YellowBile {
        //     amount
        // });
        let model = asset_server.load("models/bile-node.glb#Scene0");
        commands
            .spawn((
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                YellowBile { amount },
            ))
            .id()
    }
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub enum SurfaceLayer {
    #[default]
    Empty,
    CiliaBelt {
        entity: Entity,
    },
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
}
