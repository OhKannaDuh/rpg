use crate::modules::world::ldtk::LdtkMap;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[derive(Event, Clone, Copy)]
#[add_event(plugin = WorldPlugin)]
pub struct MapAssetRemoved(pub AssetId<LdtkMap>);
