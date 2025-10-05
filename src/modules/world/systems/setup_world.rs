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

    commands.spawn(RpgWorld::from_asset(map));

    app_state.set(AppState::MainMenu);
}
