use crate::prelude::*;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum RpgState {
    #[default]
    Loading,
    AssetsLoaded,
    WorldLoaded,
    InGame,
    Paused,
    MapTransition,
}
