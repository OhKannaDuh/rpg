use crate::{data::direction::Direction, prelude::*};

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct FaceDirection(pub Direction);
