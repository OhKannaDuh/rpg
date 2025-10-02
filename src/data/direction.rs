#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
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
}
