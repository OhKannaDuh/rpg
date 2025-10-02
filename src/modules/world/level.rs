use bevy::{log::warn, platform::collections::HashMap};

use super::*;

#[derive(Event)]
#[add_event(plugin = WorldPlugin)]
pub struct LevelChangedEvent {
    pub from: Option<String>,
    pub to: String,
}

pub struct Level {
    pub identifier: String,
    pub iid: String,
    pub size: Size,
    pub grid: Grid,
    pub transform: LdtkLevelTransform,
    // pub world_origin: WorldOrigin,
    pub tile_layers: HashMap<String, TileLayer>,
    pub entity_layers: HashMap<String, EntityLayer>,
    pub int_grid_layers: HashMap<String, IntGridLayer>,
    pub auto_layers: HashMap<String, AutoLayer>,
    pub entities: HashMap<String, LdtkEntity>,
    pub neighbor_levels: HashMap<Direction, Vec<String>>,
}

impl Level {
    pub fn from_instance(
        instance: &ldtk_rust::Level,
        project: &ldtk_rust::Project,
        layer_definitions: &HashMap<String, LayerDef>,
        entity_definitions: &HashMap<String, EntityDef>,
    ) -> Self {
        let mut tile_layers = HashMap::new();
        let mut entity_layers = HashMap::new();
        let mut int_grid_layers = HashMap::new();
        let mut auto_layers = HashMap::new();

        if let Some(layers) = &instance.layer_instances {
            for layer in layers {
                let Some(def) = layer_definitions.get(&layer.identifier) else {
                    warn!(
                        "Could not find layer definition for layer: {}",
                        layer.identifier
                    );
                    continue;
                };

                let iid = instance.iid.clone();

                match LayerType::from_key(layer.layer_instance_type.clone()) {
                    Some(LayerType::Tile) => {
                        tile_layers.insert(
                            layer.iid.clone(),
                            TileLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::Entity) => {
                        entity_layers.insert(
                            layer.iid.clone(),
                            EntityLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::IntGrid) => {
                        int_grid_layers.insert(
                            layer.iid.clone(),
                            IntGridLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::AutoLayer) => {
                        auto_layers.insert(
                            layer.iid.clone(),
                            AutoLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    _ => {}
                }
            }
        }

        let mut entities: HashMap<String, LdtkEntity> = HashMap::new();
        for layer in entity_layers.values() {
            for entity in &layer.instance.entity_instances {
                let Some(def) = entity_definitions.get(&entity.identifier) else {
                    warn!(
                        " - - Could not find entity definition for entity: {}",
                        entity.identifier
                    );
                    continue;
                };

                info!(
                    " - - Loading entity: {} ({})",
                    entity.identifier, entity.iid
                );

                let level_iid = instance.iid.clone();
                let layer_iid = layer.instance.iid.clone();

                entities.insert(
                    layer_iid.clone(),
                    LdtkEntity::from_instance(entity, level_iid, layer_iid, def.clone()),
                );
            }
        }

        let mut neighbor_levels = HashMap::new();
        neighbor_levels.insert(Direction::North, vec![]);
        neighbor_levels.insert(Direction::East, vec![]);
        neighbor_levels.insert(Direction::South, vec![]);
        neighbor_levels.insert(Direction::West, vec![]);

        for neighbor in &instance.neighbours {
            if let Some(direction) = Direction::from_ldtk_neighbor(neighbor.dir.clone())
                && let Some(levels) = neighbor_levels.get_mut(&direction)
            {
                levels.push(neighbor.level_iid.clone());
            }
        }

        neighbor_levels.retain(|_, levels| !levels.is_empty());

        Level {
            identifier: instance.identifier.clone(),
            iid: instance.iid.clone(),
            size: Size::new(
                instance.px_wid / project.default_grid_size,
                instance.px_hei / project.default_grid_size,
                project.default_grid_size,
            ),
            grid: Grid::new(
                instance.px_wid / project.default_grid_size,
                instance.px_hei / project.default_grid_size,
                project.default_grid_size,
            ),
            transform: LdtkLevelTransform::from_instance(instance),
            tile_layers,
            entity_layers,
            int_grid_layers,
            auto_layers,
            entities,
            neighbor_levels,
        }
    }
}
