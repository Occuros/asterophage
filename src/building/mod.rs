use crate::building::building_systems::*;
use bevy::prelude::*;
use crate::building::building_components::{BuildingPlacedEvent, ConveyorPlacedEvent};

use self::building_components::{Building, Extractor, BeltElement, CompleteConveryorBelt};

pub mod building_components;
mod building_systems;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildingPlacedEvent>()
            .add_event::<ConveyorPlacedEvent>()
            .register_type::<Building>()
            .register_type::<Extractor>()
            .register_type::<BeltElement>()
            .register_type::<CompleteConveryorBelt>()
            .add_systems(Update, place_building_system)
            .add_systems(Update, respond_to_conveyor_belt_placement_event.after(place_building_system))
            .add_systems(Update, handle_conveyor_placement_system.after(respond_to_conveyor_belt_placement_event))
            .add_systems(Update, debug_draw_conveyors)
        // .add_systems(Update, test_place_building_system)

        ;
    }
}
