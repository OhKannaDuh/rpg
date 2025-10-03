use crate::components::*;
use crate::modules::{Nav, actor::*};
use bevy::math::I64Vec2;

fn cell_center_world(cell: I64Vec2, grid: i64) -> Vec2 {
    let g = grid as f32;
    Vec2::new(cell.x as f32 * g + g * 0.5, cell.y as f32 * g + g * 0.5)
}

#[add_system(schedule = Update, plugin = ActorPlugin, run_if = in_state(GameState::Playing))]
fn move_actors(
    nav_query: Query<&Nav>,
    mut move_query: Query<(
        Entity,
        &mut GridMover,
        &mut GridPosition,
        &mut FaceDirection,
        &mut Transform,
        Option<&mut GridMoveState>,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut mover, mut pos, mut facing, mut transform, state_opt) in move_query.iter_mut()
    {
        let grid = mover.grid_size.max(1);

        if pos.world_cell.is_none() {
            let p = transform.translation.truncate();
            let cx = (p.x / grid as f32).floor() as i64;
            let cy = (p.y / grid as f32).floor() as i64;
            pos.world_cell = Some(I64Vec2::new(cx, cy));
        }
        let current = pos.world_cell.unwrap();

        if let Some(mut state) = state_opt {
            let dur = mover.seconds_per_cell.max(0.0001);
            state.t = (state.t + time.delta_secs() / dur).min(1.0);

            let from_px = cell_center_world(state.from, grid);
            let to_px = cell_center_world(state.to, grid);
            let p = from_px.lerp(to_px, state.t);

            transform.translation.x = p.x;
            transform.translation.y = p.y;

            if state.t >= 1.0 {
                pos.world_cell = Some(state.to);

                let end_px = cell_center_world(state.to, grid);
                transform.translation.x = end_px.x;
                transform.translation.y = end_px.y;

                commands.entity(entity).remove::<GridMoveState>();
            }

            continue;
        }

        let Some(dir) = mover.intent.take() else {
            continue;
        };
        facing.0 = dir;

        let target = current + dir.delta().as_i64vec2();

        if nav_query.iter().any(|nav| nav.is_solid_world(target)) {
            continue;
        }

        let start_px = cell_center_world(current, grid);
        transform.translation.x = start_px.x;
        transform.translation.y = start_px.y;

        commands.entity(entity).insert(GridMoveState {
            from: current,
            to: target,
            t: 0.0,
        });
    }
}
