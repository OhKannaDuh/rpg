use crate::{modules::WorldPlugin, prelude::*};
use std::collections::VecDeque;

#[derive(Resource, Default)]
#[insert_resource(plugin = WorldPlugin)]
pub struct LevelManager {
    active: HashMap<String, Entity>,
    to_activate: VecDeque<String>,
    to_deactivate: VecDeque<String>,
}

impl LevelManager {
    pub fn is_active(&self, iid: &str) -> bool {
        self.active.contains_key(iid)
    }

    pub fn set_active(&mut self, iid: String, entity: Entity) {
        self.active.insert(iid, entity);
    }

    pub fn unset_active(&mut self, iid: impl Into<String>) -> Option<Entity> {
        let key = iid.into();
        self.active.remove(&key)
    }

    pub fn get_entity(&self, iid: impl Into<String>) -> Option<Entity> {
        let key = iid.into();
        self.active.get(&key).cloned()
    }

    pub fn active_level_iids(&self) -> impl ExactSizeIterator<Item = &String> {
        self.active.keys()
    }

    pub fn request_activate(&mut self, iid: impl Into<String>) {
        let key = iid.into();
        if self.is_active(&key) || self.to_activate.contains(&key) {
            return;
        }

        self.to_activate.push_back(key);
    }

    pub fn request_deactivate(&mut self, iid: impl Into<String>) {
        let key = iid.into();
        if !self.is_active(&key) || self.to_deactivate.contains(&key) {
            return;
        }

        self.to_deactivate.push_back(key);
    }

    pub fn to_activate(&mut self) -> Option<String> {
        self.to_activate.pop_front()
    }

    pub fn to_deactivate(&mut self) -> Option<String> {
        self.to_deactivate.pop_front()
    }
}
