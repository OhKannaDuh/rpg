prelude!();

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WorldRoot;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct ChunkRoot; // This should know what chunk it is by holding a ChunkId

#[derive(Component, Clone, Debug, PartialEq, Reflect, PartialOrd)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct PointOfInterest {
    pub observation_radius_squared: f32,
}
