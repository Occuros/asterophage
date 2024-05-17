use std::ops::Sub;

use crate::building::building_components::{BlackHole, Preview};
use crate::SpatialTree;
use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

pub fn black_hole_system(
    mut commands: Commands,
    black_hole_q: Query<(&BlackHole, &Transform), Without<Preview>>,
    tree: Res<SpatialTree>,
) {
    for (_, transform) in black_hole_q.iter() {
        for (pos, other_entity) in tree.within_distance(transform.translation, 0.5) {
            let Some(other_entity) = other_entity else {continue};

            if pos.x.sub(transform.translation.x).abs() < 0.28
                && pos.z.sub(transform.translation.z).abs() < 0.28
            {
                debug!("killing {:?}", other_entity);
                commands.get_entity(other_entity).map(|e| e.despawn_recursive());
            }
        }
    }
}
