prelude!();

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppLoadingSystems {
    PopulateDefinitions,
    PopulateChunkData,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameStatePlayingSet;

pub struct SystemSetsPlugin;
game_module_build!(SystemSetsPlugin);

impl GameModule for SystemSetsPlugin {
    fn system_sets(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (GameStatePlayingSet.run_if(in_state(GameState::Playing)),),
        );
    }
}
