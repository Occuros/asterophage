use std::f32::consts::TAU;
use bevy::asset::AssetServer;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Commands, Component, default, Entity, SceneBundle, Transform};
use bevy_vector_shapes::painter::ShapeCommands;
use bevy_vector_shapes::prelude::*;
use crate::general::general_components::SpatiallyTracked;

#[derive(Component, Default)]
pub struct YellowBileResource {
    pub amount: i32,
}

#[derive(Component, Default)]
pub struct YellowBileItem {

}

impl YellowBileItem {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        mut shapes: &mut ShapeCommands,
    ) -> Entity {
        shapes.reset = true;
        shapes.transform = Transform::from_translation(position + Vec3::Y * 0.1);
        shapes.color = Color::YELLOW;
        shapes.rotate_x(TAU * 0.25);
        shapes.rotate(rotation);
        shapes.circle(0.05).insert(YellowBileItem {}).insert(SpatiallyTracked{}).id()
    }
}

impl YellowBileResource {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        size: f32,
        amount: i32,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
    ) -> Entity {
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
                YellowBileResource { amount },
            ))
            .id()
    }
}