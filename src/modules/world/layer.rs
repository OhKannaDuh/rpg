use super::*;
use crate::prelude::*;
use ldtk_rust::{LayerDefinition, LayerInstance};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LayerType {
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

#[derive(Clone)]
pub struct LayerDef {
    pub identifier: String,
    pub layer_type: LayerType,
    pub grid_size: i64,
    pub z: i32,
}

impl LayerDef {
    pub fn from_instance(instance: &LayerDefinition) -> Option<Self> {
        let layer_type = LayerType::from_key(instance.layer_definition_type.clone())?;

        Some(LayerDef {
            identifier: instance.identifier.clone(),
            layer_type,
            grid_size: instance.grid_size,
            z: LayerDef::z_from_instance(instance),
        })
    }

    fn z_from_instance(instance: &LayerDefinition) -> i32 {
        match instance.identifier.as_str() {
            "BASE" => -10,
            "BG_AUTO_1" => -8,
            "BG_TILES_1" => -7,
            "BG_AUTO_2" => -6,
            "BG_TILES_2" => -5,
            "BG_AUTO_3" => -4,
            "BG_TILES_3" => -3,
            "BG_AUTO_4" => -2,
            "BG_TILES_4" => -1,

            "FG_AUTO_1" => 1,
            "FG_TILES_1" => 2,
            "FG_AUTO_2" => 3,
            "FG_TILES_2" => 4,
            "FG_AUTO_3" => 5,
            "FG_TILES_3" => 6,
            "FG_AUTO_4" => 7,
            "FG_TILES_4" => 8,

            _ => 0,
        }
    }
}

pub struct TileLayer {
    pub def: LayerDef,
    pub tileset_uid: i64,
    pub level_iid: String,
    pub instance: LayerInstance,
}

impl TileLayer {
    pub fn from_instance(instance: &LayerInstance, level_iid: String, def: LayerDef) -> Self {
        TileLayer {
            def,
            tileset_uid: instance.tileset_def_uid.unwrap(),
            level_iid,
            instance: instance.clone(),
        }
    }
}

pub struct EntityLayer {
    pub def: LayerDef,
    pub level_iid: String,
    pub instance: LayerInstance,
}

impl EntityLayer {
    pub fn from_instance(instance: &LayerInstance, level_iid: String, def: LayerDef) -> Self {
        EntityLayer {
            def,
            level_iid,
            instance: instance.clone(),
        }
    }
}

pub struct IntGridLayer {
    pub def: LayerDef,
    pub tileset_uid: i64,
    pub level_iid: String,
    pub instance: LayerInstance,
}

impl IntGridLayer {
    pub fn from_instance(instance: &LayerInstance, level_iid: String, def: LayerDef) -> Self {
        IntGridLayer {
            def,
            tileset_uid: instance.tileset_def_uid.unwrap_or(0),
            level_iid,
            instance: instance.clone(),
        }
    }
}

pub struct AutoLayer {
    pub def: LayerDef,
    pub tileset_uid: i64,
    pub level_iid: String,
    pub instance: LayerInstance,
}

impl AutoLayer {
    pub fn from_instance(instance: &LayerInstance, level_iid: String, def: LayerDef) -> Self {
        AutoLayer {
            def,
            tileset_uid: instance.tileset_def_uid.unwrap(),
            level_iid,
            instance: instance.clone(),
        }
    }
}
