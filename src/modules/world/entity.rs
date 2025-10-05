use ldtk_rust::{EntityDefinition, EntityInstance};

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
    pub chunk_iid: String,
    pub layer_iid: String,
    pub def: EntityDef,
    // pub ldtk_tile_position: LdtkPosition,
}

impl LdtkEntity {
    pub fn from_instance(
        instance: &EntityInstance,
        chunk_iid: String,
        layer_iid: String,
        def: EntityDef,
    ) -> Self {
        Self {
            iid: instance.iid.clone(),
            chunk_iid,
            layer_iid,
            def,
            // ldtk_tile_position: LdtkPosition::from_entity_instance(instance),
        }
    }
}
