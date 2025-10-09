prelude!();

use crate::modules::world::map::*;

#[derive(Clone, Debug)]
pub struct TileLayer {
    pub tileset_def_uid: i64,
    pub layer_def_uid: i64,
    pub tiles: Vec<TileInstance>,
}

impl TileLayer {
    pub fn is(layer: &LayerInstance) -> bool {
        !layer.grid_tiles.is_empty() || !layer.auto_layer_tiles.is_empty()
    }

    pub fn from_instance(layer: &LayerInstance) -> Self {
        Self {
            tileset_def_uid: layer.tileset_def_uid.unwrap_or(-1),
            layer_def_uid: layer.layer_def_uid,
            tiles: layer
                .grid_tiles
                .iter()
                .chain(layer.auto_layer_tiles.iter())
                .cloned()
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct EntityLayer {
    pub entities: HashMap<MapEntityId, MapEntity>,
}

impl EntityLayer {
    pub fn is(layer: &LayerInstance) -> bool {
        !layer.entity_instances.is_empty()
    }

    pub fn from_instance(
        layer: &LayerInstance,
        world_id: &WorldId,
        chunk_id: &ChunkId,
        entity_defs: &EntityDefs,
    ) -> Self {
        let layer_id = layer.iid.clone();
        let mut entities: HashMap<MapEntityId, MapEntity> = HashMap::new();

        for entity_instance in layer.entity_instances.iter() {
            info!(
                "Found entity {} ({}) in layer {}",
                entity_instance.iid, entity_instance.def_uid, layer.iid
            );

            let Some(def) = entity_defs.by_uid(entity_instance.def_uid) else {
                continue;
            };

            entities.insert(
                MapEntityId(entity_instance.iid.clone()),
                MapEntity::from_entity_instance(
                    entity_instance,
                    chunk_id.clone(),
                    world_id.clone(),
                    layer_id.clone(),
                    def,
                ),
            );
        }

        Self { entities }
    }
}

#[derive(Clone, Debug)]
pub struct DataLayer {}

impl DataLayer {
    pub fn is(layer: &LayerInstance) -> bool {
        layer.int_grid.is_some() && !TileLayer::is(layer)
    }

    pub fn from_instance(layer: &LayerInstance) -> Self {
        Self {}
    }
}
