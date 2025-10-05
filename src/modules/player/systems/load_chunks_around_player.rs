use crate::components::*;
use crate::entities::*;
use crate::modules::player::*;
use crate::modules::world::*;

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::Playing))]
fn load_chunks_around_player(
    player_query: Query<(&GridPosition, &Transform), With<Player>>,
    world_query: Query<&RpgWorld>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found");
        return;
    };

    let mut iids = vec![];

    for (grid, transform) in player_query.iter() {
        let chunk_position = grid.get_chunk(transform.translation.truncate());

        let Some(chunk) = world.get_chunk_from_position(&chunk_position) else {
            // warn!("No chunk found at position: {:?}", chunk_position);
            continue;
        };

        chunk_manager.request_activate(chunk.id.clone());

        for chunk_id in &chunk.neighbor_chunks {
            chunk_manager.request_activate(chunk_id.clone());
        }
    }

    iids.sort();
    iids.dedup();

    let active: Vec<_> = chunk_manager.active_chunk_ids().cloned().collect();

    for active_iid in active {
        if !iids.contains(&active_iid) {
            chunk_manager.request_deactivate(active_iid);
        }
    }
}
