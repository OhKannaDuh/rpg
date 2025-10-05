use crate::entities::*;
use crate::modules::player::*;

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::Playing))]
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    let dir = if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        Some(Direction::North)
    } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        Some(Direction::South)
    } else if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        Some(Direction::West)
    } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        Some(Direction::East)
    } else {
        None
    };

    let Some(dir) = dir else {
        return;
    };

    let delta = dir.delta();
    let movement =
        Vec2::new(delta.x as f32, delta.y as f32) * DEFAULT_ACTOR_SPEED * time.delta_secs();

    for mut controller in &mut query {
        controller.translation = Some(movement);
    }
}
