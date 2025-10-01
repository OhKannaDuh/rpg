use crate::data::config::*;
use crate::modules::world::ldtk::*;
use crate::modules::world::map_bounds::MapBounds;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn build_map_bounds(
    mut commands: Commands,
    mut map_changed: EventReader<MapAssetChanged>,
    maps: Res<Assets<LdtkMap>>,
    q_maps: Query<(Entity, &LdtkMapHandle, &LdtkMapConfig)>,
) {
    for MapAssetChanged(changed_id) in map_changed.read().copied() {
        info!("Building map bounds for changed map: {:?}", changed_id);

        for (root, handle, cfg) in q_maps.iter() {
            if handle.0.id() != changed_id {
                continue;
            }

            let Some(ldtk_map) = maps.get(&handle.0) else {
                info!("LDtk map asset not loaded yet: {:?}", handle.0.id());
                continue;
            };

            let default_grid = ldtk_map.project.default_grid_size;
            let level = &ldtk_map.project.levels[cfg.selected_level];

            let tiles_x = (level.px_wid / default_grid) as u32;
            let tiles_y = (level.px_hei / default_grid) as u32;

            commands.spawn((
                MapBounds::from_center_anchor(
                    UVec2::new(tiles_x, tiles_y),
                    default_grid as f32,
                    Vec2::ZERO,
                ),
                ChildOf(root),
            ));
        }
    }
}
