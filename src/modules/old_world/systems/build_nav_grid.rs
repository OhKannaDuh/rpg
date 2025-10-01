use crate::data::config::*;
use crate::modules::world::ldtk::*;
use crate::modules::world::nav_grid::NavGrid;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn build_nav_grid(
    mut commands: Commands,
    mut map_changed: EventReader<MapAssetChanged>,
    maps: Res<Assets<LdtkMap>>,
    q_maps: Query<(Entity, &LdtkMapHandle, &LdtkMapConfig)>,
) {
    for MapAssetChanged(changed_id) in map_changed.read().copied() {
        info!("Building nav grid for changed map: {:?}", changed_id);
        for (root, handle, cfg) in q_maps.iter() {
            if handle.0.id() != changed_id {
                continue;
            }

            let Some(ldtk_map) = maps.get(&handle.0) else {
                continue;
            };

            let default_grid = ldtk_map.project.default_grid_size;
            let level = &ldtk_map.project.levels[cfg.selected_level];

            let w = (level.px_wid / default_grid) as u32;
            let h = (level.px_hei / default_grid) as u32;

            if let Some(layer) = level.layer_instances.as_ref().and_then(|layers| {
                layers.iter().find(|l| {
                    l.identifier == LAYER_COLLISION_ID && l.layer_instance_type == "IntGrid"
                })
            }) {
                debug_assert_eq!(w, layer.c_wid as u32);
                debug_assert_eq!(h, layer.c_hei as u32);

                let mut solid = vec![false; (w * h) as usize];
                for (i, &val) in layer.int_grid_csv.iter().enumerate() {
                    let x = (i as u32) % w;
                    let y_top = (i as u32) / w;
                    let y = (h - 1) - y_top;
                    solid[(y * w + x) as usize] = val != 0;
                }

                commands.spawn((
                    NavGrid {
                        size: UVec2::new(w, h),
                        tile: ldtk_map.project.default_grid_size as f32,
                        origin: NavGrid::origin_for_center_anchor(UVec2::new(w, h), TILE_SIZE),
                        solid,
                    },
                    ChildOf(root),
                ));
            } else {
                warn!("No collision layer found in map: {:?}", changed_id);
            }
        }
    }
}
