prelude!();

pub trait GameModule {
    fn assets(&self, _app: &mut App) {}
    fn configure_loading_state(&self, _app: &mut App) {}
    fn resources(&self, _app: &mut App) {}
    fn types(&self, _app: &mut App) {}
    fn states(&self, _app: &mut App) {}
    fn system_sets(&self, _app: &mut App) {}
    fn messages(&self, _app: &mut App) {}
    fn plugins(&self, _app: &mut App) {}
    fn systems(&self, _app: &mut App) {}
}
