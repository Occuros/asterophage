use crate::general::Pastel;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Default, Reflect, Clone, Copy, PartialEq)]
pub enum BuildingType {
    #[default]
    None,
    Extractor,
    ConveyorBelt,
}

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
                position, rotation, size, commands, shapes,
            )),
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
}

impl BeltElement {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        scale: f32,
        commands: &mut Commands,
        shapes: &mut ShapeCommands,
    ) -> Entity {
        let entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(position)
                        .with_rotation(rotation)
                        .with_scale(Vec3::splat(scale)),
                    ..default()
                },
                BeltElement {
                    conveyor_belt: None,
                    speed: 1.0,
                },
                Building {
                    building_type: BuildingType::ConveyorBelt,
                },
                Name::new("Belt Piece"),
            ))
            .with_shape_children(&shapes.config(), |shapes| {
                shapes.hollow = true;
                shapes.transform = Transform::from_rotation(
                    Quat::from_rotation_y(TAU * 0.25) * Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.25),
                )
                    .with_translation(Vec3::Y * 0.01);
                shapes.thickness = 0.01;
                shapes.color = Color::YELLOW.pastel();
                shapes.ngon(3.0, 0.2);
                shapes.translate(Vec3::Y * -0.15);
                shapes.rect(Vec2::new(0.1, 0.3));
            })
            .id();


        entity
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct CompleteConveryorBelt {
    pub start_position: GridPosition,
    pub end_position: GridPosition,
    pub belt_pieces: Vec<BeltPiece>,
}

impl CompleteConveryorBelt {
    pub fn spawn_new(commands: &mut Commands, belt_piece: BeltPiece) -> Entity {
        let conveyor_belt_entity = commands.spawn_empty().insert(CompleteConveryorBelt {
            start_position: belt_piece.grid_position,
            end_position: belt_piece.grid_position,
            belt_pieces: vec![belt_piece],
        }).insert(Name::new("Conveyor")).id();
        conveyor_belt_entity
    }

    pub fn get_connecting_positions_from_start(&self) -> Vec<GridPosition> {
        let Some(start_piece) = self.belt_pieces.first() else { return vec![]; };
        vec![
            self.start_position.get_relative_left(start_piece.grid_rotation),
            self.start_position.get_relative_back(start_piece.grid_rotation),
            self.start_position.get_relative_right(start_piece.grid_rotation),
        ]
    }

    pub fn get_connecting_positions_from_end(&self) -> Vec<GridPosition> {
        let Some(last_piece) = self.belt_pieces.last() else { return vec![]; };
        vec![
            self.end_position.get_relative_left(last_piece.grid_rotation),
            self.end_position.get_relative_forward(last_piece.grid_rotation),
            self.end_position.get_relative_right(last_piece.grid_rotation),
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
