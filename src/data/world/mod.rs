use crate::{data::config::*, modules::world::RpgWorld};
use bevy::{
    log::info,
    math::{IVec2, UVec2, Vec2},
    reflect::Reflect,
};
use ldtk_rust::{EntityInstance, Level};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Default)]
pub struct TileSize(pub UVec2);

impl TileSize {
    pub fn new(w: u32, h: u32) -> Self {
        TileSize(UVec2::new(w, h))
    }

    pub fn from_pixels(w: u32, h: u32) -> Self {
        TileSize(UVec2::new(w / TILE_SIZE_U32, h / TILE_SIZE_U32))
    }

    pub fn tiles(&self) -> UVec2 {
        self.0
    }

    pub fn pixels(&self) -> UVec2 {
        self.0 * UVec2::splat(TILE_SIZE_U32)
    }

    pub fn as_tilemap_size(&self) -> bevy_ecs_tilemap::map::TilemapSize {
        bevy_ecs_tilemap::map::TilemapSize {
            x: self.0.x,
            y: self.0.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect, Default)]
pub struct TilePosition(pub IVec2);

impl TilePosition {
    pub fn from_ldtk(entity: &EntityInstance, level: &Level) -> Self {
        let height = level.px_hei as u32 / TILE_SIZE_U32;

        let p = Vec2::new(entity.grid[0] as f32, entity.grid[1] as f32) / TILE_SIZE;
        let w = p.floor();

        TilePosition(IVec2::new(w.x as i32, height as i32 - w.y as i32 - 1))
    }

    pub fn from_bevy(x: i32, y: i32) -> Self {
        TilePosition(IVec2::new(x, y))
    }

    pub fn tiles(&self) -> IVec2 {
        self.0
    }

    pub fn pixels(&self) -> IVec2 {
        self.0 * IVec2::splat(TILE_SIZE_U32 as i32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MapLayer {
    // -4 to -1
    Background1,
    Background2,
    Background3,
    Background4,
    // 0 (Maybe dynamic if we ever have an entity that has a hitbox smaller than its sprite)
    Entities,
    // 1 to 4
    Foreground1,
    Foreground2,
    Foreground3,
    Foreground4,
}

impl MapLayer {
    pub fn z(&self) -> f32 {
        match self {
            MapLayer::Background1 => -4.0,
            MapLayer::Background2 => -3.0,
            MapLayer::Background3 => -2.0,
            MapLayer::Background4 => -1.0,
            MapLayer::Entities => 1.0,
            MapLayer::Foreground1 => 2.0,
            MapLayer::Foreground2 => 3.0,
            MapLayer::Foreground3 => 4.0,
            MapLayer::Foreground4 => 5.0,
        }
    }

    pub fn from_layer_key(key: String) -> Option<Self> {
        match key.to_ascii_uppercase().as_str() {
            "BACKGROUND1" => Some(MapLayer::Background1),
            "BACKGROUND2" => Some(MapLayer::Background2),
            "BACKGROUND3" => Some(MapLayer::Background3),
            "BACKGROUND4" => Some(MapLayer::Background4),
            "FOREGROUND1" => Some(MapLayer::Foreground1),
            "FOREGROUND2" => Some(MapLayer::Foreground2),
            "FOREGROUND3" => Some(MapLayer::Foreground3),
            "FOREGROUND4" => Some(MapLayer::Foreground4),
            _ => None,
        }
    }
}

pub enum LevelEntity {
    LevelTransition(LevelTransition), // MAP_TRANSITION
    LevelTransitionDestination(LevelTransitionDestination), // MAP_TRANSITION_DESTINATION
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelTransition {
    pub size: TileSize,
    pub position: TilePosition,
    pub destination: LevelTransitionDestination,
}

impl LevelTransition {
    pub fn new(entity: &EntityInstance, world: &RpgWorld, level: &Level) -> Option<Self> {
        if entity.identifier != ENTITY_MAP_TRANSITION_ID {
            info!("Entity is not a map transition");
            return None;
        }

        info!("Getting destination field");
        let field = entity
            .field_instances
            .iter()
            .find(|f| f.identifier == FIELD_DESTINATION_ID)?;

        info!("Parsing destination field");
        let obj = field.value.as_ref().and_then(|v| v.as_object())?;

        info!("Getting level iid");
        let level_iid = obj.get("levelIid").and_then(|v| v.as_str())?;

        info!("Getting layer iid");
        let layer_iid = obj.get("layerIid").and_then(|v| v.as_str())?;

        info!("Getting entity iid");
        let entity_iid = obj.get("entityIid").and_then(|v| v.as_str())?;

        info!("Getting level for destination entity");
        let level = world
            .ldtk_project
            .levels
            .iter()
            .find(|lvl| lvl.iid == level_iid)?;

        info!("Finding layer instance");
        let layers = level.layer_instances.as_deref().unwrap_or(&[]);
        let layer = layers.iter().find(|ly| ly.iid == layer_iid)?;

        info!("Finding destination entity instance");
        let dest_entity = layer
            .entity_instances
            .iter()
            .find(|e| e.iid == entity_iid)?;

        info!("Creating LevelTransitionDestination");
        let destination = LevelTransitionDestination::new(dest_entity, level)?;

        info!("Creating LevelTransition");
        Some(LevelTransition {
            size: TileSize::from_pixels(entity.width as u32, entity.height as u32),
            position: TilePosition::from_ldtk(entity, level),
            destination,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LevelTransitionDestination {
    pub position: TilePosition,
    pub level_iid: String,
}

impl LevelTransitionDestination {
    pub fn new(entity: &EntityInstance, level: &Level) -> Option<Self> {
        if entity.identifier != ENTITY_MAP_TRANSITION_DESTINATION_ID {
            return None;
        }

        Some(LevelTransitionDestination {
            position: TilePosition::from_ldtk(entity, level),
            level_iid: level.iid.clone(),
        })
    }
}
