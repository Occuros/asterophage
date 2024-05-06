use std::cmp::PartialEq;
use std::f32::consts::TAU;

use crate::building::building_components::*;
use crate::general::Pastel;
use crate::player::player_components::GameCursor;
use crate::world_grid::world_gird_components::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

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
        let Ok(belt) = belt_q.get(event.building_entity) else {return};
        let Some(conveyor_entity) = belt.conveyor_belt else {return};
        let Ok(mut conveyor) = conveyor_q.get_mut(conveyor_entity)  else {return};
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

        for i in 0..conveyors_entities_to_check.len() {
            let Some(&next_conveyor_entity) = conveyors_entities_to_check.first() else {
                break;
            };
            let next_conveyor = conveyor_q.get(next_conveyor_entity).unwrap();
            // info!(
            //     "check {} start: {:?} end: {:?}, origin: {:?}",
            //     i, next_conveyor.start_position, next_conveyor.end_position, grid_position
            // );

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

pub fn debug_draw_conveyors(
    mut shapes: ShapePainter,
    conveyor_q: Query<&ConveyorBelt>,
    world_grid: Res<WorldGrid>,
) {
    for conveyor in conveyor_q.iter() {
        for belt in conveyor.belt_pieces.iter() {
            shapes.transform = Transform::from_translation(
                world_grid.grid_to_world(&belt.grid_position) + Vec3::Y * 0.05,
            )
                .with_rotation(Quat::from_rotation_x(TAU * 0.25));
            shapes.thickness = 0.02;
            shapes.hollow = true;

            if belt.grid_position == conveyor.start_position() {
                shapes.color = Color::BLACK;
                shapes.circle(0.1);
            }
            if belt.grid_position == conveyor.end_position() {
                shapes.color = Color::RED;
                shapes.circle(0.15);
            }
            shapes.color = Color::PURPLE.pastel();
            shapes.rect(Vec2::splat(0.9 * world_grid.grid_size));
        }
    }
}
