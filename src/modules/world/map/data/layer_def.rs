prelude!();

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LayerType {
    Unknown,
    Tile,
    Entity,
    IntGrid,
    AutoLayer,
}

impl LayerType {
    pub fn from_key(s: String) -> Option<Self> {
        match s.as_str() {
            "Tiles" => Some(LayerType::Tile),
            "Entities" => Some(LayerType::Entity),
            "IntGrid" => Some(LayerType::IntGrid),
            "AutoLayer" => Some(LayerType::AutoLayer),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerDef {
    pub identifier: String,
    pub layer_type: LayerType,
    pub grid_size: i64,
    pub z: f32,
}

impl LayerDef {
    pub fn from_instance(instance: &LayerDefinition) -> Self {
        let layer_type = LayerType::from_key(instance.layer_definition_type.clone())
            .unwrap_or(LayerType::Unknown);

        LayerDef {
            identifier: instance.identifier.clone(),
            layer_type,
            grid_size: instance.grid_size,
            z: LayerDef::z_from_instance(instance) as f32,
        }
    }

    fn z_from_instance(instance: &LayerDefinition) -> i32 {
        match instance.identifier.as_str() {
            "BG_TILES" => -8,
            "FG_TILES" => 8,
            _ => -8,
        }
    }
}
