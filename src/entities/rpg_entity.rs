use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Name, Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct RpgEntity;
