use bevy::{math::IVec2, reflect::Reflect};

#[derive(Clone, Copy, Reflect, Debug, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    North,
    East,
    #[default]
    South,
    West,
}

impl Direction {
    pub fn from_ldtk_neighbor(neighbor: String) -> Option<Self> {
        match neighbor.as_str() {
            "n" => Some(Direction::North),
            "e" => Some(Direction::East),
            "s" => Some(Direction::South),
            "w" => Some(Direction::West),
            _ => None,
        }
    }

    pub fn delta(self) -> IVec2 {
        match self {
            Direction::North => IVec2::new(0, 1),
            Direction::South => IVec2::new(0, -1),
            Direction::West => IVec2::new(-1, 0),
            Direction::East => IVec2::new(1, 0),
        }
    }
}
