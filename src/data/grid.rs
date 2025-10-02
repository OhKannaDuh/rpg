use bevy::math::I64Vec2;

pub struct Grid {
    pub width: i64,
    pub height: i64,
    pub grid_size: i64,
}

impl Grid {
    pub fn new(width: i64, height: i64, grid_size: i64) -> Self {
        Self {
            width,
            height,
            grid_size,
        }
    }

    pub fn get_bevy_position(&self, index: i64) -> I64Vec2 {
        let x = index % self.width;
        let y = index / self.width;

        I64Vec2::new(x, self.height - 1 - y)
    }
}
