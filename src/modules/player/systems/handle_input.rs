use crate::{
    components::move_intent::MoveIntent,
    data::{config::TILE_SIZE, direction::Direction},
    entities::player::Player,
    modules::{
        actor::systems::move_actors::move_actors, player::plugin::PlayerPlugin, world::RpgWorld,
    },
    prelude::*,
};

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(RpgState::InGame))]
fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut MoveIntent, With<Player>>,
) {
    let dir = if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        Some(Direction::Up)
    } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        Some(Direction::Down)
    } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        Some(Direction::Left)
    } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        Some(Direction::Right)
    } else {
        None
    };

    for mut intent in &mut q {
        intent.0 = dir;
    }
}

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(RpgState::InGame))]
fn draw_gizmo(mut gizmos: Gizmos, query: Query<&Transform, With<Player>>) {
    if let Ok(transform) = query.single() {
        let pos2d = transform.translation.truncate();

        gizmos.circle_2d(pos2d, 0.5, Color::srgb(1.0, 0.0, 0.0));
    }
}

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(RpgState::InGame))]
fn draw_nearby_collision(
    mut gizmos: Gizmos,
    query: Query<&Transform, With<Player>>,
    world_query: Query<&RpgWorld>,
) {
    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found when spawning player");
        return;
    };

    let Some(rpg_level) = &world.active_level else {
        warn!("No active level found in RpgWorld");
        return;
    };

    let Ok(player_tf) = query.single() else {
        return;
    };

    let nav = &rpg_level.nav;
    let player_tile = rpg_level.world_to_tile(player_tf.translation.truncate());

    let range: i32 = 6;
    let half = TILE_SIZE * 0.5;

    // helper to draw a square outline centered at `c`
    let mut draw_square = |c: Vec2, color: Color| {
        let bl = c + Vec2::new(-half, -half);
        let br = c + Vec2::new(half, -half);
        let tr = c + Vec2::new(half, half);
        let tl = c + Vec2::new(-half, half);

        gizmos.line_2d(bl, br, color);
        gizmos.line_2d(br, tr, color);
        gizmos.line_2d(tr, tl, color);
        gizmos.line_2d(tl, bl, color);
    };

    for dy in -range..=range {
        for dx in -range..=range {
            let t = player_tile + IVec2::new(dx, dy);
            if !nav.in_bounds(t) {
                continue;
            }

            if nav.is_blocked(t) {
                let c = rpg_level.tile_center(t);
                draw_square(c, Color::srgb(1.0, 0.0, 0.0));
            }
        }
    }

    let c = rpg_level.tile_center(player_tile);
    draw_square(c, Color::srgb(1.0, 1.0, 0.0));
}

// #[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(RpgState::InGame), after = move_actors)]
// fn after_player_moved(
//     // q_transitions: Query<&MapTransitions>,
//     // mut event: EventWriter<PlayerEnteredTransition>,
//     mut next_state: ResMut<NextState<RpgState>>,
//     q_player: Query<(Entity, &Transform), With<Player>>,
// ) {
//     // let Ok(transitions) = q_transitions.single() else {
//     //     return;
//     // };

//     // if let Ok((player, transform)) = q_player.single() {
//     //     let pos = transform.translation.truncate();

//     //     for transition in transitions.0.iter() {
//     //         if transition.contains_point(pos) {
//     //             event.write(PlayerEnteredTransition {
//     //                 player,
//     //                 transition: transition.clone(),
//     //             });

//     //             next_state.set(RpgState::MapTransition);
//     //         }
//     //     }
//     // }
// }
