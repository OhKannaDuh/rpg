use crate::components::face_direction::FaceDirection;
use crate::components::move_intent::MoveIntent;
use crate::entities::rpg_entity::*;
use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(RpgEntity, Sprite, FaceDirection, MoveIntent)]
pub struct Actor;
