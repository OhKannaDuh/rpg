use bevy::math::I64Vec2;
use ldtk_rust::Level;

#[derive(Clone, Copy)]
pub struct WorldOrigin(pub I64Vec2);

impl WorldOrigin {
    pub fn from_ldtk_level(level: &Level) -> Self {
        Self(I64Vec2::new(level.world_x, level.world_y + level.px_hei))
    }
}

#[derive(Clone, Copy)]
pub struct WorldPosition {
    pub local: I64Vec2,
    pub origin: I64Vec2,
}

impl WorldPosition {
    pub fn new_ldtk_local(local_top_left_px: I64Vec2, origin_bottom_left: I64Vec2) -> Self {
        Self {
            local: local_top_left_px,
            origin: origin_bottom_left,
        }
    }

    pub fn world_px(&self) -> I64Vec2 {
        self.origin + I64Vec2::new(self.local.x, -self.local.y)
    }

    pub fn world_tile(&self, grid: i64) -> I64Vec2 {
        let p = self.world_px();
        I64Vec2::new(p.x.div_euclid(grid), p.y.div_euclid(grid))
    }

    pub fn tile_offset_px(&self, grid: i64) -> I64Vec2 {
        let p = self.world_px();
        I64Vec2::new(p.x.rem_euclid(grid), p.y.rem_euclid(grid))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LdtkLevelTransform {
    pub world_x: i64,
    pub world_y: i64,
    pub px_wid: i64,
    pub px_hei: i64,
}

impl LdtkLevelTransform {
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
