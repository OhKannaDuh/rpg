use std::fmt::Display;

use bevy::{log::warn, platform::collections::HashMap};

use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ChunkId(pub String);

impl ChunkId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn from_level(level: &ldtk_rust::Level) -> Self {
        Self(level.iid.clone())
    }
}

impl Display for ChunkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition {
    pub local_x: i64,
    pub local_y: i64,
    pub world_x: i64,
    pub world_y: i64,
}

pub struct Chunk {
    pub name: String,
    pub id: ChunkId,
    pub size: Size,
    pub grid: Grid,
    pub transform: ChunkTransform,
    pub position: ChunkPosition,
    pub layer_groups: LayerGroups,
    // pub tile_layers: HashMap<String, TileLayer>,
    // pub entity_layers: HashMap<String, EntityLayer>,
    // pub int_grid_layers: HashMap<String, IntGridLayer>,
    // pub auto_layers: HashMap<String, AutoLayer>,
    pub entities: HashMap<String, LdtkEntity>,
    pub neighbor_chunks: HashSet<ChunkId>,
}

impl Chunk {
    pub fn from_instance(
        instance: &ldtk_rust::Level,
        project: &ldtk_rust::Project,
        layer_definitions: &HashMap<String, LayerDef>,
        entity_definitions: &HashMap<String, EntityDef>,
    ) -> Self {
        let mut layer_groups = LayerGroups::default();

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
                        layer_groups.tile_layers.insert(
                            layer.iid.clone(),
                            TileLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::Entity) => {
                        layer_groups.entity_layers.insert(
                            layer.iid.clone(),
                            EntityLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::IntGrid) => {
                        layer_groups.int_grid_layers.insert(
                            layer.iid.clone(),
                            IntGridLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    Some(LayerType::AutoLayer) => {
                        layer_groups.auto_layers.insert(
                            layer.iid.clone(),
                            AutoLayer::from_instance(layer, iid, def.clone()),
                        );
                    }
                    _ => {}
                }
            }
        }

        let mut entities: HashMap<String, LdtkEntity> = HashMap::new();
        for layer in layer_groups.entity_layers.values() {
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

                let chunk_iid = instance.iid.clone();
                let layer_iid = layer.instance.iid.clone();

                entities.insert(
                    layer_iid.clone(),
                    LdtkEntity::from_instance(entity, chunk_iid, layer_iid, def.clone()),
                );
            }
        }

        let transform = ChunkTransform::from_instance(instance);
        let pixel_position = transform.bottom_left_bevy().as_vec2();
        let position = ChunkPosition {
            x: (pixel_position.x / PIXELS_PER_CHUNK).floor() as i64,
            y: (pixel_position.y / PIXELS_PER_CHUNK).floor() as i64,
        };

        Chunk {
            name: instance.identifier.clone(),
            id: ChunkId::from_level(instance),
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
            transform,
            position,
            layer_groups,
            entities,
            neighbor_chunks: instance
                .neighbours
                .iter()
                .map(|n| ChunkId::new(n.level_iid.clone()))
                .collect(),
        }
    }
}
