use crate::modules::Level;

pub struct LdtkPosition {
    x: i64,
    y: i64,
}

impl LdtkPosition {
    pub fn new(ldtk_origin_x: i64, ldtk_origin_y: i64) -> Self {
        Self {
            x: ldtk_origin_x,
            y: ldtk_origin_y,
        }
    }

    pub fn from_level_instance(instance: &ldtk_rust::Level, project: &ldtk_rust::Project) -> Self {
        Self {
            x: instance.world_x / project.default_grid_size,
            y: instance.world_y / project.default_grid_size,
        }
    }

    pub fn from_entity_instance(instance: &ldtk_rust::EntityInstance) -> Self {
        Self {
            x: instance.grid[0],
            y: instance.grid[1],
        }
    }

    pub fn to_bevy_position(&self, level: &Level) -> BevyPosition {
        BevyPosition {
            x: self.x,
            y: (level.size.tiles().y as i64 - self.y - 1),
        }
    }
}

pub struct BevyPosition {
    pub x: i64,
    pub y: i64,
}
