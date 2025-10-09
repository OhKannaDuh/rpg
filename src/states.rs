prelude!();
use bevy::state::state::FreelyMutableState;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::Loading)]
pub enum AppLoadingState {
    #[default]
    LoadingAssets, // Handled by bevy_asset_loader
    PopulateDefinitions,
    PopulateChunkData,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    MainMenu,
    Settings,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Playing,
    Dialogue,
    Cutscene,
    Paused,
}

pub struct StatesPlugin;
game_module_build!(StatesPlugin);

impl GameModule for StatesPlugin {
    fn states(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<AppLoadingState>()
            .add_sub_state::<MainMenuState>()
            .add_sub_state::<GameState>()
            .add_loading_state(
                LoadingState::new(AppLoadingState::LoadingAssets)
                    .continue_to_state(AppLoadingState::PopulateDefinitions),
            );

        // Loading state transitions
        app.configure_state_advance(
            AppLoadingState::PopulateDefinitions,
            AppLoadingState::PopulateChunkData,
            AppLoadingSystems::PopulateDefinitions,
        )
        .configure_state_advance(
            AppLoadingState::PopulateChunkData,
            AppState::MainMenu,
            AppLoadingSystems::PopulateChunkData,
        );
    }

    fn systems(&self, app: &mut App) {
        // @todo implement main menu
        app.add_systems(OnEnter(AppState::MainMenu), advance_to(AppState::InGame));
    }
}

fn advance_to<TO>(to: TO) -> impl FnMut(ResMut<NextState<TO>>) + 'static
where
    TO: States + FreelyMutableState + Copy + 'static,
{
    move |mut next: ResMut<NextState<TO>>| {
        next.set(to);
    }
}

pub trait AppAdvanceStateExt {
    fn configure_state_advance<FROM, TO, L>(
        &mut self,
        from: FROM,
        to: TO,
        after_set: L,
    ) -> &mut Self
    where
        FROM: States + Copy + 'static,
        TO: States + FreelyMutableState + Copy + 'static,
        L: SystemSet + Clone + 'static;
}

impl AppAdvanceStateExt for App {
    fn configure_state_advance<FROM, TO, L>(
        &mut self,
        from: FROM,
        to: TO,
        after_set: L,
    ) -> &mut Self
    where
        FROM: States + Copy + 'static,
        TO: States + FreelyMutableState + Copy + 'static,
        L: SystemSet + Clone + 'static,
    {
        self.add_systems(OnEnter(from), advance_to::<TO>(to).after(after_set))
    }
}
