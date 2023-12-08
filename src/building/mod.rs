use crate::building::building_systems::*;
use bevy::prelude::*;

pub mod building_components;
mod building_systems;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, place_building_system)
            // .add_systems(Update, test_place_building_system)

        ;
    }
}
