use crate::building::building_components::{BeltElement, ConveyorSegment};
use crate::building::conveyor_belt::{
    ConveyorBelt, ConveyorSegmentsChanged, ItemReachedOtherBeltTrigger,
};
use crate::world_grid::components::yellow_bile::YellowBileItem;
use crate::world_grid::world_gird_components::{GridPiece, WorldGrid};
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn conveyor_system(
    time: Res<Time>,
    mut q_conveyor: Query<(Entity, &mut ConveyorBelt)>,
    mut transform_q: Query<(&mut Transform, &mut YellowBileItem), Without<BeltElement>>,
    mut commands: Commands,
) {
    for (entity, conveyor) in q_conveyor.iter_mut() {
        let conveyor = conveyor.into_inner();
        let segments = &mut conveyor.segments;
        let segments_length = segments.len();
        let mut segment_blocked_progress = vec![1.0; segments_length];

        for item in &mut conveyor.items {
            let segment = &segments[item.segment_index];
            // let mut start_position = segment.start_position;
            // let mut end_position = segment.end_position;
            let previous_progress = item.segment_progress;
            let previous_index = item.segment_index;
            // Update segment progress based on speed and time

            item.segment_progress += conveyor.belt_speed * time.delta_secs() / segment.length;
            let item_width_progress = 0.1 / segment.length;

            // let mut reached_end = false;
            // If progress exceeds 1.0, move to the next segment
            while item.segment_progress >= 1.0 {
                // Subtract 1.0 to keep remaining progress
                item.segment_progress -= 1.0;
                item.segment_index += 1;

                // Check if we have reached the end of the path
                if item.segment_index >= segments_length {
                    item.segment_index = segments_length.saturating_sub(1);
                    item.segment_progress = 1.0; // Stop at the end
                    item.position = segment.position_for_progress(item.segment_progress);
                    if let Some(_) = conveyor.connected_conveyor_belt {
                        commands.trigger_targets(
                            ItemReachedOtherBeltTrigger {
                                belt_item: item.clone(),
                                next_conveyor: conveyor.connected_conveyor_belt.unwrap(),
                            },
                            entity,
                        );
                        // reached_end = true;
                    }

                    break;
                }
            }

            let segment = &segments[item.segment_index];

            // Update start and end position for the next segment
            // start_position = segment.start_position;
            // end_position = segment.end_position;
            // Check for overlapping and update blocked progress in the current segment
            let max_progress = segment_blocked_progress[item.segment_index];
            if item.segment_progress >= max_progress {
                // If blocked, revert to previous progress
                item.segment_progress = previous_progress;
                item.segment_index = previous_index;
                segment_blocked_progress[item.segment_index] =
                    item.segment_progress - item_width_progress;
            }

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
    let Ok(conveyor_belt) = q_conveyor_belts.get_mut(trigger.entity()) else {
        return;
    };
    let conveyor_belt = conveyor_belt.into_inner();

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
            current_segment.start_position = belt_position;

            if is_first {
                current_segment.start_position -=
                    belt.grid_rotation.get_direction() * belt_width_offset;
                is_first = false;
            }

            previous_belt = Some(belt);
            continue;
        }

        let previous_belt_position =
            world_grid.grid_to_world(&previous_belt.unwrap().grid_position);

        // Check if the direction has changed; if so, finalize the current segment
        if previous_belt.unwrap().grid_rotation != belt.grid_rotation {
            // Adjust the end position slightly forward to align with the belt piece's center
            current_segment.end_position = previous_belt_position
                + previous_belt.unwrap().grid_rotation.get_direction() * belt_width_offset * 2.0;
            current_segment.direction =
                Dir3::new(current_segment.end_position - current_segment.start_position).unwrap();
            current_segment.length =
                (current_segment.end_position - current_segment.start_position).length();
            segments.push(current_segment);

            // Start a new segment
            current_segment = ConveyorSegment::default();
            current_segment.start_position = belt_position
        } else {
            // Continue the current segment without changing direction
            current_segment.end_position = belt_position;
        }

        previous_belt = Some(belt);
    }

    // Finalize the last segment if there's an unfinished one
    if let Some(previous_belt) = previous_belt {
        // Adjust the end position of the last segment
        current_segment.end_position = world_grid.grid_to_world(&previous_belt.grid_position)
            + previous_belt.grid_rotation.get_direction() * belt_width_offset;

        current_segment.direction =
            Dir3::new(current_segment.end_position - current_segment.start_position).unwrap();

        current_segment.length =
            (current_segment.end_position - current_segment.start_position).length();

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
                        ConveyorSegment::new(current_segment.end_position, transform.translation);
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
    for item in &conveyor_belt.items {
        let position = item.position;
        let segment_index = conveyor_belt
            .get_segment_index_for_position(position)
            .unwrap();
        let segment: &ConveyorSegment = &conveyor_belt.segments[segment_index];
        let progress = segment.progress_for_point(item.position);
        item_updates.push((segment_index, progress));
    }

    for i in 0..item_updates.len() {
        conveyor_belt.items[i].segment_index = item_updates[i].0;
        conveyor_belt.items[i].segment_progress = item_updates[i].1;
    }
}

pub fn handle_item_reached_other_belt(
    trigger: Trigger<ItemReachedOtherBeltTrigger>,
    mut q_conveyor_belt: Query<&mut ConveyorBelt>,
) {
    if trigger.entity() == trigger.event().next_conveyor {
        let mut conveyor = q_conveyor_belt.get_mut(trigger.entity()).unwrap();
        let item = &trigger.event().belt_item;
        conveyor.remove_item(item);
        if !conveyor.has_space_at_position(item.position, 0.2) {
            conveyor.items.insert(0, item.clone());
            return;
        }
        conveyor.insert_item(item);
        return;
    }

    let Ok([mut current_conveyor, mut next_conveyor]) =
        q_conveyor_belt.get_many_mut([trigger.entity(), trigger.event().next_conveyor])
    else {
        return;
    };

    let item = &trigger.event().belt_item;
    if !next_conveyor.has_space_at_position(item.position, 0.2) {
        return;
    }
    current_conveyor.remove_item(item);
    next_conveyor.insert_item(item);
}
