use super::*;
use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct Player;
