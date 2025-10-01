use crate::{modules::world::ldtk::LdtkIidMap, prelude::*};

#[derive(Default, Component)]
pub struct LdtkMapConfig {
    ldtk_iid_map: LdtkIidMap,
    pub selected_level: usize,
}

impl LdtkMapConfig {
    pub fn new(ldtk_iid_map: LdtkIidMap) -> Self {
        Self {
            ldtk_iid_map,
            selected_level: 0,
        }
    }

    pub fn set_selected_level(&mut self, level: usize) {
        self.selected_level = level;
    }

    pub fn set_selected_level_by_iid(&mut self, iid: &str) {
        if let Some(&level) = self.ldtk_iid_map.0.get(iid) {
            self.selected_level = level;
        } else {
            warn!("Level IID '{}' not found in LdtkIidMap", iid);
        }
    }
}
