use crate::prelude::*;
use_mod!(assets, world, chunk, chunk_manager, layer, entity, nav);

mod systems;

#[add_plugin(to_group = CorePlugins)]
pub struct WorldPlugin;

#[butler_plugin]
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkMap>()
            .register_asset_loader(LdtkLoader)
            .add_plugins(TilemapPlugin);

        app.configure_loading_state(
            LoadingStateConfig::new(AppLoadingState::LoadingAssets)
                .load_collection::<WorldAssets>(),
        );

        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugins(RapierDebugRenderPlugin::default());
    }
}
