use crate::{
    modules::{ChunkPosition, TilePosition},
    prelude::*,
};
use bevy::math::I64Vec2;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct FaceDirection(pub Direction);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Transform)]
pub struct GridPosition;

impl GridPosition {
    pub fn get_chunk(&self, pos: Vec2) -> ChunkPosition {
        ChunkPosition {
            x: (pos.x / PIXELS_PER_CHUNK).floor() as i64,
            y: (pos.y / PIXELS_PER_CHUNK).floor() as i64,
        }
    }

    pub fn get_position_in_chunk(&self, pos: Vec2) -> TilePosition {
        let chunk = self.get_chunk(pos);

        let chunk_origin = Vec2::new(
            chunk.x as f32 * PIXELS_PER_CHUNK,
            chunk.y as f32 * PIXELS_PER_CHUNK,
        );

        let local_pixel_x = (pos.x - chunk_origin.x).floor();
        let local_pixel_y = (pos.y - chunk_origin.y).floor();

        TilePosition {
            local_x: (local_pixel_x / TILE_SIZE).floor() as i64,
            local_y: (local_pixel_y / TILE_SIZE).floor() as i64,
            world_x: (pos.x / TILE_SIZE).floor() as i64,
            world_y: (pos.y / TILE_SIZE).floor() as i64,
        }
    }
}
