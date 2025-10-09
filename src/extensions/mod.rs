prelude!();
use bevy::ecs::system::ScheduleSystem;

pub trait GameAppExt {
    fn on_playing_game_update<M, C>(&mut self, systems: C) -> &mut Self
    where
        C: IntoScheduleConfigs<ScheduleSystem, M>;
}

impl GameAppExt for App {
    fn on_playing_game_update<M, C>(&mut self, systems: C) -> &mut Self
    where
        C: IntoScheduleConfigs<ScheduleSystem, M>,
    {
        self.add_systems(Update, systems.in_set(GameStatePlayingSet))
    }
}
pub trait LoadCollectionExt {
    /// Uses the default `AppLoadingState::LoadingAssets`.
    fn load_asset_collection<C>(&mut self) -> &mut Self
    where
        C: AssetCollection + Send + Sync + 'static;
}

impl LoadCollectionExt for App {
    fn load_asset_collection<C>(&mut self) -> &mut Self
    where
        C: AssetCollection + Send + Sync + 'static,
    {
        self.configure_loading_state(
            LoadingStateConfig::new(AppLoadingState::LoadingAssets).load_collection::<C>(),
        )
    }
}
