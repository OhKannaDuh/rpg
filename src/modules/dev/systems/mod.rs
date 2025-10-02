use super::*;
use crate::{
    modules::{Nav, RpgWorld},
    prelude::*,
};
use bevy::{ecs::query, render::camera::ScalingMode};

#[add_system(schedule = OnEnter(AppState::MainMenu), plugin = DevPlugin)]
fn spawn_camera(mut commands: Commands) {
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
}

#[add_system(schedule = Update, plugin = DevPlugin, run_if = in_state(GameState::Playing))]
fn move_camera(
    world_query: Query<&RpgWorld>,
    mut camera_query: Query<(&mut Transform, &Projection), With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(world) = world_query.single() else {
        return;
    };

    let Some(level) = world.get_active_level() else {
        return;
    };

    let Ok((mut transform, projection)) = camera_query.single_mut() else {
        return;
    };

    let delta = time.delta_secs();

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    direction = direction.normalize_or_zero();

    transform.translation += direction * delta * 320.0;

    if let Projection::Orthographic(ortho) = projection {
        let half_w = (ortho.area.max.x - ortho.area.min.x) * 0.5;
        let half_h = (ortho.area.max.y - ortho.area.min.y) * 0.5;

        let size = level.size.pixels().as_vec2();

        if size.x <= half_w * 2.0 {
            transform.translation.x = size.x * 0.5;
        } else {
            transform.translation.x = transform.translation.x.clamp(half_w, size.x - half_w);
        }

        if size.y <= half_h * 2.0 {
            transform.translation.y = size.y * 0.5;
        } else {
            transform.translation.y = transform.translation.y.clamp(half_h, size.y - half_h);
        }
    }
}

#[add_system(schedule = OnEnter(AppState::MainMenu), plugin = DevPlugin)]
fn skip_main_menu(mut app_state: ResMut<NextState<AppState>>) {
    info!("Skipping main menu...");
    app_state.set(AppState::InGame);
}

#[add_system(schedule = Update, plugin = DevPlugin, run_if = in_state(GameState::Playing))]
fn render_baked_collision(
    world_query: Query<&RpgWorld>,
    nav_query: Query<&Nav>,
    mut gizmos: Gizmos,
) {
    let Ok(world) = world_query.single() else {
        return;
    };

    let Some(level) = world.get_active_level() else {
        return;
    };

    const TILE: f32 = 16.0;
    const MAP_ORIGIN: Vec2 = Vec2::new(0.0, 0.0);

    let color = Color::srgba(1.0, 0.1, 0.1, 0.65);

    for nav in nav_query.iter() {
        if let Some(baked) = &nav.baked_collision {
            for r in baked {
                let size = Vec2::new(r.width as f32 * TILE, r.height as f32 * TILE);

                let center = Vec2::new(
                    (r.x as f32 + r.width as f32 * 0.5) * TILE,
                    (r.y as f32 + r.height as f32 * 0.5) * TILE,
                ) + MAP_ORIGIN;

                let isometry = Isometry2d {
                    rotation: Rot2::default(),
                    translation: center,
                };

                gizmos.rect_2d(isometry, size, color);
            }
        }
    }
}
