use bevy::{math::Vec2, reflect::Reflect};

#[derive(Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Direction::Up => Vec2::Y,
            Direction::Down => -Vec2::Y,
            Direction::Left => -Vec2::X,
            Direction::Right => Vec2::X,
        }
    }
}
