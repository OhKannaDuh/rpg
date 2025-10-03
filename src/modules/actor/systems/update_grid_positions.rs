use crate::components::GridPosition;
use crate::modules::actor::*;
use crate::modules::world::*;
use bevy::math::I64Vec2;

#[add_system(schedule = Update, plugin = ActorPlugin, run_if = in_state(GameState::Playing))]
fn update_grid_positions(
    mut position_query: Query<(&mut GridPosition, &Transform)>,
    level_manager: Res<LevelManager>,
    world_query: Query<&RpgWorld>,
) {
    let Ok(world) = world_query.single() else {
        return;
    };

    let grid_size = world.grid_size as f32;

    for (mut position, transform) in position_query.iter_mut() {
        let p = transform.translation.truncate().floor();
        let world_x = (p.x / grid_size).floor() as i64;
        let world_y = (p.y / grid_size).floor() as i64;

        position.world_cell = Some(I64Vec2::new(world_x, world_y));

        let Some(level) = level_manager
            .active_level_iids()
            .filter_map(|iid| world.levels.get(iid))
            .find(|lvl| lvl.contains(p))
            .or_else(|| world.levels.values().find(|lvl| lvl.contains(p)))
        else {
            position.local_cell = None;
            position.level_iid = None;
            continue;
        };

        let level_origin_bl = level.transform.bottom_left_bevy().as_vec2();
        let local_px = p - level_origin_bl;
        let local_x = (local_px.x / grid_size).floor() as i64;
        let local_y = (local_px.y / grid_size).floor() as i64;

        position.local_cell = Some(I64Vec2::new(local_x, local_y));
        position.level_iid = Some(level.iid.clone());
    }
}
