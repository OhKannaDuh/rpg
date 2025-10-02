use ldtk_rust::{EntityDefinition, EntityInstance};

use crate::data::LdtkPosition;

#[derive(Clone)]
pub enum EntityFieldType {
    EntityRef,
}

#[derive(Clone)]
pub struct EntityFieldDef {
    pub identifier: String,
    pub field_type: EntityFieldType,
}

#[derive(Clone)]
pub struct EntityDef {
    pub identifier: String,
    pub fields: Vec<EntityFieldDef>,
}

impl EntityDef {
    pub fn from_instance(instance: &EntityDefinition) -> Self {
        let mut fields = Vec::new();

        for field in &instance.field_defs {
            let field_type = match field.field_definition_type.as_str() {
                "EntityRef" => EntityFieldType::EntityRef,
                _ => continue,
            };

            fields.push(EntityFieldDef {
                identifier: field.identifier.clone(),
                field_type,
            });
        }

        Self {
            identifier: instance.identifier.clone(),
            fields,
        }
    }
}

pub struct LdtkEntity {
    pub iid: String,
    pub level_iid: String,
    pub layer_iid: String,
    pub def: EntityDef,
    pub ldtk_tile_position: LdtkPosition,
}

impl LdtkEntity {
    pub fn from_instance(
        instance: &EntityInstance,
        level_iid: String,
        layer_iid: String,
        def: EntityDef,
    ) -> Self {
        Self {
            iid: instance.iid.clone(),
            level_iid,
            layer_iid,
            def,
            ldtk_tile_position: LdtkPosition::from_entity_instance(instance),
        }
    }
}

// {
//     "__identifier": "MAP_TRANSITION",
//     "__grid": [0,24],
//     "__pivot": [0,0],
//     "__tags": [],
//     "__tile": null,
//     "__smartColor": "#BE4A2F",
//     "iid": "bc481060-8560-11f0-b1b0-c5fad159b130",
//     "width": 16,
//     "height": 112,
//     "defUid": 8,
//     "px": [0,384],
//     "fieldInstances": [{ "__identifier": "DESTINATION", "__type": "EntityRef", "__value": {
//         "entityIid": "0f9d8790-8560-11f0-b1b0-f1bc4d00ae2a",
//         "layerIid": "b6b36230-8560-11f0-b1b0-af974e18ee9e",
//         "levelIid": "33fbe700-8560-11f0-b1b0-11ccddd48483",
//         "worldIid": "fe4e52e1-8560-11f0-975a-73522ff61829"
//     }, "__tile": null, "defUid": 12, "realEditorValues": [{
//         "id": "V_String",
//         "params": ["0f9d8790-8560-11f0-b1b0-f1bc4d00ae2a"]
//     }] }],
//     "__worldX": 0,
//     "__worldY": 384
// },
