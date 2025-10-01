use crate::{
    components::{
        face_direction::FaceDirection, move_intent::MoveIntent, movement_state::MovementState,
    },
    modules::{
        actor::plugin::ActorPlugin,
        world::{RpgWorld, nav_grid},
    },
    // modules::world::nav_grid::NavGrid,
    prelude::*,
};

#[add_system(schedule = Update, plugin = ActorPlugin, run_if = in_state(RpgState::InGame))]
pub fn move_actors(
    time: Res<Time>,
    world_query: Query<&RpgWorld>,
    mut q: Query<(
        Entity,
        &mut Transform,
        &mut FaceDirection,
        &MoveIntent,
        Option<&mut MovementState>,
    )>,
    mut commands: Commands,
) {
    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found when spawning player");
        return;
    };

    let Some(rpg_level) = &world.active_level else {
        warn!("No active level found in RpgWorld");
        return;
    };

    let dt = time.delta_secs();
    let nav = &rpg_level.nav;

    for (entity, mut tf, mut facing, intent, state_opt) in &mut q {
        let z = tf.translation.z;

        if let Some(mut state) = state_opt {
            let secs_per_cell = 1.0 / state.speed_cells_per_sec;
            state.progress = (state.progress + dt / secs_per_cell).min(1.0);

            let new_xy = state.from.lerp(state.to, state.progress);
            tf.translation.x = new_xy.x;
            tf.translation.y = new_xy.y;
            tf.translation.z = z;

            if state.progress >= 1.0 - f32::EPSILON {
                tf.translation.x = state.to.x;
                tf.translation.y = state.to.y;

                let speed = state.speed_cells_per_sec;
                let arrived_at_world = state.to;
                commands.entity(entity).remove::<MovementState>();

                if let Some(dir) = intent.0 {
                    if dir == facing.0 {
                        let arrived_tile = rpg_level.world_to_tile(arrived_at_world);
                        let step = IVec2::new(dir.to_vec2().x as i32, dir.to_vec2().y as i32);
                        let next_tile = arrived_tile + step;

                        if nav.is_walkable(next_tile) {
                            let next_to = rpg_level.tile_center(next_tile);
                            commands.entity(entity).insert(MovementState::new(
                                arrived_at_world,
                                next_to,
                                speed,
                            ));
                        }
                    } else {
                        facing.0 = dir;
                    }
                }
            }

            continue;
        }

        if let Some(dir) = intent.0 {
            if dir != facing.0 {
                facing.0 = dir;
            } else {
                let from_tile = rpg_level.world_to_tile(tf.translation.truncate());
                let from = rpg_level.tile_center(from_tile);

                let step = IVec2::new(dir.to_vec2().x as i32, dir.to_vec2().y as i32);
                let to_tile = from_tile + step;

                if nav.is_walkable(to_tile) {
                    let to = rpg_level.tile_center(to_tile);

                    tf.translation.x = from.x;
                    tf.translation.y = from.y;
                    tf.translation.z = z;

                    commands
                        .entity(entity)
                        .insert(MovementState::new(from, to, 8.0));
                }
            }
        }
    }
}
