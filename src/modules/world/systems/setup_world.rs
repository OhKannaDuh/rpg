use crate::modules::world::*;

#[add_system(schedule = OnEnter(AppLoadingState::ProcessingWorldFile), plugin = WorldPlugin)]
fn setup_world(
    mut commands: Commands,
    world: Res<WorldAssets>,
    maps: Res<Assets<LdtkMap>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let Some(map) = maps.get(&world.ldtk_project) else {
        error!("LDtk handle present but asset not loaded yet");
        return;
    };

    let mut world = RpgWorld::from_asset(map);
    world.request_level_change("fe4e52e2-8560-11f0-975a-47c38d270567");

    commands.spawn(world);

    info!("Done loading world...");

    app_state.set(AppState::MainMenu);
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::Playing))]
fn watch_for_level_change_request(
    query: Query<&RpgWorld, Changed<RpgWorld>>,
    mut event: EventWriter<LevelChangedEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for world in query.iter() {
        if let Some(level_request) = &world.level_request {
            if let Some(active_level_key) = &world.active_level_key {
                event.write(LevelChangedEvent {
                    from: Some(active_level_key.clone()),
                    to: level_request.clone(),
                });
            } else {
                event.write(LevelChangedEvent {
                    from: None,
                    to: level_request.clone(),
                });
            }

            game_state.set(GameState::LevelTransition);
        }
    }
}
