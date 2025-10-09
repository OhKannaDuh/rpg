prelude!();
public!(world_map);

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "world/world.ldtk")]
    pub handle: Handle<WorldMap>,
}
