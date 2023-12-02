use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub local_aim_target: Vec3,
}

#[derive(Component, Default)]
pub struct Bullet{}

#[derive(Component, Default)]
pub struct LifeTime{
    pub time_left: f32,
}

#[derive(Bundle, Default)]
pub struct BulletBundle {
    pbr_bundle: PbrBundle,
    bullet: Bullet,
    collider: Collider,
    rigid_body: RigidBody,
    velocity: LinearVelocity,
    life_time: LifeTime,
}

impl BulletBundle {
    pub fn new(
        position: Vec3,
        rotation: Quat,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,) -> BulletBundle {
        let size = 0.1;

        let shape = shape::Icosphere {radius: size, subdivisions: 12};
        let transform = Transform::from_translation(position).with_rotation(rotation);
        Self {
            pbr_bundle: PbrBundle {
                transform,
                mesh: meshes.add(Mesh::try_from(shape).unwrap()),
                material: materials.add(Color::PURPLE.into()),
                ..default()
            },
            bullet: Bullet{},
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(size),
            velocity: LinearVelocity(transform.forward() * 5.0),
            life_time: LifeTime{time_left: 5.0},
            ..default()
        }
    }
}

#[derive(Resource, Default)]
pub struct GameCursor {
    pub ui_position: Option<Vec2>,
    pub world_position: Option<Vec3>,
}