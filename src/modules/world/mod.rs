mod ldtk;
pub mod nav_grid;
pub mod rpg_level;

use crate::{
    data::{
        config::{ENTITY_MAP_TRANSITION_ID, TILE_SIZE_U32},
        world::{LevelEntity, LevelTransition, MapLayer},
    },
    modules::world::{
        ldtk::*,
        rpg_level::{LevelChanged, RpgLevel},
    },
    prelude::*,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use ldtk_rust::Level;

#[derive(Debug, thiserror::Error)]
pub enum LoadLevelError {
    #[error("Level id {0} not found")]
    IdNotFound(usize),
    #[error("Level iid `{0}` not found")]
    IidNotFound(String),
    #[error("Tile size mismatch: level pixels ({w}px, {h}px) not divisible by TILE_SIZE={tile}")]
    TileSizeMismatch { w: u32, h: u32, tile: u32 },
}

#[derive(Component, Debug)]
pub struct RpgWorld {
    pub ldtk_project: ldtk_rust::Project,
    pub ldtk_tilesets: HashMap<i64, Handle<Image>>,
    pub level_iid_map: HashMap<String, usize>,
    pub active_level: Option<RpgLevel>,
}

impl RpgWorld {
    pub fn load_level_from_id(&mut self, id: usize) -> Result<&RpgLevel, LoadLevelError> {
        let level = self
            .ldtk_project
            .levels
            .get(id)
            .ok_or(LoadLevelError::IdNotFound(id))?;

        self.load_level_from_ldtk(level.clone())
    }

    pub fn load_level_from_iid(&mut self, iid: &str) -> Result<&RpgLevel, LoadLevelError> {
        let id = *self
            .level_iid_map
            .get(iid)
            .ok_or_else(|| LoadLevelError::IidNotFound(iid.to_string()))?;

        self.load_level_from_id(id)
    }

    fn load_level_from_ldtk(&mut self, level: Level) -> Result<&RpgLevel, LoadLevelError> {
        self.active_level = Some(RpgLevel::new(self, level));

        Ok(self.active_level.as_ref().unwrap())
    }

    pub fn tile_to_pixels(&self, tile: IVec2) -> IVec2 {
        (tile * IVec2::splat(TILE_SIZE_U32 as i32)) + IVec2::splat(TILE_SIZE_U32 as i32 / 2)
    }
}

#[derive(AssetCollection, Resource)]
pub struct WorldAssets {
    #[asset(path = "world/world.ldtk")]
    pub ldtk_project: Handle<LdtkMap>,
}
#[add_plugin(to_group = WorldPlugins)]
pub struct WorldPlugin;

#[butler_plugin]
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LdtkMap>()
            .register_asset_loader(LdtkLoader)
            .add_plugins(TilemapPlugin);

        app.configure_loading_state(
            LoadingStateConfig::new(RpgState::Loading).load_collection::<WorldAssets>(),
        );
    }
}

#[add_system(schedule = OnEnter(RpgState::AssetsLoaded), plugin = WorldPlugin)]
fn setup_world(mut commands: Commands, world: Res<WorldAssets>, maps: Res<Assets<LdtkMap>>) {
    let Some(map) = maps.get(&world.ldtk_project) else {
        error!("LDtk handle present but asset not loaded yet");
        return;
    };

    let iid_map: HashMap<String, usize> = map
        .project
        .levels
        .iter()
        .enumerate()
        .map(|(i, level)| (level.iid.clone(), i))
        .collect();

    commands.spawn((
        RpgWorld {
            ldtk_project: map.project.clone(),
            ldtk_tilesets: map.tilesets.clone(),
            level_iid_map: iid_map,
            active_level: None,
        },
        InheritedVisibility::default(),
        GlobalTransform::default(),
    ));
}

#[add_system(schedule = OnEnter(RpgState::AssetsLoaded), plugin = WorldPlugin, after = setup_world)]
fn load_initial_level(
    mut world_query: Query<&mut RpgWorld>,
    mut level_changed_events: EventWriter<LevelChanged>,
    mut next_state: ResMut<NextState<RpgState>>,
) {
    let Ok(mut world) = world_query.single_mut() else {
        warn!("No RpgWorld entity found");
        return;
    };

    if let Err(e) = world.load_level_from_id(0) {
        error!("Failed to load initial level: {e}");
    } else {
        info!("Sending level changed event");
        level_changed_events.write(LevelChanged);
    }

    next_state.set(RpgState::WorldLoaded);
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn load_active_map(
    mut commands: Commands,
    world_query: Query<(Entity, &RpgWorld)>,
    mut level_changed_events: EventReader<LevelChanged>,
) {
    let Ok((entity, world)) = world_query.single() else {
        warn!("No RpgWorld entity found");
        return;
    };

    let Some(rpg_level) = &world.active_level else {
        warn!("No active level found");
        return;
    };

    for _ in level_changed_events.read() {
        commands.entity(entity).despawn_related::<Children>();

        let mut tilesets = HashMap::new();
        for def in &world.ldtk_project.defs.tilesets {
            if let Some(tex) = world.ldtk_tilesets.get(&def.uid) {
                tilesets.insert(def.uid, (tex.clone(), def));
            }
        }

        // Tileset layers
        for (_layer_id, layer) in rpg_level
            .level
            .layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .enumerate()
        {
            let Some(uid) = layer.tileset_def_uid else {
                continue;
            };

            let key = layer.identifier.clone();
            let Some(map_layer) = MapLayer::from_layer_key(key) else {
                continue;
            };

            let (texture, _tileset) = tilesets[&uid].clone();

            let grid = layer.grid_size as u32;

            let size = TilemapSize {
                x: (rpg_level.level.px_wid as u32 / grid),
                y: (rpg_level.level.px_hei as u32 / grid),
            };

            let tile_size = TilemapTileSize {
                x: grid as f32,
                y: grid as f32,
            };

            let map_entity = commands.spawn(ChildOf(entity)).id();
            let mut storage = TileStorage::empty(size);

            let h = size.y;
            for tile in layer.grid_tiles.iter().chain(layer.auto_layer_tiles.iter()) {
                let x = (tile.px[0] as u32) / grid;
                let y = h - 1 - (tile.px[1] as u32) / grid;
                let pos = TilePos { x, y };

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: pos,
                        tilemap_id: TilemapId(map_entity),
                        texture_index: TileTextureIndex(tile.t as u32),
                        ..Default::default()
                    })
                    .id();
                storage.set(&pos, tile_entity);
            }

            commands.entity(map_entity).insert(TilemapBundle {
                grid_size: tile_size.into(),
                tile_size,
                texture: TilemapTexture::Single(texture),
                size,
                storage,
                anchor: TilemapAnchor::Center,
                map_type: TilemapType::default(),
                global_transform: GlobalTransform::from_xyz(0.0, 0.0, map_layer.z()),
                transform: Transform::from_xyz(0.0, 0.0, map_layer.z()),
                ..Default::default()
            });
        }

        info!("Active level changed");
    }
}
