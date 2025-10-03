use super::*;
use crate::modules::Nav;

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
