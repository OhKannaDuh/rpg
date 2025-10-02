use super::*;
use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(RpgEntity)]
pub struct Actor;
