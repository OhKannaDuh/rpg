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
        let origin = level.transform.bottom_left_bevy().as_vec2();

        let min = origin;
        let max = origin + size;

        if size.x <= half_w * 2.0 {
            transform.translation.x = origin.x + size.x * 0.5;
        } else {
            transform.translation.x = transform
                .translation
                .x
                .clamp(min.x + half_w, max.x - half_w);
        }

        if size.y <= half_h * 2.0 {
            transform.translation.y = origin.y + size.y * 0.5;
        } else {
            transform.translation.y = transform
                .translation
                .y
                .clamp(min.y + half_h, max.y - half_h);
        }
    }
}

#[add_system(schedule = OnEnter(AppState::MainMenu), plugin = DevPlugin)]
fn skip_main_menu(mut app_state: ResMut<NextState<AppState>>) {
    info!("Skipping main menu...");
    app_state.set(AppState::InGame);
}

#[add_system(schedule = Update, plugin = DevPlugin, run_if = in_state(GameState::Playing))]
fn handle_debug_input(mut state: ResMut<DebugState>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::F1) {
        state.show_collision = !state.show_collision;
        info!("Toggled collision rendering: {}", state.show_collision);
    }
}

#[add_system(schedule = Update, plugin = DevPlugin, run_if = in_state(GameState::Playing))]
fn render_baked_collision(nav_query: Query<&Nav>, mut gizmos: Gizmos, state: Res<DebugState>) {
    if !state.show_collision {
        return;
    }

    let color = Color::srgba(1.0, 0.1, 0.1, 0.65);

    for nav in nav_query.iter() {
        if let Some(baked) = &nav.baked_collision {
            for rect in baked {
                let position = rect.position.as_vec2();
                let size = rect.size.pixels().as_vec2();

                let center = Vec2::new(position.x + size.x * 0.5, position.y + size.y * 0.5);
                let isometry = Isometry2d::from_translation(center);

                gizmos.rect_2d(isometry, size, color);
            }
        }
    }
}
