use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use bevy_xpbd_3d::prelude::*;
use std::f32::consts::TAU;
use bevy::input::mouse::MouseWheel;
use crate::MainCamera;
use crate::player::player_components::*;


pub const PLAYER_SPEED: f32 = 2.0;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.50 })),
            material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.7, 0.6))),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        },
        Player::default(),
        Collider::cuboid(0.25, 0.25, 0.25),
        Name::new("Player"),
    ));
}




pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_cursor: Res<GameCursor>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let mut direction = Vec3::ZERO;

    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::Z;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::Z;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        player.local_aim_target =
            transform.transform_point(game_cursor.world_position.unwrap_or_default());
    }
}

pub fn move_camera_system(
    mut cameras: Query<&mut Transform, (With<Camera>, With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut zoom_level: Local<f32>
) {
    for ev in mouse_wheel.read() {
        *zoom_level += ev.y;
    }
    *zoom_level = zoom_level.clamp(10.0, 50.0);

    if let Ok(player_transform) = player_query.get_single() {
        for mut c in cameras.iter_mut() {
            let look_target = player_transform.translation - player_transform.forward() * 3.0
                + player_transform.up() * *zoom_level;
            c.translation = look_target;
            c.look_at(player_transform.translation, Vec3::Y);
            c.rotate_around(player_transform.translation, Quat::from_rotation_y(TAU * 0.5))
        }
    }
}

#[allow(dead_code)]
pub fn paint_target(game_cursor: Res<GameCursor>, mut painter: ShapePainter) {
    if game_cursor.world_position.is_none() {
        return;
    };
    let position = game_cursor.world_position.unwrap();
    painter.set_translation(position);
    painter.transform.translation += Vec3::Y * 0.01;
    painter.transform.rotation = Quat::from_rotation_x(TAU * 0.25);
    painter.hollow = false;
    painter.color = Color::ORANGE;
    painter.circle(0.3);
}

pub fn shoot(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    input: Res<ButtonInput<MouseButton>>,
    game_cursor: Res<GameCursor>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if game_cursor.world_position.is_none() || game_cursor.preview_entity.is_some() {
        return;
    };
    let target = game_cursor.world_position.unwrap();
    let (player_entity, player_transform) = player_query.single();
    let target_position = Vec3::new(target.x, player_transform.translation.y, target.z);
    let result = player_transform.looking_at(target_position, Vec3::Y);
    if input.just_pressed(MouseButton::Left) {
        // commands.spawn(BulletBundle::new(
        //     player_transform.translation,
        //     result.rotation,
        //     Some(player_entity),
        //     meshes,
        //     materials,
        // ));
        Bullet::spawn(player_transform.translation, result.rotation, Some(player_entity), &mut commands, meshes, materials);
    }
}

pub fn life_time_system(
    mut commands: Commands,
    time: Res<Time>,
    mut life_time_query: Query<(Entity, &mut LifeTime)>,
) {
    for (e, mut life_time) in life_time_query.iter_mut() {
        life_time.time_left = (life_time.time_left - time.delta_seconds()).max(0.0);
        if life_time.time_left <= 0.0 {
            commands.entity(e).despawn()
        }
    }
}

pub fn bullet_collisions_system(
    mut commands: Commands,
    bullet_query: Query<(&Bullet, &Owner)>,
    mut collision_events: EventReader<CollisionStarted>,
) {
    for CollisionStarted(e1, e2) in collision_events.read() {
        let bullet_other = if bullet_query.get(*e1).is_ok() {
            Some((e1, e2))
        } else if bullet_query.get(*e2).is_ok()  {
            Some((e2, e1))
        } else {
            None
        } ;
        
       if let Some((bullet_entity, other_entity)) = bullet_other {
           let (_, owner)  = bullet_query.get(*bullet_entity).unwrap();
            if let Some(owner_entity) = owner.entity {
                if owner_entity == *other_entity {
                    continue;
                }
            
            commands.entity(*bullet_entity).despawn();
        }
       }
    }
}




pub fn move_light_system(
    mut light_query: Query<&mut Transform, (With<PointLight>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<PointLight>)>,
) {
    let offset = Vec3::new(0.0, 4.0, -2.0);
    if let Ok(player_transform) = player_query.get_single() {
        for mut light_transform in light_query.iter_mut() {
            light_transform.translation = player_transform.translation + offset;
        }
    }
}
