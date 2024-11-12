use bevy::math::Vec3;
use bevy::prelude::{Query, Res, Time, Transform, Without};
use crate::building::building_components::{BeltElement, ConveyorBelt};
use crate::world_grid::components::yellow_bile::YellowBileItem;

pub fn conveyor_system(
    time: Res<Time>,
    mut q_conveyor: Query<&mut ConveyorBelt>,
    mut transform_q: Query<(&mut Transform, &mut YellowBileItem), Without<BeltElement>>,
) {
    for conveyor in q_conveyor.iter_mut() {
        let conveyor = conveyor.into_inner();
        let segments = &mut conveyor.segments;
        let segment_length = segments.len();
        let mut segment_blocked_progress= vec![1.0; segment_length] ;

        for item in &mut conveyor.items {
            let segment = &segments[item.segment_index];
            let mut start_position = segment.start_position;
            let mut end_position = segment.end_position;
            let previous_progress = item.segment_progress;
            // Update segment progress based on speed and time
            item.segment_progress += 0.50 * time.delta_seconds() / segment.length;
            let item_width_progress = 0.1 / segment.length;


            // If progress exceeds 1.0, move to the next segment
            while item.segment_progress >= 1.0 {
                // Subtract 1.0 to keep remaining progress
                item.segment_progress -= 1.0;
                item.segment_index += 1;

                // Check if we have reached the end of the path
                if item.segment_index >= segment_length {
                    item.segment_index = segment_length.saturating_sub(1);
                    item.segment_progress = 1.0; // Stop at the end
                    break;
                }

                // Update start and end position for the next segment
                start_position = segments[item.segment_index].start_position;
                end_position = segments[item.segment_index].end_position;
            }

            // Check for overlapping and update blocked progress in the current segment
            let max_progress = segment_blocked_progress[item.segment_index];
            if item.segment_progress >= max_progress {
                // If blocked, revert to previous progress
                item.segment_progress = previous_progress;
                segment_blocked_progress[item.segment_index] = item.segment_progress - item_width_progress;
            }


            let position = start_position + (end_position - start_position) * item.segment_progress;

            if let Ok((mut item_transform, _)) = transform_q.get_mut(item.item_entity) {
                item_transform.translation = position + Vec3::Y * 0.3;
            }
        }
    }
}
