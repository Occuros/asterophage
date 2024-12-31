use crate::building::building_components::*;
use crate::save_and_load::components::*;
use crate::world_grid::world_gird_components::{
    AsGridRotation, GridRotation, SurfaceLayer, WorldGrid,
};
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::utils::info;
use bevy_persistent::Persistent;
use bevy_vector_shapes::prelude::*;
use std::collections::VecDeque;

pub fn detect_save_and_load_key_press_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut event_writer_save: EventWriter<SaveToSaveSlot>,
    mut event_writer_load: EventWriter<LoadFromSaveSlot>,
) {
    if keys.pressed(KeyCode::ControlLeft) && keys.just_pressed(KeyCode::Digit1) {
        event_writer_save.send(SaveToSaveSlot { slot_id: 1 });
    }
    if keys.pressed(KeyCode::AltLeft) && keys.just_pressed(KeyCode::Digit1) {
        event_writer_load.send(LoadFromSaveSlot { slot_id: 1 });
    }
}

pub fn load_buildings_system(
    mut load_event: EventReader<LoadFromSaveSlot>,
    mut save_slots: Res<Persistent<SaveSlots>>,
    mut commands: Commands,
    mut shapes: ShapeCommands,
    mut asset_server: ResMut<AssetServer>,
    mut world_grid: ResMut<WorldGrid>,
    mut building_placed_event: EventWriter<BuildingPlacedEvent>,
    //terrible hack, but for now it works (observers should be used later)
    mut building_queue: Local<VecDeque<PlacedBuilding>>,
) {
    let grid_size = world_grid.grid_size;

    for event in load_event.read() {
        println!("we should be loadding from save slot {:?}", event);
        let Some(save) = save_slots.slots.get(&event.slot_id) else {
            continue;
        };
        for building in &save.buildings {
            building_queue.push_back(building.clone());
        }
    }

    let Some(building) = building_queue.pop_front() else {
        return;
    };

    info!("placing building: {:?}", building);
    let grid_position = world_grid.grid_position_from_world_position(building.position);
    let Some(cell) = world_grid.cells.get_mut(&grid_position) else {
        return;
    };

    let placed_building = Building::spawn(
        building.building_type,
        building.position,
        building.rotation,
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
        grid_rotation: building.rotation.grid_rotation(),
    });

    // if let Some(cell) = world_grid.cells.get_mut(&grid_position) {
    //     if cell.surface_layer != SurfaceLayer::Empty {
    //         return;
    //     };
    //
    //
    //     let (entity, transform, building) = building_q.get_mut(preview_entity).unwrap();
    //     if let Ok(requires_ground) = requires_ground_q.get(entity) {
    //         if !requires_ground.allowed_ground.contains(&cell.ground_layer) {
    //             return;
    //         }
    //     }
    //
    //     let placed_building = Building::spawn(
    //         building.building_type,
    //         building_position,
    //         transform.rotation,
    //         grid_size,
    //         &mut commands,
    //         &mut asset_server,
    //         &mut shapes,
    //     );
    //     commands.entity(placed_building.unwrap()).insert(Active {});
    //
    //     cell.surface_layer = SurfaceLayer::Building {
    //         entity: placed_building.unwrap(),
    //     };
    //
    //     building_placed_event.send(BuildingPlacedEvent {
    //         entity: placed_building.unwrap(),
    //         building_type: building.building_type,
    //         grid_position,
    //         grid_rotation: transform.grid_rotation(),
    //     });
    // }
}

pub fn save_building_system(
    mut events: EventReader<SaveToSaveSlot>,
    mut save_slots: ResMut<Persistent<SaveSlots>>,
    q_buildings: Query<(&Building, &Transform)>,
) {
    for event in events.read() {
        save_slots
            .update(|save_slots| {
                let mut placed_buildings = vec![];
                for (building, transform) in &q_buildings {
                    placed_buildings.push(PlacedBuilding {
                        building_type: building.building_type,
                        position: transform.translation,
                        rotation: transform.rotation,
                        size: transform.scale.x,
                    })
                }
                info!("saved buildings {:?}", placed_buildings);

                save_slots.slots.insert(
                    event.slot_id,
                    SaveSlot {
                        buildings: placed_buildings,
                    },
                );
            })
            .expect("Updating Save Slots failed")
    }
}
