use crate::data::config::*;
use crate::data::map_transition::MapTransition;
use crate::modules::world::ldtk::*;
use crate::modules::world::map_transitions::MapTransitions;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn build_map_transitions(
    mut map_changed: EventReader<MapAssetChanged>,
    maps: Res<Assets<LdtkMap>>,
    // mut transitions: ResMut<MapTransitions>,
    mut q_maps: Query<(&LdtkMapHandle, &LdtkMapConfig, &mut MapTransitions)>,
) {
    for MapAssetChanged(changed_id) in map_changed.read().copied() {
        info!("Building map transitions for changed map: {:?}", changed_id);
        for (handle, cfg, mut transitions) in q_maps.iter_mut() {
            if handle.0.id() != changed_id {
                continue;
            }

            let Some(ldtk_map) = maps.get(&handle.0) else {
                continue;
            };

            transitions.0.clear();

            let default_grid = ldtk_map.project.default_grid_size;
            let level = &ldtk_map.project.levels[cfg.selected_level];

            let tile_w = (level.px_wid / default_grid) as u32;
            let tile_h = (level.px_hei / default_grid) as u32;

            let layers = level.layer_instances.as_deref().unwrap_or(&[]);
            for layer in layers
                .iter()
                .filter(|l| l.layer_instance_type == "Entities")
            {
                debug_assert_eq!(tile_w, layer.c_wid as u32);
                debug_assert_eq!(tile_h, layer.c_hei as u32);

                for ent in &layer.entity_instances {
                    if ent.identifier == ENTITY_MAP_TRANSITION_ID {
                        match MapTransition::create(ent, ldtk_map, level) {
                            Ok(t) => {
                                transitions.0.push(t);
                            }
                            Err(e) => {
                                warn!("MapTransition parse failed at entity IID {}: {e}", ent.iid);
                            }
                        }
                    }
                }
            }
        }
    }
}
