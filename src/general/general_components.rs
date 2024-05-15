use bevy::prelude::*;
use crate::building::building_components::BuildingType;

#[derive(Component, Default, Reflect)]
pub struct BuildingButton {
    pub building_type: BuildingType
}

#[derive(Component, Default, Reflect)]
pub struct SpatiallyTracked {

}

#[derive(Component, Default, Reflect, GizmoConfigGroup)]
pub struct Gizmodius{}