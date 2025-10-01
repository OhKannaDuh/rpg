use crate::modules::camera::*;
use crate::prelude::*;
use bevy::render::camera::ScalingMode;

#[add_system(schedule = OnEnter(RpgState::WorldLoaded), plugin = CameraPlugin)]
fn load_world(mut commands: Commands, mut next_state: ResMut<NextState<RpgState>>) {
    info!("Spawning camera...");
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 540.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    next_state.set(RpgState::InGame);
}
