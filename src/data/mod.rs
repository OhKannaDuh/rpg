prelude!();

#[derive(Reflect, Clone, Copy, Debug, Default)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_vec2(vec: Vec2) -> Self {
        if vec.x.abs() > vec.y.abs() {
            if vec.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if vec.y > 0.0 {
            Direction::Up
        } else {
            Direction::Down
        }
    }
}
