prelude!();

#[derive(Debug, Clone)]
pub enum EntityFieldType {
    String,
    Integer,
}

impl EntityFieldType {
    pub fn from_field_definition(field_def: &FieldDefinition) -> Self {
        match field_def.field_definition_type.as_str() {
            "String" => EntityFieldType::String,
            "Int" => EntityFieldType::Integer,
            _ => panic!("Unknown field type: {}", field_def.field_definition_type),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityFieldDef {
    pub identifier: String,
    pub uid: i64,
    pub field_type: EntityFieldType,
}

#[derive(Debug, Clone)]
pub struct EntityDef {
    pub identifier: String,
    pub uid: i64,
    pub fields: Vec<EntityFieldDef>,
}

impl EntityDef {
    pub fn from_instance(instance: &EntityDefinition) -> Self {
        let mut fields = Vec::new();

        for field in &instance.field_defs {
            let field_type = EntityFieldType::from_field_definition(field);

            fields.push(EntityFieldDef {
                identifier: field.identifier.clone(),
                uid: field.uid,
                field_type,
            });
        }

        Self {
            identifier: instance.identifier.clone(),
            uid: instance.uid,
            fields,
        }
    }

    pub fn get_field_for(&self, instance: &FieldInstance) -> Option<&EntityFieldDef> {
        self.fields.iter().find(|f| f.uid == instance.def_uid)
    }

    pub fn contains_field_for(&self, instance: &FieldInstance) -> bool {
        self.fields.iter().any(|f| f.uid == instance.def_uid)
    }
}
