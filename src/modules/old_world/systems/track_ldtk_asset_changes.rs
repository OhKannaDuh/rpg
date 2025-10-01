use crate::modules::world::ldtk::*;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn track_ldtk_asset_changes(
    mut event_changed: EventWriter<MapAssetChanged>,
    mut event_removed: EventWriter<MapAssetRemoved>,
    mut event_reader: EventReader<AssetEvent<LdtkMap>>,
) {
    for ev in event_reader.read() {
        match *ev {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                event_changed.write(MapAssetChanged(id));
            }
            AssetEvent::Removed { id } => {
                event_removed.write(MapAssetRemoved(id));
            }
            _ => {}
        }
    }
}
