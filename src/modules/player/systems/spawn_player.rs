use crate::components::*;
use crate::entities::*;
use crate::modules::player::*;

#[add_system(schedule = OnEnter(AppState::InGame), plugin = PlayerPlugin)]
fn spawn_player(mut commands: Commands) {
    info!("Spawning player...");

    commands.spawn((
        Name::new("Player"),
        Player,
        GridMover {
            grid_size: 16,
            seconds_per_cell: 1.0 / 8.0,
            intent: None,
        },
        Sprite {
            color: Color::srgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::splat(16.0)),
            ..Default::default()
        },
        Transform::from_xyz(320.0, -320.0, 0.0),
    ));
}
