prelude!();
use crate::modules::world::map::ChunkPosition;
use crate::modules::world::map::TilePosition;
use crate::modules::world::map::WorldId;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Transform)]
pub struct GridPosition;

impl GridPosition {
    pub fn get_chunk(&self, transform: &Transform) -> ChunkPosition {
        let world_pos = transform.translation.truncate();

        ChunkPosition {
            chunk_x: (world_pos.x / PIXELS_PER_CHUNK).floor() as i64,
            chunk_y: (world_pos.y / PIXELS_PER_CHUNK).floor() as i64,
            pixel_x: world_pos.x.floor() as i64,
            pixel_y: world_pos.y.floor() as i64,
        }
    }

    pub fn get_position_in_chunk(&self, transform: &Transform) -> TilePosition {
        let world_pos = transform.translation.truncate();
        let chunk = self.get_chunk(transform);

        let chunk_origin = Vec2::new(
            chunk.chunk_x as f32 * PIXELS_PER_CHUNK,
            chunk.chunk_y as f32 * PIXELS_PER_CHUNK,
        );

        let local_pixel_x = (world_pos.x - chunk_origin.x).floor();
        let local_pixel_y = (world_pos.y - chunk_origin.y).floor();

        TilePosition {
            local_x: (local_pixel_x / TILE_SIZE).floor() as i64,
            local_y: (local_pixel_y / TILE_SIZE).floor() as i64,
            world_x: (world_pos.x / TILE_SIZE).floor() as i64,
            world_y: (world_pos.y / TILE_SIZE).floor() as i64,
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Name, Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct GameEntity;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(GameEntity, GridPosition, CurrentWorldId)]
pub struct Actor;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct FootPosition(pub f32);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct CurrentWorldId(pub Option<WorldId>);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct ActorDebug;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct ActorDebugRoot(pub String);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(ActorDebug)]
pub struct ActorDebugResolver;
