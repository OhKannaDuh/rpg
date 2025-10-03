use crate::modules::world::*;

#[add_system(schedule = OnEnter(AppLoadingState::ProcessingWorldFile), plugin = WorldPlugin)]
fn setup_world(
    mut commands: Commands,
    world: Res<WorldAssets>,
    maps: Res<Assets<LdtkMap>>,
    mut level_manager: ResMut<LevelManager>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let Some(map) = maps.get(&world.ldtk_project) else {
        error!("LDtk handle present but asset not loaded yet");
        return;
    };

    let world = RpgWorld::from_asset(map);
    // world.request_level_change("ae72e210-8560-11f0-8c8f-5d6b77ac436c");

    commands.spawn(world);

    info!("Done loading world...");

    app_state.set(AppState::MainMenu);
}

// #[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::Playing))]
// fn watch_for_level_change_request(
//     query: Query<&RpgWorld, Changed<RpgWorld>>,
//     mut event: EventWriter<LevelChangedEvent>,
//     mut game_state: ResMut<NextState<GameState>>,
// ) {
//     for world in query.iter() {
//         if let Some(level_request) = &world.level_request {
//             if let Some(active_level_key) = &world.active_level_key {
//                 event.write(LevelChangedEvent {
//                     from: Some(active_level_key.clone()),
//                     to: level_request.clone(),
//                 });
//             } else {
//                 event.write(LevelChangedEvent {
//                     from: None,
//                     to: level_request.clone(),
//                 });
//             }

//             game_state.set(GameState::LevelTransition);
//         }
//     }
// }
