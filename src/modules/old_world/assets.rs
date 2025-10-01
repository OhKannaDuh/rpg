use crate::{modules::world::ldtk::LdtkMap, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "world/world.ldtk")]
    pub ldtk_project: Handle<LdtkMap>,
}
