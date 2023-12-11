use crate::general::Pastel;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::TimerMode::Repeating;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;
use std::time::Duration;

#[derive(Default, Reflect, Clone, Copy)]
pub enum BuildingTypes {
    #[default]
    None,
    Extractor,
    ConveyorBelt,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Building {
    pub building_type: BuildingTypes,
}

impl Building {
    pub fn spawn(
        building_type: BuildingTypes,
        position: Vec3,
        rotation: Quat,
        size: f32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
        shapes: &mut ShapeCommands,
    ) -> Option<Entity> {
        match building_type {
            BuildingTypes::None => None,
            BuildingTypes::Extractor => Some(Extractor::spawn(
                position,
                rotation,
                size,
                commands,
                asset_server,
            )),
            BuildingTypes::ConveyorBelt => Some(BeltPiece::spawn(position, rotation, size, commands, shapes)),
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
                    building_type: BuildingTypes::Extractor,
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
pub struct BeltPiece {
    pub exit_direction: GridPosition,
    pub speed: f32,
}

impl BeltPiece {
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
                Building {
                    building_type: BuildingTypes::ConveyorBelt,
                },
                Name::new("Belt Piece"),
            ))
            .with_shape_children(&shapes.config(), |shapes| {
                shapes.hollow = true;
                shapes.transform = Transform::from_rotation(
                    Quat::from_rotation_x(TAU * 0.25) * Quat::from_rotation_z(TAU * 0.50),
                ).with_translation(Vec3::Y * 0.01);
                shapes.thickness = 0.01;
                shapes.color = Color::YELLOW.pastel();
                shapes.ngon(3.0, 0.2);
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
