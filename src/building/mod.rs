use crate::building::building_systems::*;
use bevy::prelude::*;
use crate::building::building_components::BuildingPlacedEvent;

use self::building_components::{Building, Extractor, BeltElement, CompleteConveryorBelt};

pub mod building_components;
mod building_systems;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildingPlacedEvent>()
            .register_type::<Building>()
            .register_type::<Extractor>()
            .register_type::<BeltElement>()
            .register_type::<CompleteConveryorBelt>()
            .add_systems(Update, place_building_system)
            .add_systems(Update, handle_belt_placement_system.before(place_building_system))
            .add_systems(Update, debug_draw_conveyors)
        // .add_systems(Update, test_place_building_system)

        ;
    }
}
