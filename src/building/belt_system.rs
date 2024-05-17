use std::f32::consts::TAU;
use std::ops::Sub;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Color, Gizmos, Query, Res, Time, Transform, Without};
use bevy_spatial::SpatialAccess;
use bevy_vector_shapes::painter::ShapePainter;
use crate::building::building_components::{BeltElement, Preview};
use crate::general::general_components::Gizmodius;
use crate::SpatialTree;
use crate::world_grid::components::yellow_bile::YellowBileItem;

pub fn spatial_belt_system(
    mut belt_q: Query<(&mut BeltElement, &Transform), (Without<YellowBileItem>, Without<Preview>)>,
    mut item_q: Query<(&mut Transform, &mut YellowBileItem)>,
    time: Res<Time>,
    tree: Res<SpatialTree>,
    mut painter: ShapePainter,
    mut gizmos: Gizmos<Gizmodius>,
) {
    painter.transform.rotate_x(TAU * 0.25);

    // for (_, mut item) in item_q.iter_mut() {
    //     item.velocity *= 0.9;
    // }

    for (belt, belt_transform) in belt_q.iter() {
        painter.transform.translation = belt_transform.translation + Vec3::Y * 0.15;
        // painter.;
        // gizmos.sphere(belt_transform.translation, Quat::IDENTITY, 0.3, Color::WHITE);
        for (position, item_entity) in tree.within_distance(belt_transform.translation, 0.5) {
            let Some(item_entity) = item_entity else { continue; };
            if (position.x - belt_transform.translation.x).abs() < 0.25 && (position.z - belt_transform.translation.z).abs() < 0.25 {
                let Ok((mut item_transform, mut item)) = item_q.get_mut(item_entity) else { continue; };


                // let next_position = item_transform.translation + item.velocity.normalize() * (belt.speed) * time.delta_seconds();
                let next_position = item_transform.translation + item.velocity.normalize() * 0.1;

                let mut can_move = true;
                for (other_pos, other_item) in tree.within_distance(next_position, 0.1).iter() {
                    let Some(other_item) = *other_item else { continue; };
                    if other_item == item_entity { continue; }
                    if other_pos.sub(next_position).normalize().dot(item.velocity.normalize()) < 0.0 { continue; }
                    can_move = false;
                    // gizmos.line(item_transform.translation, next_position, Color::BLACK);
                    // gizmos.arrow(item_transform.translation, *other_pos, Color::RED);
                    // gizmos.sphere(next_position, Quat::IDENTITY, 0.05, Color::RED);
                    // gizmos.sphere(item_transform.translation + Vec3::Y * 0.2, Quat::IDENTITY, 0.05, Color::RED);
                    break;
                }

                if !can_move { continue; }
                item.velocity *= 0.9;
                item.velocity += -belt_transform.forward() * 1.0;
                item_transform.translation += item.velocity.normalize() * belt.speed * time.delta_seconds();
                // }
            }
        };
    }
}
