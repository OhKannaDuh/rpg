use crate::entities::*;
use crate::modules::*;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = CameraPlugin, run_if = in_state(GameState::Playing))]
fn move_camera(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let Ok(mut transform) = camera_query.single_mut() else {
        return;
    };

    let speed = 10.0;
    let dt = time.delta_secs();
    transform.translation.x +=
        (player_transform.translation.x - transform.translation.x) * speed * dt;
    transform.translation.y +=
        (player_transform.translation.y - transform.translation.y) * speed * dt;
}
