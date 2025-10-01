use crate::entities::actor::*;
use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Actor)]
pub struct Player;
