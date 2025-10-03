use super::*;
use crate::{components::GridPosition, prelude::*};

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(RpgEntity, GridPosition)]
pub struct Actor;
