// use ldtk_rust::Flag;

// use crate::{components::*, modules::world::*};

use crate::components::*;
use crate::modules::world::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::Playing))]
fn activate_levels(
    world_query: Query<&RpgWorld>,
    mut level_manager: ResMut<LevelManager>,
    mut commands: Commands,
) {
    let Some(level_iid) = level_manager.to_activate() else {
        return;
    };

    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found");
        return;
    };

    let Some(data) = world.levels.get(&level_iid) else {
        warn!("Level ID not found in world: {}", level_iid);
        return;
    };

    let mut nav: Nav = Nav::new(data);

    let world_pos = data.transform.bottom_left_bevy().as_vec2();
    let level_entity = commands
        .spawn((
            LevelRoot {
                iid: level_iid.clone(),
            },
            Name::new(format!("Level {}", level_iid)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
        ))
        .id();

    for layer in data.tile_layers.values() {
        let map_entity: Entity = commands.spawn(ChildOf(level_entity)).id();

        info!("Spawning tile layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = data.size.get_tilemap_size();

        let mut storage = TileStorage::empty(size);

        layer.instance.grid_tiles.iter().for_each(|tile| {
            let pos = TilePos {
                x: (tile.px[0] / layer.def.grid_size) as u32,
                y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: pos,
                    tilemap_id: TilemapId(map_entity),
                    texture_index: TileTextureIndex(tile.t as u32),
                    ..Default::default()
                })
                .id();

            storage.set(&pos, tile_entity);
        });

        commands.entity(map_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            tile_size,
            texture: TilemapTexture::Single(texture),
            size,
            storage,
            map_type: TilemapType::default(),
            anchor: TilemapAnchor::BottomLeft,
            transform: Transform::from_xyz(0.0, 0.0, layer.def.z as f32),
            ..Default::default()
        });
    }

    // Load auto layers
    for layer in data.auto_layers.values() {
        let map_entity: Entity = commands.spawn(ChildOf(level_entity)).id();

        info!("Spawning auto layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = data.size.get_tilemap_size();

        let mut storage = TileStorage::empty(size);

        layer.instance.auto_layer_tiles.iter().for_each(|tile| {
            let pos = TilePos {
                x: (tile.px[0] / layer.def.grid_size) as u32,
                y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: pos,
                    tilemap_id: TilemapId(map_entity),
                    texture_index: TileTextureIndex(tile.t as u32),
                    ..Default::default()
                })
                .id();

            storage.set(&pos, tile_entity);
        });

        commands.entity(map_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            tile_size,
            texture: TilemapTexture::Single(texture),
            size,
            storage,
            map_type: TilemapType::default(),
            anchor: TilemapAnchor::BottomLeft,
            transform: Transform::from_xyz(0.0, 0.0, layer.def.z as f32),
            ..Default::default()
        });
    }

    // Load auto layers
    for layer in data.int_grid_layers.values() {
        if layer.instance.auto_layer_tiles.is_empty() {
            continue;
        }

        let map_entity: Entity = commands.spawn(ChildOf(level_entity)).id();

        info!("Spawning (auto) int grid layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = data.size.get_tilemap_size();

        let mut storage = TileStorage::empty(size);

        layer.instance.auto_layer_tiles.iter().for_each(|tile| {
            let pos = TilePos {
                x: (tile.px[0] / layer.def.grid_size) as u32,
                y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: pos,
                    tilemap_id: TilemapId(map_entity),
                    texture_index: TileTextureIndex(tile.t as u32),
                    ..Default::default()
                })
                .id();

            storage.set(&pos, tile_entity);
        });

        commands.entity(map_entity).insert(TilemapBundle {
            grid_size: tile_size.into(),
            tile_size,
            texture: TilemapTexture::Single(texture),
            size,
            storage,
            map_type: TilemapType::default(),
            anchor: TilemapAnchor::BottomLeft,
            transform: Transform::from_xyz(0.0, 0.0, layer.def.z as f32),
            ..Default::default()
        });
    }

    for layer in data
        .int_grid_layers
        .values()
        .filter(|l| l.def.identifier != "TILE_FLAGS")
    {
        layer
            .instance
            .int_grid_csv
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                let Some(flag) = NavFlag::from_id(*v) else {
                    return;
                };

                let local = data.grid.get_bevy_position(i as i64);
                nav.set_flag(local.x, local.y, flag);
            });
    }

    for layer in data
        .int_grid_layers
        .values()
        .filter(|l| l.def.identifier == "TILE_FLAGS")
    {
        layer
            .instance
            .int_grid_csv
            .iter()
            .enumerate()
            .for_each(|(i, v)| {
                if *v != 1 {
                    return;
                }

                let local = data.grid.get_bevy_position(i as i64);
                nav.set_flag(local.x, local.y, NavFlag::Blocked);
            });
    }

    nav.bake_collision();
    commands.spawn((nav, ChildOf(level_entity)));

    info!("Activating level: {}", level_iid);
    level_manager.set_active(level_iid.clone(), level_entity);
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::Playing))]
fn deactivate_levels(
    world_query: Query<&RpgWorld>,
    mut level_manager: ResMut<LevelManager>,
    mut commands: Commands,
) {
    let Some(level_iid) = level_manager.to_deactivate() else {
        return;
    };

    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found");
        return;
    };

    let iid = level_iid.clone();
    let Some(entity) = level_manager.get_entity(iid) else {
        warn!("Level entity not found");
        return;
    };

    commands.entity(entity).despawn();

    info!("Deactivating level: {}", level_iid);
    level_manager.unset_active(level_iid);
}
