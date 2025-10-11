prelude!();

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WorldRoot;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct ChunkRoot;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WaterTile {
    pub base_texture_index: u32,
    pub stride: i64,
}
