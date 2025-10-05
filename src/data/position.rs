use bevy::math::I64Vec2;
use ldtk_rust::Level;

#[derive(Debug, Clone, Copy)]
pub struct ChunkTransform {
    pub world_x: i64,
    pub world_y: i64,
    pub px_wid: i64,
    pub px_hei: i64,
}

impl ChunkTransform {
    pub fn new(world_x: i64, world_y: i64, px_wid: i64, px_hei: i64) -> Self {
        Self {
            world_x,
            world_y,
            px_wid,
            px_hei,
        }
    }

    pub fn from_instance(level: &Level) -> Self {
        Self {
            world_x: level.world_x,
            world_y: level.world_y,
            px_wid: level.px_wid,
            px_hei: level.px_hei,
        }
    }

    pub fn local_to_bevy(&self, x_local: i64, y_local: i64) -> I64Vec2 {
        I64Vec2::new(x_local, self.px_hei - y_local)
    }

    pub fn world_to_bevy(&self, x_local: i64, y_local: i64) -> I64Vec2 {
        I64Vec2::new(self.world_x + x_local, -(self.world_y + y_local))
    }

    pub fn top_left_bevy(&self) -> I64Vec2 {
        I64Vec2::new(self.world_x, -self.world_y)
    }

    pub fn bottom_left_bevy(&self) -> I64Vec2 {
        I64Vec2::new(self.world_x, -(self.world_y + self.px_hei))
    }
}
