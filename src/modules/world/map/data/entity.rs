prelude!();

use super::ChunkId;
use super::WorldId;
use crate::modules::world::map::EntityDef;
use crate::modules::world::map::EntityFieldType;
use bevy::math::I64Vec2;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapEntityId(pub String);

#[derive(Clone, Debug)]
pub enum MapEntityFieldValue {
    String(String),
    Integer(i64),
}

#[derive(Clone, Debug)]
pub struct MapEntityField {
    pub field_type: EntityFieldType,
    pub value: MapEntityFieldValue,
}

impl MapEntityField {
    pub fn as_str(&self) -> String {
        if let MapEntityFieldValue::String(s) = &self.value {
            s.clone()
        } else {
            String::new()
        }
    }
}

#[derive(Clone, Debug)]
pub struct MapEntity {
    pub id: MapEntityId,
    pub chunk_id: ChunkId,
    pub world_id: WorldId,
    pub layer_id: String,

    pub world_x: i64,
    pub world_y: i64,
    pub width_px: i64,
    pub height_px: i64,

    pub def: EntityDef,

    pub fields: HashMap<String, MapEntityField>,
}

impl MapEntity {
    pub fn from_toc(
        toc_data: &LdtkTocInstanceData,
        def: &EntityDef,
        worlds: &[ldtk_rust::World],
    ) -> MapEntity {
        let world_iid = toc_data.iids.world_iid.as_str();
        let Some(world) = worlds.iter().find(|w| w.iid == world_iid) else {
            panic!("World with iid {} not found", world_iid);
        };

        let level_iid = toc_data.iids.level_iid.as_str();
        let Some(level) = world.levels.iter().find(|l| l.iid == level_iid) else {
            panic!("Level with iid {} not found", level_iid);
        };

        let Some(layers) = level.layer_instances.as_ref() else {
            panic!(
                "Layers not found in level {}, in world {}",
                level_iid, world_iid
            );
        };

        let layer_iid = toc_data.iids.layer_iid.as_str();
        let Some(layer) = layers.iter().find(|l| l.iid == layer_iid) else {
            panic!("Layer with iid {} not found", layer_iid);
        };

        let entity_iid = toc_data.iids.entity_iid.as_str();
        let Some(instance) = layer.entity_instances.iter().find(|e| e.iid == entity_iid) else {
            panic!(
                "Entity with iid {} not found in layer {}",
                entity_iid, layer_iid
            );
        };

        let mut fields = HashMap::new();

        for field_instance in &instance.field_instances {
            let Some(field_def) = def.get_field_for(field_instance) else {
                panic!(
                    "Field {} not found in entity def {}",
                    field_instance.identifier, def.uid
                );
            };

            let Some(value_instance) = &field_instance.value else {
                panic!(
                    "No Value present for field {} on {}",
                    field_instance.identifier, entity_iid
                );
            };

            let value = match field_def.field_type {
                EntityFieldType::String => {
                    MapEntityFieldValue::String(value_instance.as_str().unwrap().to_string())
                }
                EntityFieldType::Integer => {
                    MapEntityFieldValue::Integer(value_instance.as_i64().unwrap())
                }
            };

            fields.insert(
                field_instance.identifier.clone(),
                MapEntityField {
                    field_type: field_def.field_type.clone(),
                    value,
                },
            );
        }

        MapEntity {
            id: MapEntityId(toc_data.iids.entity_iid.clone()),
            chunk_id: ChunkId(toc_data.iids.level_iid.clone()),
            world_id: WorldId(toc_data.iids.world_iid.clone()),
            layer_id: toc_data.iids.layer_iid.clone(),

            world_x: toc_data.world_x + (toc_data.wid_px / 2),
            world_y: -(toc_data.world_y) - (toc_data.hei_px / 2),
            width_px: toc_data.wid_px,
            height_px: toc_data.hei_px,

            def: def.clone(),

            fields,
        }
    }

    pub fn many_from_toc(
        entry: &LdtkTableOfContentEntry,
        entity_def: &EntityDef,
        worlds: &[ldtk_rust::World],
    ) -> Vec<MapEntity> {
        entry
            .instances_data
            .iter()
            .map(|inst| MapEntity::from_toc(inst, entity_def, worlds))
            .collect()
    }

    pub fn from_entity_instance(
        instance: &EntityInstance,
        chunk_id: ChunkId,
        world_id: WorldId,
        layer_id: String,
        def: &EntityDef,
    ) -> Self {
        let entity_iid = &instance.iid;
        let mut fields = HashMap::new();

        for field_instance in &instance.field_instances {
            let Some(field_def) = def.get_field_for(field_instance) else {
                panic!(
                    "Field {} not found in entity def {}",
                    field_instance.identifier, def.uid
                );
            };

            let Some(value_instance) = &field_instance.value else {
                panic!(
                    "No Value present for field {} on {}",
                    field_instance.identifier, entity_iid
                );
            };

            let value = match field_def.field_type {
                EntityFieldType::String => {
                    MapEntityFieldValue::String(value_instance.as_str().unwrap().to_string())
                }
                EntityFieldType::Integer => {
                    MapEntityFieldValue::Integer(value_instance.as_i64().unwrap())
                }
            };

            fields.insert(
                field_instance.identifier.clone(),
                MapEntityField {
                    field_type: field_def.field_type.clone(),
                    value,
                },
            );
        }

        MapEntity {
            id: MapEntityId(entity_iid.clone()),
            chunk_id,
            world_id,
            layer_id,
            world_x: instance.world_x.unwrap() + (instance.width / 2),
            world_y: -(instance.world_y.unwrap()) - (instance.height / 2),
            width_px: instance.width,
            height_px: instance.height,

            def: def.clone(),

            fields,
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width_px as f32, self.height_px as f32)
    }

    pub fn transform(&self) -> Transform {
        Transform::from_xyz(self.world_x as f32, self.world_y as f32, 0.0)
    }

    pub fn bottom_left(&self) -> I64Vec2 {
        I64Vec2::new(
            self.world_x - (self.width_px / 2),
            self.world_y - (self.height_px / 2),
        )
    }

    pub fn center(&self) -> I64Vec2 {
        I64Vec2::new(self.world_x, self.world_y)
    }
}
