use crate::{data::config::TILE_SIZE, prelude::*};

#[derive(Component, Debug, Clone, Default)]
pub struct NavGrid {
    pub size: UVec2,
    pub solid: Vec<bool>,
    pub origin: Vec2,
}

impl NavGrid {
    pub fn new(size: UVec2, solid: Vec<bool>, origin: Vec2) -> Self {
        NavGrid {
            size,
            solid,
            origin,
        }
    }

    pub fn origin_bottom_left_for_centered_level(size: UVec2, tile: f32) -> Vec2 {
        Vec2::new(-(size.x as f32) * tile * 0.5, -(size.y as f32) * tile * 0.5)
    }

    #[inline]
    fn idx(&self, x: u32, y: u32) -> usize {
        (y as usize) * (self.size.x as usize) + (x as usize)
    }

    #[inline]
    pub fn in_bounds(&self, t: IVec2) -> bool {
        t.x >= 0 && t.y >= 0 && (t.x as u32) < self.size.x && (t.y as u32) < self.size.y
    }

    #[inline]
    pub fn is_blocked(&self, t: IVec2) -> bool {
        !self.in_bounds(t) || self.solid[self.idx(t.x as u32, t.y as u32)]
    }
    #[inline]
    pub fn is_walkable(&self, t: IVec2) -> bool {
        !self.is_blocked(t)
    }
}
