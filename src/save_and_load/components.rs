use crate::building::building_components::BuildingType;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PlacedBuilding {
    pub building_type: BuildingType,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: f32,
}

// #[derive(Serialize, Deserialize, Debug, Default)]
// pub struct AllPlacedBuildings {
//     pub building: Vec<PlacedBuilding>,
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SaveSlot {
    pub buildings: Vec<PlacedBuilding>,
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct SaveSlots {
    pub slots: HashMap<usize, SaveSlot>,
}

#[derive(Event, Debug)]
pub struct SaveToSaveSlot {
    pub slot_id: usize,
}

#[derive(Event, Debug)]
pub struct LoadFromSaveSlot {
    pub slot_id: usize,
}

// #[derive(Default)]
// pub struct AllPlacedBuildingsAsstSaver;
// #[derive(Default)]
// pub struct AllPlacedBuildingsLoader;
//
// impl AssetSaver for AllPlacedBuildingsAsstSaver {
//     type Asset = AllPlacedBuildings;
//     type Settings = ();
//     type OutputLoader = AllPlacedBuildingsLoader;
//     type Error = std::io::Error;
//
//     async fn save<'a>(
//         &'a self,
//         writer: &'a mut Writer,
//         asset: SavedAsset<'a, Self::Asset>,
//         settings: &'a Self::Settings,
//     ) -> impl ConditionalSendFuture<
//         Output = Result<<Self::OutputLoader as AssetLoader>::Settings, Self::Error>,
//     > {
//         // Serialize the asset to RON format
//         let ron_string = ron::ser::to_string(&*asset)
//             .map_err(|e| Self::Error::new(std::io::ErrorKind::Other, e))?;
//         // Write the serialized data to the writer
//         writer.write_all(ron_string.as_bytes()).await?;
//         Ok(settings)
//     }
// }
//
// impl AssetLoader for AllPlacedBuildingsLoader {
//     type Asset = AllPlacedBuildings;
//     type Settings = ();
//     type Error = std::io::Error;
//
//     async fn load<'a>(
//         &'a self,
//         reader: &'a mut Reader,
//         settings: &'a Self::Settings,
//         load_context: &'a mut LoadContext,
//     ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
//         let mut bytes = Vec::new();
//         reader.read_to_end(&mut bytes).await?;
//         let asset = ron::de::from_bytes::<AllPlacedBuildings>(&bytes)?;
//         Ok(asset)
//     }
//
//     fn extensions(&self) -> &[&str] {
//         &[".ron"]
//     }
// }
