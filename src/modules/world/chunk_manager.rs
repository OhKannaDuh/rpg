use crate::{
    modules::{ChunkId, WorldPlugin},
    prelude::*,
};
use std::collections::VecDeque;

#[derive(Resource, Default)]
#[insert_resource(plugin = WorldPlugin)]
pub struct ChunkManager {
    active: HashMap<ChunkId, Entity>,
    to_activate: VecDeque<ChunkId>,
    to_deactivate: VecDeque<ChunkId>,
}

impl ChunkManager {
    pub fn is_active(&self, id: &ChunkId) -> bool {
        self.active.contains_key(id)
    }

    pub fn set_active(&mut self, id: ChunkId, entity: Entity) {
        self.active.insert(id, entity);
    }

    pub fn unset_active(&mut self, id: impl Into<ChunkId>) -> Option<Entity> {
        let key = id.into();
        self.active.remove(&key)
    }

    pub fn get_entity(&self, id: impl Into<ChunkId>) -> Option<Entity> {
        let key = id.into();
        self.active.get(&key).cloned()
    }

    pub fn active_chunk_ids(&self) -> impl ExactSizeIterator<Item = &ChunkId> {
        self.active.keys()
    }

    pub fn request_activate(&mut self, id: impl Into<ChunkId>) {
        let key = id.into();
        if self.is_active(&key) || self.to_activate.contains(&key) {
            return;
        }

        self.to_activate.push_back(key);
    }

    pub fn request_deactivate(&mut self, id: impl Into<ChunkId>) {
        let key = id.into();
        if !self.is_active(&key) || self.to_deactivate.contains(&key) {
            return;
        }

        self.to_deactivate.push_back(key);
    }

    pub fn to_activate(&mut self) -> Option<ChunkId> {
        self.to_activate.pop_front()
    }

    pub fn to_deactivate(&mut self) -> Option<ChunkId> {
        self.to_deactivate.pop_front()
    }
}
