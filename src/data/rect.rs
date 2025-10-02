use bevy::math::I64Vec2;

use crate::data::{Size, WorldPosition};

pub struct Rect {
    pub position: I64Vec2,
    pub size: Size,
}
