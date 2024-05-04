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
            grid_position: grid_position,
            grid_rotation: transform.grid_rotation(),
        });
    }
}

pub fn handle_belt_placement_system(
    mut commands: Commands,
    mut building_placed_event: EventReader<BuildingPlacedEvent>,
    world_grid: Res<WorldGrid>,
    mut belt_q: Query<&mut BeltElement>,
    transform_q: Query<&Transform>,
    mut conveyor_q: Query<&mut CompleteConveryorBelt>,
) {
    for building_placed in building_placed_event.read() {
        if !matches!(building_placed.building_type, BuildingType::ConveyorBelt) {
            continue;
        }

        let belt_piece = BeltPiece {
            direction: building_placed.grid_rotation,
            entity: building_placed.entity,
            grid_position: building_placed.grid_position,
        };

        let forward_position = building_placed
            .grid_position
            .get_relative_forward(building_placed.grid_rotation);
        let backward_position: GridPosition = building_placed
            .grid_position
            .get_relative_back(building_placed.grid_rotation);

        let left_position = building_placed
            .grid_position
            .get_relative_left(building_placed.grid_rotation);
        let right_position = building_placed
            .grid_position
            .get_relative_right(building_placed.grid_rotation);

        let converyer_retriever = |grid_position: GridPosition| -> Option<Entity> {
            world_grid
                .cells
                .get(&grid_position)
                .and_then(|cell| match cell.surface_layer {
                    SurfaceLayer::Building { entity } => {
                        belt_q.get(entity).map(|b| b.conveyor_belt).ok().flatten()
                    }
                    _ => None,
                })
        };

        let belt_retriever = |grid_position: GridPosition| -> Option<Entity> {
            world_grid
                .cells
                .get(&grid_position)
                .and_then(|cell| match cell.surface_layer {
                    SurfaceLayer::Building { entity } => {
                        if belt_q.get_component::<BeltElement>(entity).is_ok() {
                            Some(entity)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
        };
        let forward_belt_entity = belt_retriever(forward_position);
        let back_belt_entity = belt_retriever(backward_position);
        let left_belt_entity = belt_retriever(left_position);
        let right_belt_entity = belt_retriever(right_position);

        let forward_conveyor_entity = converyer_retriever(forward_position);
        let back_conveyor_entity = converyer_retriever(backward_position);
        let left_conveyor_entity = converyer_retriever(left_position);
        let right_conveyor_entity = converyer_retriever(right_position);

        info!(
            "searching front conveyor at {:?} from {:?}",
            forward_position, building_placed.grid_position
        );

        let mut final_conveyor_entity: Option<Entity> = None;
        if let Some(forward_conveyor_entity) = forward_conveyor_entity {
            info!("we have forward conveyor");
            if let Ok(mut conveyor) = conveyor_q.get_mut(forward_conveyor_entity) {
                if conveyor.start_position == forward_position {
                    info!("it starts in front");
                    if let Some(forward_position_entity) = forward_belt_entity {
                        info!("There is a belt");
                        let transform = transform_q.get(forward_position_entity).unwrap();
                        if transform
                            .grid_rotation()
                            .difference(building_placed.grid_rotation)
                            < 2
                        {
                            conveyor.belt_pieces.insert(0, belt_piece);
                            conveyor.start_position = belt_piece.grid_position;
                            final_conveyor_entity = Some(forward_conveyor_entity);
                            info!("augmenting existing conveyor");
                        }
                    }
                }
            }
        }
        info!(
            "searching back conveyor  at {:?} from {:?}",
            backward_position, building_placed.grid_position
        );
        if let Some(back_converyor_entity) = back_conveyor_entity {
            info!("we have back conveyor");
            if final_conveyor_entity.is_none() {
                let mut back_conveyor = conveyor_q.get_mut(back_converyor_entity).unwrap();

                let back_belt_entity = back_belt_entity.unwrap();
                let back_transform = transform_q.get(back_belt_entity).unwrap();
                if back_conveyor.end_position == backward_position
                    && back_transform.grid_rotation() == building_placed.grid_rotation
                {
                    back_conveyor.end_position = building_placed.grid_position;
                    back_conveyor.belt_pieces.push(belt_piece);
                    final_conveyor_entity = Some(back_converyor_entity);
                }
            } else if forward_conveyor_entity.unwrap() != back_converyor_entity {
                let [mut final_coneyor, mut back_conveyor] = conveyor_q
                    .get_many_mut([forward_conveyor_entity.unwrap(), back_converyor_entity])
                    .unwrap();

                let back_belt_entity = back_belt_entity.unwrap();
                let back_transform = transform_q.get(back_belt_entity).unwrap();
                if back_conveyor.end_position == backward_position
                    && back_transform.grid_rotation() == building_placed.grid_rotation
                {
                    if final_coneyor.belt_pieces.len() > back_conveyor.belt_pieces.len() {
                        for bp in back_conveyor.belt_pieces.iter() {
                            let mut b: Mut<'_, BeltElement> =
                                belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                            b.conveyor_belt = final_conveyor_entity;
                        }
                        final_coneyor
                            .belt_pieces
                            .append(&mut back_conveyor.belt_pieces);
                        final_coneyor.start_position = back_conveyor.start_position;
                        commands.entity(back_converyor_entity).despawn_recursive();
                        info!("we replaced back conveyor");
                    } else {
                        for bp in final_coneyor.belt_pieces.iter() {
                            let mut b: Mut<'_, BeltElement> =
                                belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                            b.conveyor_belt = Some(back_converyor_entity);

                            back_conveyor.belt_pieces.insert(0, *bp);
                        }
                        final_coneyor.belt_pieces.clear();
                        back_conveyor.end_position = final_coneyor.end_position;
                        commands
                            .entity(final_conveyor_entity.unwrap())
                            .despawn_recursive();
                        final_conveyor_entity = Some(back_converyor_entity);
                        info!("we replaced final back conveyor");
                    }
                }
            }
        }
        if final_conveyor_entity.is_none() {
            if let Some(right_conveyor_entity) = right_conveyor_entity {
                info!("we have right conveyor");
                if final_conveyor_entity.is_none() {
                    let mut right_conveyor = conveyor_q.get_mut(right_conveyor_entity).unwrap();
                    let right_transform = transform_q.get(right_belt_entity.unwrap()).unwrap();
                    let relative_forward =
                        right_position.get_relative_forward(right_transform.grid_rotation());

                    if right_conveyor.end_position == right_position
                        && relative_forward == building_placed.grid_position
                    {
                        right_conveyor.end_position = building_placed.grid_position;
                        right_conveyor.belt_pieces.push(belt_piece);
                        final_conveyor_entity = Some(right_conveyor_entity);
                    }
                } else if final_conveyor_entity.unwrap() != right_conveyor_entity {
                    let [mut final_coneyor, mut right_conveyor] = conveyor_q
                        .get_many_mut([final_conveyor_entity.unwrap(), right_conveyor_entity])
                        .unwrap();

                    let right_transform = transform_q.get(right_belt_entity.unwrap()).unwrap();
                    let relative_forward =
                        right_position.get_relative_forward(right_transform.grid_rotation());
                    if right_conveyor.end_position == right_position
                        && relative_forward == building_placed.grid_position
                    {
                        if final_coneyor.belt_pieces.len() > right_conveyor.belt_pieces.len() {
                            for bp in right_conveyor.belt_pieces.iter() {
                                let mut b: Mut<'_, BeltElement> =
                                    belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                                b.conveyor_belt = final_conveyor_entity;
                            }
                            final_coneyor
                                .belt_pieces
                                .append(&mut right_conveyor.belt_pieces);
                            final_coneyor.start_position = right_conveyor.start_position;
                            commands.entity(right_conveyor_entity).despawn_recursive();
                            info!("we replaced right conveyor");
                        } else {
                            for bp in final_coneyor.belt_pieces.iter() {
                                let mut b: Mut<'_, BeltElement> =
                                    belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                                b.conveyor_belt = Some(right_conveyor_entity);

                                right_conveyor.belt_pieces.insert(0, *bp);
                            }
                            final_coneyor.belt_pieces.clear();
                            right_conveyor.end_position = final_coneyor.end_position;
                            commands
                                .entity(final_conveyor_entity.unwrap())
                                .despawn_recursive();
                            final_conveyor_entity = Some(right_conveyor_entity);
                            info!("we replaced final right conveyor");
                        }
                    }
                }
            }
        }

        if final_conveyor_entity.is_none() {
            if let Some(left_conveyor_entity) = left_conveyor_entity {
                info!("we have left conveyor");
                if final_conveyor_entity.is_none() {
                    let mut left_conveyor = conveyor_q.get_mut(left_conveyor_entity).unwrap();
                    let right_transform = transform_q.get(left_belt_entity.unwrap()).unwrap();
                    let relative_forward =
                        left_position.get_relative_forward(right_transform.grid_rotation());
                    if left_conveyor.end_position == left_position
                        && relative_forward == building_placed.grid_position
                    {
                        left_conveyor.end_position = building_placed.grid_position;
                        left_conveyor.belt_pieces.push(belt_piece);
                        final_conveyor_entity = Some(left_conveyor_entity);
                    }
                } else if final_conveyor_entity.unwrap() != left_conveyor_entity {
                    let [mut final_coneyor, mut left_conveyor] = conveyor_q
                        .get_many_mut([final_conveyor_entity.unwrap(), left_conveyor_entity])
                        .unwrap();

                    let left_transform = transform_q.get(left_belt_entity.unwrap()).unwrap();
                    let relative_forward =
                        left_position.get_relative_forward(left_transform.grid_rotation());
                    if left_conveyor.end_position == left_position
                        && relative_forward == building_placed.grid_position
                    {
                        if final_coneyor.belt_pieces.len() > left_conveyor.belt_pieces.len() {
                            for bp in left_conveyor.belt_pieces.iter() {
                                let mut b: Mut<'_, BeltElement> =
                                    belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                                b.conveyor_belt = final_conveyor_entity;
                            }
                            final_coneyor
                                .belt_pieces
                                .append(&mut left_conveyor.belt_pieces);
                            final_coneyor.start_position = left_conveyor.start_position;
                            commands.entity(left_conveyor_entity).despawn_recursive();
                            info!("we replaced left conveyor");
                        } else {
                            for bp in final_coneyor.belt_pieces.iter() {
                                let mut b: Mut<'_, BeltElement> =
                                    belt_q.get_component_mut::<BeltElement>(bp.entity).unwrap();
                                b.conveyor_belt = Some(left_conveyor_entity);
                                left_conveyor.belt_pieces.insert(0, *bp);
                            }
                            final_coneyor.belt_pieces.clear();
                            left_conveyor.end_position = final_coneyor.end_position;
                            commands
                                .entity(final_conveyor_entity.unwrap())
                                .despawn_recursive();
                            final_conveyor_entity = Some(left_conveyor_entity);
                            info!("we replaced final left conveyor");
                        }
                    }
                }
            }
        }

        info!("placed entity {:?}", building_placed.entity);
        if let Ok(mut placed_belt) = belt_q.get_mut(building_placed.entity)
        {
            placed_belt.conveyor_belt = final_conveyor_entity;
            if placed_belt.conveyor_belt.is_none() {
                info!("creating fresh conveyor");

                let conveyor_entity = CompleteConveryorBelt::spawn_new(&mut commands, belt_piece);
                placed_belt.conveyor_belt = Some(conveyor_entity);
            }
        }
    }
}

pub fn debug_draw_conveyors(
    mut shapes: ShapePainter,
    conveyor_q: Query<&CompleteConveryorBelt>,
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

            if belt.grid_position == conveyor.start_position {
                shapes.color = Color::BLACK;
                shapes.circle(0.1);
            }
            if belt.grid_position == conveyor.end_position {
                shapes.color = Color::RED;
                shapes.circle(0.15);
            }
            shapes.color = Color::PURPLE.pastel();
            shapes.rect(Vec2::splat(0.9 * world_grid.grid_size));
        }
    }
}
