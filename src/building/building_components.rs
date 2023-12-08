use std::time::Duration;
use bevy::prelude::*;
use bevy::prelude::TimerMode::Repeating;
use crate::world_grid::world_gird_components::*;


#[derive(Default, Reflect)]
pub enum BuildingTypes {
    #[default]
    None,
    Extractor,
    ConveyorBelt,
}

#[derive(Component, Default, Reflect)]
pub struct Building {
    pub building_type: BuildingTypes,
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

        commands.spawn((
            SceneBundle {
                scene: model,
                transform: Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::splat(size)),
                ..default()
            },
            Building {
                building_type: BuildingTypes::Extractor
            },
            Extractor {
                timer: Timer::new(Duration::from_secs_f32(1.0), Repeating),

            },
            RequiresGround {
                allowed_ground: vec![GroundLayerType::BloodResource,
                                     GroundLayerType::BlackBileResource,
                                     GroundLayerType::PhlegmResource,
                                     GroundLayerType::YellowBileResource],
            }
        )).id()
    }
}