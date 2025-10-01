use crate::{
    modules::world::{assets::WorldAssets, ldtk::*},
    prelude::*,
};

#[add_plugin(to_group = WorldPlugins)]
pub struct WorldPlugin;

#[butler_plugin]
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkMap>()
            .register_asset_loader(LdtkLoader)
            .add_plugins(TilemapPlugin);

        app.configure_loading_state(
            LoadingStateConfig::new(RpgState::Loading).load_collection::<WorldAssets>(),
        );
    }
}
