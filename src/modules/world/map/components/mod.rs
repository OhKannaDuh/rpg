prelude!();

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WorldRoot;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct ChunkRoot;
