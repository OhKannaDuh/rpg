use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Name, Transform, Visibility)]
pub struct RpgEntity;
