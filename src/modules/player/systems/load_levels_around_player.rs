use crate::components::*;
use crate::entities::*;
use crate::modules::player::*;
use crate::modules::world::*;

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::Playing))]
fn load_levels_around_player(
    player_query: Query<&GridPosition, With<Player>>,
    world_query: Query<&RpgWorld>,
    mut level_manager: ResMut<LevelManager>,
) {
    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found");
        return;
    };

    let mut iids = vec![];

    for position in player_query.iter() {
        let Some(iid) = &position.level_iid else {
            warn!("No level ID found for player");
            return;
        };

        iids.push(iid.clone());
        level_manager.request_activate(iid.clone());

        let Some(level) = world.levels.get(iid) else {
            warn!("Level ID not found in world: {}", iid);
            return;
        };

        for neighbor_iid in level.neighbor_levels.iter() {
            iids.push(neighbor_iid.clone());
            level_manager.request_activate(neighbor_iid);
        }
    }

    iids.sort();
    iids.dedup();

    let active: Vec<_> = level_manager.active_level_iids().cloned().collect();

    for active_iid in active {
        if !iids.contains(&active_iid) {
            level_manager.request_deactivate(active_iid);
        }
    }
}
