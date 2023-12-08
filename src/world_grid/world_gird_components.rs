use std::f32::consts::TAU;
use bevy::prelude::*;
use std::fmt::{Debug};
use std::ops;
use std::time::Duration;
use bevy::time::TimerMode::Repeating;
use bevy_vector_shapes::prelude::*;
use bevy::utils::HashMap;
use bevy_xpbd_3d::parry::transformation::utils::transform;
use bevy_xpbd_3d::prelude::Position;

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

#[derive(Reflect, Default)]
pub enum GridRotation {
    #[default]
    N,
    S,
    W,
    E
}



impl YellowBile {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        amount: i32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        mut shapes: &mut ShapeCommands,
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
        commands.spawn(SceneBundle {
            scene: model,
            transform: Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::splat(size)),
            ..default()
        }).id()
    }
}



#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub enum SurfaceLayer {
    #[default]
    Empty,
    CiliaBelt {
        entity: Entity
    },
    Building {
        entity: Entity
    },
    Resource {
        entity: Entity
    }
}

#[derive(Hash, Eq, PartialEq, Default, Clone, Reflect, Debug)]
pub struct Cell {
    pub ground_layer: GroundLayerType,
    pub surface_layer: SurfaceLayer,
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