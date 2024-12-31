use crate::building::building_components::{BeltElement, ConveyorSegment};
use crate::building::conveyor_belt::{
    ConveyorBelt, ConveyorSegmentsChanged, ItemReachedOtherBeltTrigger,
};
use crate::utilities::utility_methods::{RoundBeltExt, RoundExt};
use crate::world_grid::components::yellow_bile::YellowBileItem;
use crate::world_grid::world_gird_components::{GridPiece, WorldGrid};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::utils::info;

pub fn conveyor_system(
    time: Res<Time>,
    mut q_conveyor: Query<(Entity, &mut ConveyorBelt)>,
    mut transform_q: Query<(&mut Transform, &mut YellowBileItem), Without<BeltElement>>,
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut is_paused: Local<bool>,
) {
    if (keys.just_pressed(KeyCode::KeyP)) {
        *is_paused = !*is_paused;
    }
    if *is_paused {
        return;
    }

    for (entity, conveyor) in q_conveyor.iter_mut() {
        let conveyor = conveyor.into_inner();
        let segments = &mut conveyor.segments;
        let segments_length = segments.len();
        let mut segment_blocked_progress = vec![1.0; segments_length];

        for item in &mut conveyor.items {
            let segment = &segments[item.segment_index];
            let previous_progress = item.segment_progress;
            let previous_index = item.segment_index;

            item.segment_progress += (conveyor.belt_speed * time.delta_secs() / segment.length());
            item.segment_progress.round_custom();
            let item_width_progress = (item.item_width / segment.length()).round_custom();

            // If progress exceeds 1.0, move to the next segment
            if item.segment_progress >= segment_blocked_progress[item.segment_index] {
                let next_segment_progress = item.segment_progress - 1.0;
                if item.segment_index == segments_length - 1 {
                    // info!("we reached end of segment {}", item.item_entity);
                    item.segment_progress = segment_blocked_progress[item.segment_index];
                    if (conveyor.connected_conveyor_belt.is_some()) {
                        segment_blocked_progress[item.segment_index] -= item_width_progress;
                        item.position = segment.end_position();
                        commands.trigger_targets(
                            ItemReachedOtherBeltTrigger {
                                belt_item: item.clone(),
                                next_conveyor: conveyor.connected_conveyor_belt.unwrap(),
                            },
                            entity,
                        );
                        continue;
                    }
                }
                // if item.segment_progress > 1.0 {
                //     error!(
                //         "Segment progress was way to fast! {}",
                //         item.segment_progress
                //     )
                // }

                let next_segment_index = item.segment_index + 1;
                let max_progress = segment_blocked_progress[item.segment_index];
                // info!(
                //     "segment stuff: {} {} {:?}",
                //     next_segment_index,
                //     item.segment_progress,
                //     segment_blocked_progress.get(next_segment_index),
                // );

                if next_segment_index < segments_length
                    && next_segment_progress > 0.0
                    && next_segment_progress < segment_blocked_progress[next_segment_index]
                {
                    item.segment_index = next_segment_index;
                    item.segment_progress = next_segment_progress;
                    // info!("reached next segment: {}", item.item_entity)
                } else if item.segment_progress >= max_progress {
                    // info!(
                    //     "we should block at {} - c:{} p:{} -> b:{}",
                    //     item.item_entity,
                    //     item.segment_progress,
                    //     previous_progress,
                    //     segment_blocked_progress[item.segment_index],
                    // );
                    item.segment_progress = previous_progress;
                    segment_blocked_progress[item.segment_index] =
                        previous_progress - item_width_progress;
                } else {
                    info!(
                        "we weren't blocked at {} - {} -> {}",
                        item.item_entity,
                        item.segment_progress,
                        segment_blocked_progress[item.segment_index],
                    );
                }
                // Check if we have reached the end of the path
                // if item.segment_index == segments_length - 1 && item.segment_progress >= 1.0 {
                //     if (conveyor.connected_conveyor_belt.is_some()) {
                //         commands.trigger_targets(
                //             ItemReachedOtherBeltTrigger {
                //                 belt_item: item.clone(),
                //                 next_conveyor: conveyor.connected_conveyor_belt.unwrap(),
                //             },
                //             entity,
                //         );
                //     }
                // }
            }

            // let segment = &segments[item.segment_index];
            //
            // // Update start and end position for the next segment
            // // start_position = segment.start_position;
            // // end_position = segment.end_position;
            // // Check for overlapping and update blocked progress in the current segment
            // let max_progress_index = item.segment_index;
            // let max_progress = segment_blocked_progress[max_progress_index];
            // if item.segment_progress >= max_progress {
            //     if item.segment_index == previous_index {
            //         segment_blocked_progress[item.segment_index] =
            //             previous_progress - item_width_progress;
            //     } else {
            //         segment_blocked_progress[previous_index] =
            //             previous_progress - item_width_progress;
            //     }
            //
            //     // If blocked, revert to previous progress
            //     item.segment_progress = previous_progress;
            //     item.segment_index = previous_index;
            //
            //     // info!(
            //     //     "{} max progress {}:{} new progress: {};{}",
            //     //     item.item_entity,
            //     //     max_progress_index,
            //     //     max_progress,
            //     //     item.segment_index,
            //     //     item.segment_progress
            //     // );
            //
            //     if (item.segment_progress < item_width_progress) {
            //         let diff = item_width_progress - item.segment_progress;
            //         segment_blocked_progress[item.segment_index - 1] = 1.0 - diff;
            //     }
            // }

            // if (item.segment_index == segments.len() - 1
            //     && item.segment_progress == 1.0
            //     && conveyor.connected_conveyor_belt.is_some())
            // {
            //     commands.trigger_targets(
            //         ItemReachedOtherBeltTrigger {
            //             belt_item: item.clone(),
            //             next_conveyor: conveyor.connected_conveyor_belt.unwrap(),
            //         },
            //         entity,
            //     );
            // }

            let position = segment.position_for_progress(item.segment_progress);

            if let Ok((mut item_transform, _)) = transform_q.get_mut(item.item_entity) {
                item_transform.translation = position + Vec3::Y * 0.3;
                item.position = position;
            }
        }
    }
}

pub fn segments_changed(
    trigger: Trigger<ConveyorSegmentsChanged>,
    mut q_conveyor_belts: Query<&mut ConveyorBelt>,
    q_belts: Query<(&Transform, &BeltElement)>,
    world_grid: Res<WorldGrid>,
    mut commands: Commands,
) {
    info!("segments have changed");
    let Ok(conveyor_belt) = q_conveyor_belts.get_mut(trigger.entity()) else {
        return;
    };
    let conveyor_belt = conveyor_belt.into_inner();

    let segment_count_before = conveyor_belt.segments.len();
    conveyor_belt.segments.clear();
    let segments = &mut conveyor_belt.segments;
    let belt_width_offset = 0.25; // Half of the belt width (0.5 / 2)

    // Initialize the first segment
    let mut current_segment = ConveyorSegment::default();
    let mut previous_belt = None;
    let mut is_first = true;

    for belt in &conveyor_belt.belt_pieces {
        let belt_position = world_grid.grid_to_world(&belt.grid_position);

        // Set up the first segment start position if it's the first piece
        if previous_belt.is_none() {
            // Adjust the start position slightly backward to align with the belt piece's center
            let mut start_position = belt_position;

            if is_first {
                start_position -= belt.grid_rotation.get_direction() * belt_width_offset;
                is_first = false;
            }
            current_segment.set_start_position(start_position);

            previous_belt = Some(belt);
            continue;
        }

        let previous_belt_position =
            world_grid.grid_to_world(&previous_belt.unwrap().grid_position);

        // Check if the direction has changed; if so, finalize the current segment
        if previous_belt.unwrap().grid_rotation != belt.grid_rotation {
            // Adjust the end position slightly forward to align with the belt piece's center
            let end_position = previous_belt_position
                + previous_belt.unwrap().grid_rotation.get_direction() * belt_width_offset * 2.0;
            current_segment.set_end_position(end_position);
            segments.push(current_segment);
            // Start a new segment
            current_segment = ConveyorSegment::new(belt_position, Vec3::ZERO);
        } else {
            // Continue the current segment without changing direction
            current_segment.set_end_position(belt_position);
        }

        previous_belt = Some(belt);
    }

    // Finalize the last segment if there's an unfinished one
    if let Some(previous_belt) = previous_belt {
        // Adjust the end position of the last segment
        let end_position = world_grid.grid_to_world(&previous_belt.grid_position)
            + previous_belt.grid_rotation.get_direction() * belt_width_offset;

        current_segment.set_end_position(end_position);
        segments.push(current_segment.clone());

        //allow transfer of items to other belts
        let next_position = &previous_belt
            .grid_position
            .get_relative_forward(previous_belt.grid_rotation);
        if let Some(building_entity) = world_grid.get_building_entity(next_position) {
            if let Ok((transform, belt)) = q_belts.get(building_entity) {
                if transform
                    .grid_rotation()
                    .difference(previous_belt.grid_rotation)
                    <= 1
                {
                    conveyor_belt.connected_conveyor_belt = belt.conveyor_belt;

                    let mut connector_segment =
                        ConveyorSegment::new(current_segment.end_position(), transform.translation);
                    connector_segment.is_connector = true;

                    segments.push(connector_segment);
                    commands
                        .entity(trigger.entity())
                        .observe(handle_item_reached_other_belt);
                }
            }
        }
    }

    let mut item_updates = vec![];
    let segment_count = conveyor_belt.segments.len();
    // let mut min_segment = conveyor_belt.segments.len() - 1;
    for item in &conveyor_belt.items {
        let position = item.position;
        let segment_index = conveyor_belt
            .get_segment_index_for_position(position, true)
            .expect(&format!(
                "Failed to get segment index for conveyor {:?} item: {:?}",
                conveyor_belt.segments, item
            ));

        let segment: &ConveyorSegment = &conveyor_belt.segments[segment_index];
        let progress = segment.progress_for_point(item.position);

        if (item.segment_index != segment_index) {
            let text = format!(
                "segment index changed for {}->{} p:{}->{} {}->{}",
                item.segment_index,
                segment_index,
                position,
                segment.position_for_progress(progress),
                item.segment_progress,
                progress
            );
            warn!("{}", text);
        }
        item_updates.push((segment_index, progress));
    }
    //before update check
    let mut segment_check = conveyor_belt.segments.len();
    let mut progress_check = 1.0;
    for (i, item) in conveyor_belt.items.iter().enumerate() {
        if item.segment_index < segment_check {
            segment_check = item.segment_index;
            progress_check = item.segment_progress;
        }

        if item.segment_index > segment_check {
            error!(
                "something went really wrong before update due to segment i:{} {} - {}",
                i, segment_check, item.segment_index
            );
        } else if item.segment_progress > progress_check {
            error!(
                "something went really wrong before update due to progress i:{} s:{} {} - {}",
                i, item.segment_index, progress_check, item.segment_progress
            );
        }
        progress_check = item.segment_progress;
    }

    for i in 0..item_updates.len() {
        let mut item = conveyor_belt.items.get_mut(i).unwrap();
        let update = &item_updates[i];
        if segment_count != segment_count_before {
            let message = format!(
                "{} segment updates changed progress {} -> {}",
                segment_count - segment_count_before,
                item.segment_progress,
                update.1
            );
            info!(message);
        }
        item.segment_index = update.0;
        item.segment_progress = update.1;
    }

    let mut segment_check = conveyor_belt.segments.len();
    let mut progress_check = 1.0;

    for (i, item) in conveyor_belt.items.iter().enumerate() {
        if item.segment_index < segment_check {
            segment_check = item.segment_index;
            progress_check = item.segment_progress;
            continue;
        }

        if item.segment_index > segment_check {
            error!(
                "something went really wrong due to a segment i:{} {} - {}",
                i, segment_check, item.segment_index
            );
        } else if item.segment_progress > progress_check {
            error!(
                "something went really wrong due to progress i:{} s:{} {} - {}",
                i, item.segment_index, progress_check, item.segment_progress
            );
        }
    }
}

pub fn handle_item_reached_other_belt(
    trigger: Trigger<ItemReachedOtherBeltTrigger>,
    mut q_conveyor_belt: Query<&mut ConveyorBelt>,
) {
    if trigger.entity() == trigger.event().next_conveyor {
        let mut conveyor = q_conveyor_belt.get_mut(trigger.entity()).unwrap();
        let item = &trigger.event().belt_item;
        // conveyor.remove_item(item);
        if !conveyor.has_space_at_position(item.position, item.item_width, Some(item.item_entity)) {
            return;
        }
        conveyor.remove_item(item);
        conveyor.insert_item(item);
        return;
    }

    let Ok([mut current_conveyor, mut next_conveyor]) =
        q_conveyor_belt.get_many_mut([trigger.entity(), trigger.event().next_conveyor])
    else {
        return;
    };

    let item = &trigger.event().belt_item;
    if !next_conveyor.has_space_at_position(item.position, item.item_width, None) {
        return;
    }
    current_conveyor.remove_item(item);
    next_conveyor.insert_item(item);
}
