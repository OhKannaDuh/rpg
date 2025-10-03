use crate::{components::Fader, modules::camera::*};
use bevy::render::camera::ScalingMode;

#[add_system(schedule = OnEnter(AppState::MainMenu), plugin = CameraPlugin)]
fn spawn_camera(mut commands: Commands) {
    info!("Spawning camera...");
    let camera = commands
        .spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical {
                    viewport_height: 384.0,
                },
                ..OrthographicProjection::default_2d()
            }),
        ))
        .id();

    // commands.spawn((
    //     Fader { rate: 1.0 },
    //     Sprite {
    //         color: Color::srgba(0.0, 0.0, 0.0, 1.0),
    //         custom_size: Some(Vec2::splat(9999.0)),
    //         ..Default::default()
    //     },
    //     Transform::from_xyz(0.0, 0.0, 500.0),
    //     ChildOf(camera),
    // ));
}
