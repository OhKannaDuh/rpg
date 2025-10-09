prelude!();

use super::data::ChunkId;

#[derive(Message)]
pub struct ChunkLoaded {
    pub entity: Entity,
    pub chunk_id: ChunkId,
}
