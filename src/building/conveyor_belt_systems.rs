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
        let mut next_spot = vec![1.0; segments_length];

        for item in &mut conveyor.items {
            let segment = &segments[item.segment_index];
            let previous_progress = item.segment_progress;
            let last_segment_index = segments_length - 1;
            let mut reached_next_belt = false;

            item.segment_progress += (conveyor.belt_speed * time.delta_secs() / segment.length());
            item.segment_progress = item.segment_progress.clamp(0.0, 1.0);
            let item_width_progress = (item.item_width / segment.length());
            if next_spot[item.segment_index] >= 0.0 {
                item.segment_progress = item.segment_progress.min(next_spot[item.segment_index]);
                next_spot[item.segment_index] = item.segment_progress - item_width_progress;
            } else {
                item.segment_progress = previous_progress;
            }

            if item.segment_progress == 1.0 {
                if item.segment_index == last_segment_index {
                    reached_next_belt = conveyor.connected_conveyor_belt.is_some();
                } else {
                    let next_segment = &segments[item.segment_index + 1];
                    let next_segment_item_width_progress =
                        (item.item_width / next_segment.length());
                    onveyor: conveyor.connected_conveyor_belt.unwrap(),
                },
                entity,
                );
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
