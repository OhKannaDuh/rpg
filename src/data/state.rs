use crate::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::Loading)]
pub enum AppLoadingState {
    #[default]
    LoadingAssets,
    ProcessingWorldFile,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    Playing,
    Dialogue,
    Cutscene,
}
