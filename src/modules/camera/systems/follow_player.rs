use crate::data::config::TILE_SIZE;
use crate::entities::player::Player;
use crate::modules::camera::*;
use crate::modules::world::RpgWorld;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = CameraPlugin, run_if = in_state(RpgState::InGame))]
fn follow_player(
    mut q_cam: Query<(&mut Transform, &Projection), (With<Camera2d>, Without<Player>)>,
    q_player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    world_query: Query<&RpgWorld>,
    time: Res<Time>,
) {
    let Ok(world) = world_query.single() else {
        return;
    };
    let Some(rpg_level) = &world.active_level else {
        return;
    };
    let nav = &rpg_level.nav;

    let Ok(player_tf) = q_player.single() else {
        return;
    };
    let Ok((mut cam_tf, proj)) = q_cam.single_mut() else {
        return;
    };

    let speed = 10.0;
    let dt = time.delta_secs();
    cam_tf.translation.x += (player_tf.translation.x - cam_tf.translation.x) * speed * dt;
    cam_tf.translation.y += (player_tf.translation.y - cam_tf.translation.y) * speed * dt;

    let map_min = nav.origin;
    let map_size = Vec2::new(nav.size.x as f32, nav.size.y as f32) * TILE_SIZE;
    let map_max = map_min + map_size;

    if let Projection::Orthographic(ortho) = proj {
        let half_w = (ortho.area.max.x - ortho.area.min.x) * 0.5;
        let half_h = (ortho.area.max.y - ortho.area.min.y) * 0.5;

        if map_size.x <= half_w * 2.0 {
            cam_tf.translation.x = (map_min.x + map_max.x) * 0.5;
        } else {
            cam_tf.translation.x = cam_tf
                .translation
                .x
                .clamp(map_min.x + half_w, map_max.x - half_w);
        }

        if map_size.y <= half_h * 2.0 {
            cam_tf.translation.y = (map_min.y + map_max.y) * 0.5;
        } else {
            cam_tf.translation.y = cam_tf
                .translation
                .y
                .clamp(map_min.y + half_h, map_max.y - half_h);
        }
    }
}
