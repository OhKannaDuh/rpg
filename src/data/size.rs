use bevy::math::I64Vec2;
use bevy_ecs_tilemap::map::TilemapSize;

pub struct Size {
    width_in_tiles: i64,
    height_in_tiles: i64,
    grid_size: i64,
}

impl Size {
    pub fn new(width_in_tiles: i64, height_in_tiles: i64, grid_size: i64) -> Self {
        Self {
            width_in_tiles,
            height_in_tiles,
            grid_size,
        }
    }

    pub fn from_pixel_size(pixel_width: i64, pixel_height: i64, grid_size: i64) -> Self {
        Self {
            width_in_tiles: pixel_width / grid_size,
            height_in_tiles: pixel_height / grid_size,
            grid_size,
        }
    }

    pub fn tiles(&self) -> I64Vec2 {
        I64Vec2::new(self.width_in_tiles, self.height_in_tiles)
    }

    pub fn pixels(&self) -> I64Vec2 {
        I64Vec2::new(
            self.width_in_tiles * self.grid_size,
            self.height_in_tiles * self.grid_size,
        )
    }

    pub fn get_tilemap_size(&self) -> TilemapSize {
        TilemapSize {
            x: self.width_in_tiles as u32,
            y: self.height_in_tiles as u32,
        }
    }
}
