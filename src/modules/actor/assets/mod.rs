prelude!();

#[derive(AssetCollection, Resource)]
pub struct ActorAssets {
    #[asset(path = "textures/entities/lodestone.png")]
    pub lodestone: Handle<Image>,
}
