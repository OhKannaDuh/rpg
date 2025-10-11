prelude!();

#[derive(AssetCollection, Resource)]
pub struct ActorAssets {
    #[asset(path = "textures/entities", collection(typed, mapped))]
    pub images: HashMap<String, Handle<Image>>,

    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 48, columns = 8, rows = 1))]
    pub lodestone_layout: Handle<TextureAtlasLayout>,

    #[asset(texture_atlas_layout(tile_size_x = 48, tile_size_y = 48, columns = 3, rows = 4))]
    pub slime_layout: Handle<TextureAtlasLayout>,
}
