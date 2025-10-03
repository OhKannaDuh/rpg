// use crate::components::*;
// use crate::entities::*;
// use crate::modules::player::*;
// use crate::modules::world::*;

// #[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::Playing))]
// fn watch_for_level_transition(
//     player_query: Query<(&GridPosition, &Transform), With<Player>>,
//     mut world_query: Query<&mut RpgWorld>,
// ) {
//     let Ok(mut world) = world_query.single_mut() else {
//         return;
//     };

//     if world.level_request.is_some() {
//         return;
//     }

//     let Some(current_level) = world.get_active_level() else {
//         return;
//     };

//     let Ok((grid_position, transform)) = player_query.single() else {
//         return;
//     };

//     if grid_position.local_cell.is_some() || grid_position.world_cell.is_none() {
//         return;
//     }

//     let Some(level_iid) = get_level(transform.translation.truncate(), current_level, &world) else {
//         return;
//     };

//     info!("Requesting level change to {}", level_iid);
//     world.request_level_change(level_iid.as_str());
// }

// fn get_level(pos: Vec2, current: &Level, world: &RpgWorld) -> Option<String> {
//     for neighbor_iid in current.neighbor_levels.values().flatten() {
//         let Some(neighbor) = world.levels.get(neighbor_iid) else {
//             continue;
//         };

//         if neighbor.contains(pos) {
//             return Some(neighbor.iid.clone());
//         }
//     }

//     for level in world.levels.values() {
//         if level.contains(pos) {
//             return Some(level.iid.clone());
//         }
//     }

//     None
// }
