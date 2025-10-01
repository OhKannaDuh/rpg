use crate::modules::world::assets::WorldAssets;
use crate::modules::world::ldtk::*;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = OnEnter(RpgState::AssetsLoaded), plugin = WorldPlugin)]
fn load_ldtk_project(
    mut commands: Commands,
    world: Res<WorldAssets>,
    maps: Res<Assets<LdtkMap>>,
    mut iid_map: ResMut<LdtkIidMap>,
) {
    if let Some(ldtk) = maps.get(&world.ldtk_project) {
        iid_map.0 = ldtk
            .project
            .levels
            .iter()
            .enumerate()
            .map(|(i, level)| (level.iid.clone(), i))
            .collect();
    } else {
        warn!("LDtk handle present but asset not loaded yet");
        iid_map.0.clear();
    }

    for (iid, idx) in iid_map.0.iter() {
        info!("LDtk Level IID: {} -> index {}", iid, idx);
    }

    commands.spawn(LdtkMapBundle {
        ldtk_map: LdtkMapHandle(world.ldtk_project.clone()),
        ldtk_map_config: LdtkMapConfig::new(iid_map.clone()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}
