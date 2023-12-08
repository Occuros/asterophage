use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component, Default)]
pub struct Player {
    pub local_aim_target: Vec3,
}

#[derive(Component, Default)]
pub struct Bullet {}

#[derive(Component, Default)]
pub struct LifeTime {
    pub time_left: f32,
}

#[derive(Component, Default)]
pub struct Owner {
    pub entity: Option<Entity>,
}

impl Bullet {
    pub fn spawn(
        position: Vec3,
        rotation: Quat,
        owner: Option<Entity>,
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        let size = 0.05;
        let shape = shape::Icosphere {
            radius: size,
            subdivisions: 12,
        };
        let transform = Transform::from_translation(position).with_rotation(rotation);
        commands
            .spawn((
                PbrBundle {
                    transform,
                    mesh: meshes.add(Mesh::try_from(shape).unwrap()),
                    material: materials.add(Color::PURPLE.into()),
                    ..default()
                },
                Bullet {},
                RigidBody::Dynamic,
                Collider::ball(size),
                LinearVelocity(transform.forward() * 15.0),
                LifeTime { time_left: 5.0 },
                GravityScale(0.0),
                Owner { entity: owner },
            ))
            .id()
    }
}



#[derive(Resource, Default)]
pub struct GameCursor {
    pub ui_position: Option<Vec2>,
    pub world_position: Option<Vec3>,
    pub preview_entity: Option<Entity>,
}
