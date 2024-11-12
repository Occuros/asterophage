use std::f32::consts::TAU;
use crate::building::building_components::*;
use crate::general::Pastel;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::*;
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy_vector_shapes::prelude::*;
use crate::utilities::utility_methods::find_child_with_name;
use crate::world_grid::components::yellow_bile::YellowBileItem;

pub fn place_building_system(
    mut commands: Commands,
    mut shapes: ShapeCommands,
    mut asset_server: ResMut<AssetServer>,
    input: Res<ButtonInput<MouseButton>>,
    game_cursor: ResMut<GameCursor>,
    mut world_grid: ResMut<WorldGrid>,
    mut building_q: Query<(Entity, &Transform, &Building)>,
    requires_ground_q: Query<&RequiresGround>,
    mut building_placed_event: EventWriter<BuildingPlacedEvent>,
) {
    if game_cursor.world_position.is_none() {
        return;
    };
    if game_cursor.preview_entity.is_none() {
        return;
    };
    if !input.just_pressed(MouseButton::Left) {
        return;
    }
    let grid_size = world_grid.grid_size;
    let position = game_cursor.world_position.unwrap();

    let grid_position = world_grid.get_grid_position_from_world_position(position);
    let building_position = world_grid.grid_to_world(&grid_position);

    if let Some(cell) = world_grid.cells.get_mut(&grid_position) {
        if cell.surface_layer != SurfaceLayer::Empty {
            return;
        };

        let preview_entity = game_cursor.preview_entity.unwrap();

        if building_q.get(preview_entity).is_err() {
            return;
        };
        let (entity, transform, building) = building_q.get_mut(preview_entity).unwrap();
        if let Ok(requires_ground) = requires_ground_q.get(entity) {
            if !requires_ground.allowed_ground.contains(&cell.ground_layer) {
                return;
            }
        }

        let placed_building = Building::spawn(
            building.building_type,
            building_position,
            transform.rotation,
            grid_size,
            &mut commands,
            &mut asset_server,
            &mut shapes,
        );
        commands.entity(placed_building.unwrap()).insert(Active {});

        cell.surface_layer = SurfaceLayer::Building {
            entity: placed_building.unwrap(),
        };

        building_placed_event.send(BuildingPlacedEvent {
            entity: placed_building.unwrap(),
            building_type: building.building_type,
            grid_position,
            grid_rotation: transform.grid_rotation(),
        });
    }
}


pub fn remove_building_system(
    input: Res<ButtonInput<MouseButton>>,
    game_cursor: ResMut<GameCursor>,
    mut world_grid: ResMut<WorldGrid>,
    mut building_removed_event: EventWriter<BuildingRemovedEvent>,
) {
    if game_cursor.world_position.is_none() {
        return;
    };

    if !input.just_pressed(MouseButton::Right) {
        return;
    }
    let position = game_cursor.world_position.unwrap();

    let grid_position = world_grid.get_grid_position_from_world_position(position);
    if let Some(cell) = world_grid.cells.get_mut(&grid_position) {
        match cell.surface_layer {
            SurfaceLayer::Building {
                entity
            } => {
                building_removed_event.send(BuildingRemovedEvent {
                    building_entity: entity,
                    grid_position,
                });
            }
            _ => return
        }
    }
}


pub fn destroy_building_system(
    mut command: Commands,
    mut building_removed_event: EventReader<BuildingRemovedEvent>,
    mut world_grid: ResMut<WorldGrid>,
) {
    for event in building_removed_event.read() {
        command.entity(event.building_entity).despawn_recursive();
        if let Some(cell) = world_grid.cells.get_mut(&event.grid_position) {
            cell.surface_layer = SurfaceLayer::Empty;
        }
    }
}

pub fn respond_to_conveyor_belt_placement_event(
    mut commands: Commands,
    mut building_placed_event: EventReader<BuildingPlacedEvent>,
    mut belt_q: Query<&mut BeltElement>,
    mut conveyor_placed_event: EventWriter<ConveyorPlacedEvent>,
) {
    for building_placed in building_placed_event.read() {
        if !matches!(building_placed.building_type, BuildingType::ConveyorBelt) {
            continue;
        }
        let belt_piece = BeltPiece {
            grid_rotation: building_placed.grid_rotation,
            entity: building_placed.entity,
            grid_position: building_placed.grid_position,
        };

        let conveyor_belt = ConveyorBelt::spawn_new(&mut commands, belt_piece);
        let mut belt_element = belt_q.get_mut(building_placed.entity).unwrap();
        belt_element.conveyor_belt = Some(conveyor_belt);
        conveyor_placed_event.send(ConveyorPlacedEvent {
            entity: conveyor_belt,
        });
    }
}

pub fn respond_to_belt_element_removal(
    mut commands: Commands,
    mut building_removed_event: EventReader<BuildingRemovedEvent>,
    mut belt_q: Query<&mut BeltElement>,
    mut conveyor_q: Query<&mut ConveyorBelt>,
) {
    for event in building_removed_event.read() {
        let Ok(belt) = belt_q.get(event.building_entity) else { return; };
        let Some(conveyor_entity) = belt.conveyor_belt else { return; };
        let Ok(mut conveyor) = conveyor_q.get_mut(conveyor_entity)  else { return; };
        if conveyor.belt_pieces.first().unwrap().entity == event.building_entity {
            conveyor.belt_pieces.remove(0);
            if (conveyor.belt_pieces.is_empty()) {
                commands.entity(conveyor_entity).despawn_recursive();
                return;
            }
        } else if conveyor.belt_pieces.last().unwrap().entity == event.building_entity {
            let new_length = conveyor.belt_pieces.len() - 1;
            conveyor.belt_pieces.truncate(new_length);
            if (conveyor.belt_pieces.is_empty()) {
                commands.entity(conveyor_entity).despawn_recursive();
                return;
            }
        } else {
            let index = conveyor.belt_pieces.iter().position(|&b| b.entity == event.building_entity).unwrap();
            let before = conveyor.belt_pieces[0..index].to_vec();
            let after = conveyor.belt_pieces[index + 1..].to_vec();
            conveyor.belt_pieces = before;


            let new_conveyor_entity = commands.spawn(ConveyorBelt {
                belt_pieces: after[..].to_vec(),
                ..default()
            }).id();

            for belt in after.iter() {
                let mut belt_element = belt_q.get_mut(belt.entity).unwrap();
                belt_element.conveyor_belt = Some(new_conveyor_entity);
            }
        }
    }
}


pub fn handle_conveyor_placement_system(
    mut commands: Commands,
    mut belt_element_placed_event: EventReader<ConveyorPlacedEvent>,
    world_grid: Res<WorldGrid>,
    mut belt_q: Query<&mut BeltElement>,
    transform_q: Query<&Transform>,
    mut conveyor_q: Query<&mut ConveyorBelt>,
    mut conveyor_segments_q: Query<&mut ConveyorSegments>,
) {
    for conveyor_placement in belt_element_placed_event.read() {
        let conveyor = conveyor_q.get(conveyor_placement.entity).unwrap();
        let belt_transform = transform_q.get(conveyor.belt_pieces[0].entity).unwrap();
        let mut primary_conveyor_entity = conveyor_placement.entity;

        let grid_position = conveyor.start_position();
        let grid_rotation = belt_transform.grid_rotation();

        let forward_position = world_grid
            .get_grid_position_from_world_position(belt_transform.translation)
            .get_relative_forward(grid_rotation);
        let backward_position: GridPosition = grid_position.get_relative_back(grid_rotation);
        let left_position = grid_position.get_relative_left(grid_rotation);
        let right_position = grid_position.get_relative_right(grid_rotation);

        let forward_conveyor_entity =
            retrieve_conveyor_from_grid(forward_position, &world_grid, &belt_q);
        let back_conveyor_entity =
            retrieve_conveyor_from_grid(backward_position, &world_grid, &belt_q);
        let left_conveyor_entity = retrieve_conveyor_from_grid(left_position, &world_grid, &belt_q);
        let right_conveyor_entity =
            retrieve_conveyor_from_grid(right_position, &world_grid, &belt_q);

        let mut conveyors_entities_to_check = vec![
            forward_conveyor_entity,
            back_conveyor_entity,
            left_conveyor_entity,
            right_conveyor_entity,
        ]
            .iter()
            .flatten()
            .map(|e| *e)
            .collect::<Vec<_>>();

        for _ in 0..conveyors_entities_to_check.len() {
            let Some(&next_conveyor_entity) = conveyors_entities_to_check.first() else {
                break;
            };
            conveyors_entities_to_check.remove(0);
            match check_for_conveyor_merge(
                &mut commands,
                primary_conveyor_entity,
                next_conveyor_entity,
                &mut belt_q,
                &mut conveyor_q,
            ) {
                Some(e) => {
                    primary_conveyor_entity = e;
                }
                None => {
                    continue;
                }
            }
        }
        let mut conveyor = conveyor_q.get_mut(primary_conveyor_entity).unwrap();
        let segments = conveyor.create_segments(&world_grid);
        conveyor_segments_q.get_mut(primary_conveyor_entity).unwrap().segments = segments;
    }
}

fn retrieve_belt_from_grid(
    grid_position: GridPosition,
    grid: &WorldGrid,
    mut belt_q: &Query<&mut BeltElement>,
) -> Option<Entity> {
    grid.cells
        .get(&grid_position)
        .and_then(|cell| match cell.surface_layer {
            SurfaceLayer::Building { entity } => {
                if belt_q.get(entity).is_ok() {
                    Some(entity)
                } else {
                    None
                }
            }
            _ => None,
        })
}

fn retrieve_conveyor_from_grid(
    grid_position: GridPosition,
    grid: &WorldGrid,
    mut belt_q: &Query<&mut BeltElement>,
) -> Option<Entity> {
    grid.cells
        .get(&grid_position)
        .and_then(|cell| match cell.surface_layer {
            SurfaceLayer::Building { entity } => {
                belt_q.get(entity).map(|b| b.conveyor_belt).ok().flatten()
            }
            _ => None,
        })
}

fn check_for_conveyor_merge(
    commands: &mut Commands,
    primary_conveyor_entity: Entity,
    secondary_conveyor_entity: Entity,
    mut belt_q: &mut Query<&mut BeltElement>,
    mut conveyor_q: &mut Query<&mut ConveyorBelt>,
) -> Option<Entity> {
    let Ok([mut primary_conveyor, mut secondary_conveyor]) =
        conveyor_q.get_many_mut([primary_conveyor_entity, secondary_conveyor_entity])
    else {
        return None;
    };
    let Some(secondary_end_piece) = secondary_conveyor.belt_pieces.last() else {
        return None;
    };
    let Some(secondary_start_piece) = secondary_conveyor.belt_pieces.first() else {
        return None;
    };

    // info!(
    //     "checking
    //  primary start: {:?} end: {:?}
    //  secondary start: {:?} end: {:?}",
    //     primary_conveyor.start_position,
    //     primary_conveyor.end_position,
    //     secondary_start_piece.grid_position,
    //     secondary_end_piece.grid_position
    // );
    if primary_conveyor.can_connect_to_start_piece(&secondary_end_piece) {
        for bp in primary_conveyor.belt_pieces.iter() {
            let mut b = belt_q.get_mut(bp.entity).unwrap();
            b.conveyor_belt = Some(secondary_conveyor_entity);
        }
        secondary_conveyor
            .belt_pieces
            .append(&mut primary_conveyor.belt_pieces);

        commands.entity(primary_conveyor_entity).despawn_recursive();
        return Some(secondary_conveyor_entity);
    } else if primary_conveyor.end_piece_can_connect_to(secondary_start_piece) {
        for bp in secondary_conveyor.belt_pieces.iter() {
            let mut b = belt_q.get_mut(bp.entity).unwrap();
            b.conveyor_belt = Some(primary_conveyor_entity);
        }

        primary_conveyor
            .belt_pieces
            .append(&mut secondary_conveyor.belt_pieces);

        commands
            .entity(secondary_conveyor_entity)
            .despawn_recursive();
        return Some(primary_conveyor_entity);
    }

    return None;
}


pub fn extract_resources_system(
    time: Res<Time>,
    world_grid: Res<WorldGrid>,
    mut extractor_q: Query<(&mut Extractor, &Transform), With<Active>>,
    mut belt_q: Query<&mut BeltElement>,
    mut conveyor_q: Query<(&mut ConveyorBelt, &ConveyorSegments)>,
    mut shapes: ShapeCommands,
) {
    for (mut extractor, transform) in extractor_q.iter_mut() {
        extractor.timer.tick(time.delta());
        if !extractor.timer.finished() { continue; }
        let grid_position = world_grid.get_grid_position_from_world_position(transform.translation);
        let grid_rotation = transform.grid_rotation();
        let potential_positions = grid_position.get_all_surrounding_positions();
        for p in potential_positions.iter() {
            let Some(cell) = world_grid.cells.get(p) else { continue; };
            let belt = match cell.surface_layer {
                SurfaceLayer::Empty => {
                    None
                }
                SurfaceLayer::Building { entity } => {
                    belt_q.get_mut(entity).ok()
                }
                SurfaceLayer::Resource { .. } => {
                    None
                }
            };

            let Some(mut belt) = belt else { continue; };
            let Some(conveyor_entity) = belt.conveyor_belt else { continue };
            let Ok((mut conveyor, conveyor_segments)) = conveyor_q.get_mut(conveyor_entity) else { continue };
            // conveyor.start_position()

            // if belt.item.is_some() { continue; }
            let item_entity = YellowBileItem::spawn(
                world_grid.grid_to_world(&p),
                Quat::IDENTITY,
                &mut shapes,
            );

            // belt.item = Some(item_entity);

            let position = world_grid.grid_to_world(&p);
            let index = conveyor_segments.get_segment_index_for_position(position)
                .expect(&format!("Somehow item not on any segment {} - {:?}", position, conveyor_segments.segments));
            let progress = conveyor_segments.segments[index].progress_for_point(position);
            conveyor.items.push(BeltItem {
                position: world_grid.grid_to_world(&p),
                item_entity,
                segment_index: index,
                segment_progress: progress,
            });
        }
    }
}


pub fn conveyor_system(
    time: Res<Time>,
    world_grid: Res<WorldGrid>,
    mut q_conveyor: Query<(&mut ConveyorBelt, &mut ConveyorSegments)>,
    mut transform_q: Query<(&mut Transform, &mut YellowBileItem), Without<BeltElement>>,
) {
    for (mut conveyor, mut conveyor_segments) in q_conveyor.iter_mut() {
        let segments = &mut conveyor_segments.segments;
        let segment_length = segments.len();
        let mut segment_blocked_progress= vec![1.0; segment_length] ;
        // conveyor.items.sort_by(|a, b| {
        //     a.segment_index.cmp(&b.segment_index)
        //         .then_with(|| a.segment_progress.partial_cmp(&b.segment_progress).unwrap_or(std::cmp::Ordering::Equal))
        // });

        for mut item in &mut conveyor.items {
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

pub fn belt_system(
    time: Res<Time>,
    world_grid: Res<WorldGrid>,
    conveyor_q: Query<&ConveyorBelt>,
    mut belt_q: Query<(&mut BeltElement, &Transform)>,
    mut transform_q: Query<(&mut Transform, &mut YellowBileItem), Without<BeltElement>>,
) {
    for conveyor in conveyor_q.iter() {
        let mut item_to_move: Option<Entity> = None;
        let mut next_potential_belt: Option<Entity> = None;
        let Some(last_belt) = conveyor.belt_pieces.last() else { continue; };
        let next_position = last_belt.grid_position.get_relative_forward(last_belt.grid_rotation);
        if let Some(cell) = world_grid.cells.get(&next_position) {
            if let SurfaceLayer::Building { entity } = cell.surface_layer {
                if let Ok((next_belt, _)) = belt_q.get(entity) {
                    if next_belt.item.is_none() {
                        next_potential_belt = Some(entity)
                    }
                }
            }
        }
        let Ok((mut last_belt, last_transform)) = belt_q.get_mut(last_belt.entity) else { continue; };
        if let Some(item_entity) = last_belt.item {
            let Ok((mut item_transform, item)) = transform_q.get_mut(item_entity) else { continue; };
            let mut movement_direction = last_transform.translation - item_transform.translation;
            movement_direction.y = 0.0;
            movement_direction = movement_direction.try_normalize().unwrap_or(*last_transform.local_z());

            if last_belt.item_reached_center {
                movement_direction = *last_transform.local_z();
            }
            let distance_before = item_transform.translation.distance(last_transform.translation);
            let current_grid_position = item_transform.grid_position(&world_grid);

            item_transform.translation += movement_direction * last_belt.speed * time.delta_seconds();

            let distance_after = item_transform.translation.distance(last_transform.translation);
            if !last_belt.item_reached_center && distance_before < distance_after {
                item_transform.translation -= movement_direction * last_belt.speed * time.delta_seconds();
                last_belt.item_reached_center = true;
            } else {
                let next_grid_position = item_transform.grid_position(&world_grid);
                if next_grid_position != current_grid_position {
                    if next_potential_belt.is_none() {
                        item_transform.translation -= movement_direction * last_belt.speed * time.delta_seconds();
                    } else {
                        item_to_move = last_belt.item;
                        last_belt.item = None;
                    }
                }
            }
        }

        if let Some(item_entity) = item_to_move {
            if let Ok((mut next_belt, _)) = belt_q.get_mut(next_potential_belt.unwrap()) {
                next_belt.item = item_to_move;
            }
        }

        for i in (0..conveyor.belt_pieces.len() - 1).rev() {
            let current = conveyor.belt_pieces[i].entity;
            let next = conveyor.belt_pieces[i + 1].entity;
            let Ok([
                   (mut current_belt, mut current_transform),
                   (mut next_belt, _)
                   ]) = belt_q.get_many_mut([current, next]) else { continue; };
            if let Some(item_entity) = current_belt.item {
                let Ok((mut item_transform, item)) = transform_q.get_mut(item_entity) else { continue; };
                let previous_position = item_transform.translation;
                let current_grid_position = item_transform.grid_position(&world_grid);
                let mut movement_direction = current_transform.translation - item_transform.translation;
                movement_direction.y = 0.0;
                movement_direction = movement_direction.try_normalize().unwrap_or(*current_transform.local_z());

                if current_belt.item_reached_center {
                    movement_direction = *current_transform.local_z();
                }
                let distance_before = item_transform.translation.distance(current_transform.translation);
                item_transform.translation += movement_direction * current_belt.speed * time.delta_seconds();
                let distance_after = item_transform.translation.distance(current_transform.translation);
                if !current_belt.item_reached_center && distance_before < distance_after {
                    // item_transform.translation.x = current_transform.translation.x;
                    // item_transform.translation.z = current_transform.translation.z;
                    item_transform.translation -= movement_direction * current_belt.speed * time.delta_seconds();
                    current_belt.item_reached_center = true;
                }

                let next_grid_position = item_transform.grid_position(&world_grid);
                if next_grid_position != current_grid_position {
                    if next_belt.item.is_none() {
                        current_belt.item = None;
                        current_belt.item_reached_center = false;
                        next_belt.item = Some(item_entity);
                        next_belt.item_reached_center = false;
                    } else {
                        item_transform.translation = previous_position;
                    }
                }
            }
        }
    }
}


pub fn inserter_animation_system(
    mut inserter_q: Query<(Entity, &mut Inserter), Without<Preview>>,
    mut transform_q: Query<&mut Transform>,
    children_q: Query<&Children>,
    name_q: Query<&Name>,
    time: Res<Time>,
) {
    for (entity, mut inserter) in inserter_q.iter_mut() {
        if inserter.rotation_spot.is_none() {
            inserter.rotation_spot = find_child_with_name(entity, "element-d", &children_q, &name_q);
            continue;
        }
        if inserter.target_reached { continue; }

        let Ok(mut robot_transform) = transform_q.get_mut(inserter.rotation_spot.unwrap()) else { continue; };
        let target_rotation = if inserter.item.is_some() { TAU / 2.5 } else { -TAU / 2.5 };
        let end_rotation = Quat::from_rotation_x(target_rotation);

        if robot_transform.rotation.angle_between(end_rotation) > 0.01 {
            robot_transform.rotation = robot_transform.rotation.slerp(end_rotation, time.delta_seconds() * 3.0);
        } else {
            inserter.target_reached = true;
        }
    }
}

pub fn inserter_system(
    mut inserter_q: Query<(Entity, &mut Inserter), Without<Preview>>,
    mut transform_q: Query<&mut Transform>,
    mut belt_q: Query<&mut BeltElement>,
    world_grid: Res<WorldGrid>,
) {
    for (entity, mut inserter) in inserter_q.iter_mut() {
        if !inserter.target_reached { continue; };

        let Ok(mut robot_transform) = transform_q.get_mut(entity) else { continue; };
        let grid_position = robot_transform.grid_position(&world_grid);
        let back_position = grid_position.get_relative_back(robot_transform.grid_rotation());
        let forward_position = grid_position.get_relative_forward(robot_transform.grid_rotation());


        if inserter.item.is_some() {
            let Some(cell) = world_grid.cells.get(&forward_position) else { continue; };
            let SurfaceLayer::Building { entity } = cell.surface_layer else { continue; };
            let Ok(mut belt) = belt_q.get_mut(entity) else { continue; };
            if belt.item.is_some() {
                debug!("belt had something at {:?}", forward_position);
                continue;
            };
            let Ok(mut item_transform) = transform_q.get_mut(inserter.item.unwrap()) else { continue; };
            debug!("we disposed something at {:?}", forward_position);
            let target_position = world_grid.grid_to_world(&forward_position);
            item_transform.translation.x = target_position.x;
            item_transform.translation.z = target_position.z;
            belt.item = inserter.item;
            inserter.item = None;
            inserter.target_reached = false;
            continue;
        } else {
            let Some(cell) = world_grid.cells.get(&back_position) else { continue; };
            let SurfaceLayer::Building { entity } = cell.surface_layer else { continue; };
            let Ok(mut belt) = belt_q.get_mut(entity) else { continue; };
            if let Some(item) = belt.item {
                inserter.item = Some(item);
                belt.item = None;
                inserter.target_reached = false;
            }
        }
    }
}




