prelude!();
module!(data);

use bevy::math::I64Vec2;
use std::collections::VecDeque;

#[derive(Resource, Clone, Default, Debug)]
pub struct ChunkDataCollection(pub HashMap<ChunkId, ChunkData>);

#[derive(Resource, Clone, Default, Debug)]
pub struct ChunkPositionMap(pub HashMap<WorldId, HashMap<I64Vec2, ChunkId>>);

impl ChunkPositionMap {
    pub fn insert(&mut self, world_id: &WorldId, position: I64Vec2, chunk_id: ChunkId) {
        self.0
            .entry(world_id.clone())
            .or_insert_with(HashMap::new)
            .insert(position, chunk_id);
    }

    pub fn get(&self, world_id: &WorldId, position: &I64Vec2) -> Option<&ChunkId> {
        self.0.get(world_id).and_then(|m| m.get(position))
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct ChunkManager {
    pub loaded_chunks: HashSet<ChunkId>,
    pub chunk_entity_map: HashMap<ChunkId, Entity>,
    pub load_queue: VecDeque<ChunkId>,
    pub unload_queue: VecDeque<ChunkId>,
}

impl ChunkManager {
    pub fn queue_load(&mut self, chunk_id: ChunkId) {
        if !self.loaded_chunks.contains(&chunk_id) && !self.load_queue.contains(&chunk_id) {
            self.load_queue.push_back(chunk_id);
        }
    }

    pub fn queue_unload(&mut self, chunk_id: ChunkId) {
        if self.loaded_chunks.contains(&chunk_id) && !self.unload_queue.contains(&chunk_id) {
            self.unload_queue.push_back(chunk_id);
        }
    }

    pub fn associate_entity(&mut self, chunk_id: ChunkId, entity: Entity) {
        self.chunk_entity_map.insert(chunk_id.clone(), entity);
        self.loaded_chunks.insert(chunk_id);
    }

    pub fn mark_unloaded(&mut self, chunk_id: ChunkId) {
        self.chunk_entity_map.remove(&chunk_id);
        self.loaded_chunks.remove(&chunk_id);
    }
}
