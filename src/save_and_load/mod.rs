use crate::save_and_load::components::*;
use crate::save_and_load::systems::*;
use bevy::prelude::*;
use bevy_persistent::prelude::*;

mod components;
mod systems;

pub struct SaveLoadAsterophagePlugin;

impl Plugin for SaveLoadAsterophagePlugin {
    fn build(&self, app: &mut App) {
        let persistence_dir = std::env::current_dir().unwrap().join("assets");
        println!("we have found folder {:?}", persistence_dir);

        app.insert_resource(
            Persistent::<SaveSlots>::builder()
                .name("Save Slots")
                .format(StorageFormat::RonPrettyWithStructNames)
                .path(persistence_dir.join("save_slots.ron"))
                .default(SaveSlots::default())
                .build()
                .expect("failed to initialize save slots"),
        )
        .add_event::<SaveToSaveSlot>()
        .add_event::<LoadFromSaveSlot>()
        .add_systems(Update, save_building_system)
        .add_systems(Update, load_buildings_system)
        .add_systems(Update, detect_save_and_load_key_press_system);
    }
}
