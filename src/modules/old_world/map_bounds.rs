use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct MapBounds {
    pub min: Vec2,
    pub max: Vec2,
}

impl MapBounds {
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.y >= self.min.y && p.x <= self.max.x && p.y <= self.max.y
    }

    pub fn clamp(&self, p: Vec2) -> Vec2 {
        Vec2::new(
            p.x.clamp(self.min.x, self.max.x),
            p.y.clamp(self.min.y, self.max.y),
        )
    }

    pub fn from_center_anchor(tile_dims: UVec2, tile: f32, map_center: Vec2) -> Self {
        let half = Vec2::new(tile_dims.x as f32 * tile, tile_dims.y as f32 * tile) * 0.5;
        Self {
            min: map_center - half,
            max: map_center + half,
        }
    }
}
