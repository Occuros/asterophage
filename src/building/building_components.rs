use crate::general::Pastel;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;
use bevy::ecs::system::{EntityCommand, EntityCommands};
use crate::general::general_components::SpatiallyTracked;

#[derive(Default, Reflect, Clone, Copy, PartialEq)]
pub enum BuildingType {
    #[default]
    None,
    Extractor,
    ConveyorBelt,
    InserterType,
    BlackHoleType,
}

#[derive(Default, Reflect, Component)]
pub struct Preview {

}

#[derive(Default, Reflect, Component)]
pub struct Active {}

#[derive(Event)]
pub struct BuildingPlacedEvent {
    pub building_type: BuildingType,
    pub grid_position: GridPosition,
    pub grid_rotation: GridRotation,
    pub entity: Entity,
}

#[derive(Event)]
pub struct ConveyorPlacedEvent {
    pub entity: Entity,
}


#[derive(Event)]
pub struct BuildingRemovedEvent {
    pub building_entity: Entity,
    pub grid_position: GridPosition,
}


#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub building_type: BuildingType,
}



impl Building {
    pub fn spawn(
        building_type: BuildingType,
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        shapes: &mut ShapeCommands,
    ) -> Option<Entity> {
        match building_type {
            BuildingType::None => None,
            BuildingType::Extractor => Some(Extractor::spawn(
                position,
                rotation,
                size,
                commands,
                asset_server,
            )),
            BuildingType::ConveyorBelt => Some(BeltElement::spawn(
                position, rotation, size, commands, shapes, asset_server,
            )),
            BuildingType::InserterType => Some(Inserter::spawn(position, rotation, size, commands, asset_server)),
            BuildingType::BlackHoleType => {
                Some(BlackHole::spawn(position, rotation, size, commands, asset_server))
            }
        }
    }
}

#[derive(Component, Default, Reflect, Debug)]
pub struct RequiresGround {
    pub allowed_ground: Vec<GroundLayerType>,
}

#[derive(Component, Default, Reflect)]
pub struct Extractor {
    pub timer: Timer,
}

impl Extractor {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
    ) -> Entity {
        let model = asset_server.load("models/extractor.glb#Scene0");
        commands
            .spawn((
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                Building {
                    building_type: BuildingType::Extractor,
                },
                Extractor {
                    timer: Timer::new(Duration::from_secs_f32(1.0), Repeating),
                },
                RequiresGround {
                    allowed_ground: vec![
                        GroundLayerType::BloodResource,
                        GroundLayerType::BlackBileResource,
                        GroundLayerType::PhlegmResource,
                        GroundLayerType::YellowBileResource,
                    ],
                },
            ))
            .id()
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct BeltElement {
    pub speed: f32,
    pub conveyor_belt: Option<Entity>,
    pub item: Option<Entity>,
    pub item_reached_center: bool,
    pub is_corner: bool,
    pub center: Vec3,
    pub is_right: bool,

}

impl BeltElement {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        shapes: &mut ShapeCommands,
        asset_server: &mut AssetServer,
    ) -> Entity {
        let model = asset_server.load("models/conveyor-bars-stripe.glb#Scene0");

        let entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation),
                        // .with_scale(Vec3::splat(size)),
                    ..default()
                },
                BeltElement {
                    conveyor_belt: None,
                    speed: 1.0,
                    ..default()
                },
                Building {
                    building_type: BuildingType::ConveyorBelt,
                },
                Name::new("Belt Piece"),
                // SpatiallyTracked{},
            ))
            .with_children(|parent| {
                parent.spawn(SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(Vec3::Y * 0.0)
                        .with_rotation(Quat::from_rotation_y(TAU * 0.25))
                        .with_scale(Vec3::splat(size)),
                    ..default()
                });
            })
            .with_shape_children(&shapes.config(), |shapes| {
                shapes.hollow = true;
                shapes.transform = Transform::from_rotation(
                    Quat::from_rotation_y(TAU * 0.25) * Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.25),
                )
                    .with_translation(Vec3::new(0.0,0.15,0.05));
                shapes.thickness = 0.01;
                shapes.color = Color::YELLOW.pastel();
                shapes.ngon(3.0, 0.1);
                shapes.translate(Vec3::Y * -0.13);
                shapes.rect(Vec2::new(0.05, 0.15));
            })
            .id();
        entity
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Inserter {
    pub item: Option<Entity>,
    pub rotation_spot: Option<Entity>,
    pub target_reached: bool,

}

impl Inserter {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        // shapes: &mut ShapeCommands,
    ) -> Entity {
        let model = asset_server.load("models/robot-arm-a.glb#Scene0");
        let entity =  commands
            .spawn((
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                Inserter {
                    ..default()
                },
                Building {
                    building_type: BuildingType::InserterType,
                },
                Name::new("Inserter"),
            ))
            // .with_shape_children(&shapes.config(), |shapes| {
            //     shapes.hollow = true;
            //     shapes.transform = Transform::from_rotation(
            //         Quat::from_rotation_y(TAU * 0.25) * Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.25),
            //     )
            //         .with_translation(Vec3::Y * 0.01);
            //     shapes.thickness = 0.01;
            //     shapes.color = Color::YELLOW.pastel();
            //     shapes.ngon(3.0, 0.2);
            //     shapes.translate(Vec3::Y * -0.15);
            //     shapes.rect(Vec2::new(0.1, 0.3));
            // })
            .id();
        entity
    }


}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ConveyorBelt {
    //belt pieces first is at the start, last at the end
    pub belt_pieces: Vec<BeltPiece>,
}

impl ConveyorBelt {

    pub fn start_position(&self) -> GridPosition {
        match self.belt_pieces.first() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position
        }
    }

    pub fn end_position(&self) -> GridPosition {
        match self.belt_pieces.last() {
            None => GridPosition::default(),
            Some(belt) => belt.grid_position
        }
    }

    pub fn spawn_new(commands: &mut Commands, belt_piece: BeltPiece) -> Entity {
        let conveyor_belt_entity = commands.spawn_empty().insert(ConveyorBelt {
            belt_pieces: vec![belt_piece],
            ..default()
        }).insert(Name::new("Conveyor")).id();
        conveyor_belt_entity
    }

    pub fn get_connecting_positions_from_start(&self) -> Vec<GridPosition> {
        let Some(start_piece) = self.belt_pieces.first() else { return vec![]; };
        vec![
            self.start_position().get_relative_left(start_piece.grid_rotation),
            self.start_position().get_relative_back(start_piece.grid_rotation),
            self.start_position().get_relative_right(start_piece.grid_rotation),
        ]
    }

    pub fn get_connecting_positions_from_end(&self) -> Vec<GridPosition> {
        let Some(last_piece) = self.belt_pieces.last() else { return vec![]; };
        vec![
            self.end_position().get_relative_left(last_piece.grid_rotation),
            self.end_position().get_relative_forward(last_piece.grid_rotation),
            self.end_position().get_relative_right(last_piece.grid_rotation),
        ]
    }


    pub fn can_connect_to_start_piece(&self, other_piece: &BeltPiece) -> bool {
        let Some(start_piece) = self.belt_pieces.first() else { return false; };
        if !self.get_connecting_positions_from_start().contains(&other_piece.grid_position) {return false;}
        start_piece
            .grid_rotation
            .difference(other_piece.grid_rotation)
            < 2
    }

    pub fn end_piece_can_connect_to(&self, other_piece: &BeltPiece) -> bool {
        let Some(end_piece) = self.belt_pieces.last() else { return false; };
        if !self.get_connecting_positions_from_end().contains(&other_piece.grid_position) {return false;}
        end_piece
            .grid_rotation
            .difference(other_piece.grid_rotation)
            < 2
    }

}

#[derive(Reflect, Clone, Copy)]
pub struct BeltPiece {
    pub entity: Entity,
    pub grid_rotation: GridRotation,
    pub grid_position: GridPosition,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct BlackHole {

}

impl BlackHole {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
    ) -> Entity {
        let model = asset_server.load("models/tower-top.glb#Scene0");
        let entity =  commands
            .spawn((
                SceneBundle {
                    scene: model,
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(size)),
                    ..default()
                },
                BlackHole {
                    ..default()
                },
                Building {
                    building_type: BuildingType::BlackHoleType,
                },
                Name::new("BlackHole"),
            ))
            .id();
        entity
    }
}