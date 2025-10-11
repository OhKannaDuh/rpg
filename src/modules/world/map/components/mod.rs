prelude!();
public!(ldtk_tiles);

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WorldRoot;

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct ChunkRoot;

#[derive(Component, Clone, Debug, PartialEq, Reflect, PartialOrd)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct PointOfInterest {
    pub observation_radius_squared: f32,
}
