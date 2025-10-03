use crate::components::*;
use crate::entities::*;
use crate::modules::player::*;

#[add_system(schedule = Update, plugin = PlayerPlugin, run_if = in_state(GameState::Playing))]
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&mut GridMover, &GridPosition), With<Player>>,
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

    for (mut mover, position) in &mut q {
        if position.local_cell.is_none() {
            continue;
        }

        mover.intent = dir;
    }
}
