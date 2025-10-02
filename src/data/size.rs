use crate::{
    data::{BevyPosition, LdtkPosition},
    modules::Level,
};
use bevy::math::UVec2;
use bevy_ecs_tilemap::map::TilemapSize;

pub struct Size {
    width_in_tiles: u32,
    height_in_tiles: u32,
    grid_size: u32,
}

impl Size {
    pub fn new(width_in_tiles: u32, height_in_tiles: u32, grid_size: u32) -> Self {
        Self {
            width_in_tiles,
            height_in_tiles,
            grid_size,
        }
    }

    pub fn tiles(&self) -> UVec2 {
        UVec2::new(self.width_in_tiles, self.height_in_tiles)
    }

    pub fn pixels(&self) -> UVec2 {
        UVec2::new(
            self.width_in_tiles * self.grid_size,
            self.height_in_tiles * self.grid_size,
        )
    }

    pub fn get_tilemap_size(&self) -> TilemapSize {
        TilemapSize {
            x: self.width_in_tiles,
            y: self.height_in_tiles,
        }
    }

    pub fn get_ldtk_position_of_index(&self, index: u32) -> LdtkPosition {
        LdtkPosition::new(
            (index % self.width_in_tiles) as i64,
            (index / self.width_in_tiles) as i64,
        )
    }

    pub fn get_bevy_position_of_index(&self, index: u32, level: &Level) -> BevyPosition {
        let ldtk_pos = self.get_ldtk_position_of_index(index);
        ldtk_pos.to_bevy_position(level)
    }
}
