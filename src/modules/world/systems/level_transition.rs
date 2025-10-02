use ldtk_rust::Flag;

use crate::modules::world::*;

#[add_system(schedule = OnEnter(LevelTransitionState::FadingOut), plugin = WorldPlugin)]
fn fade_out(
    query: Query<&RpgWorld>,
    mut transition_state: ResMut<NextState<LevelTransitionState>>,
) {
    let Ok(world) = query.single() else {
        error!("Expected exactly one RpgWorld entity");
        return;
    };

    info!("Fading out for level transition...");
    if world.active_level_key.is_none() {
        info!("No active level, skipping fade out...");
        transition_state.set(LevelTransitionState::Loading);
        return;
    }

    transition_state.set(LevelTransitionState::Loading);
}

#[add_system(schedule = OnEnter(LevelTransitionState::Loading), plugin = WorldPlugin)]
fn transition(
    mut commands: Commands,
    mut query: Query<(Entity, &mut RpgWorld)>,
    mut transition_state: ResMut<NextState<LevelTransitionState>>,
) {
    let Ok((entity, mut world)) = query.single_mut() else {
        error!("Expected exactly one RpgWorld entity");
        return;
    };

    info!("Transitioning levels...");

    if world.active_level_key.is_some() {
        info!("Despawning current level...");
        commands.entity(entity).despawn_related::<Children>();
    }

    let Some(requested) = &world.level_request else {
        error!("Level transition state entered but no level request present");
        return;
    };

    world.active_level_key = Some(requested.clone());
    world.level_request = None;

    let Some(level) = world.get_active_level() else {
        panic!(
            "Level with IID {} not found in world",
            world
                .active_level_key
                .as_ref()
                .unwrap_or(&String::from("<unknown>"))
        );
    };

    let world_pos = level.transform.bottom_left_bevy().as_vec2();

    let mut nav: Nav = Nav::new(level);

    // Load tile layers
    for layer in level.tile_layers.values() {
        let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

        info!("Spawning tile layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = level.size.get_tilemap_size();

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
            transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
            ..Default::default()
        });
    }

    // Load auto layers
    for layer in level.auto_layers.values() {
        let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

        info!("Spawning auto layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = level.size.get_tilemap_size();

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
            transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
            ..Default::default()
        });
    }

    // Load auto layers
    for layer in level.int_grid_layers.values() {
        if layer.instance.auto_layer_tiles.is_empty() {
            continue;
        }

        let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

        info!("Spawning (auto) int grid layer: {}", layer.def.identifier);
        let texture = world.tilesets[&layer.tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: layer.def.grid_size as f32,
            y: layer.def.grid_size as f32,
        };

        let size = level.size.get_tilemap_size();

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
            transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
            ..Default::default()
        });
    }

    for layer in level
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

                let pos = level.grid.get_bevy_position(i as i64);
                nav.set_flag(pos, flag);
            });
    }

    for layer in level
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

                let pos = level.grid.get_bevy_position(i as i64);
                nav.set_flag(pos, NavFlag::Blocked);
            });
    }

    nav.bake_collision(level);
    commands.spawn((nav, ChildOf(entity)));

    // Load entities

    transition_state.set(LevelTransitionState::FadingIn);
}

#[add_system(schedule = OnEnter(LevelTransitionState::FadingIn), plugin = WorldPlugin)]
fn fade_in(query: Query<&RpgWorld>, mut game_state: ResMut<NextState<GameState>>) {
    let Ok(world) = query.single() else {
        error!("Expected exactly one RpgWorld entity");
        return;
    };

    info!("Fading in for level transition...");

    game_state.set(GameState::Playing);
}
