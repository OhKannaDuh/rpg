module!(resources, components, events, assets);

use crate::modules::actor::player::*;
use crate::modules::actor::*;

prelude!();

pub fn manage_chunks_around_player(
    query: Single<(&GridPosition, &Transform, &CurrentWorldId), With<Player>>,
    chunk_positions: Res<ChunkPositionMap>,
    chunk_data: Res<ChunkDataCollection>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let (grid_position, transform, current_world_id) = query.into_inner();

    let Some(world_id) = &current_world_id.0 else {
        return;
    };

    let chunk_position = grid_position.get_chunk(transform);

    let mut ids = Vec::new();
    if let Some(id) = chunk_positions.get(world_id, &chunk_position.get_chunk_coords()) {
        ids.push(id.clone());

        if let Some(data) = chunk_data.0.get(id) {
            for neighbor in &data.neighbors {
                ids.push(neighbor.clone());
            }
        }
    }

    let deactivate = chunk_manager
        .loaded_chunks
        .difference(&ids.iter().cloned().collect())
        .cloned()
        .collect::<Vec<_>>();

    for id in ids {
        chunk_manager.queue_load(id);
    }

    for id in deactivate {
        chunk_manager.queue_unload(id);
    }
}

pub fn process_chunk_queues(
    world_root_query: Single<Entity, With<WorldRoot>>,
    chunk_data: Res<ChunkDataCollection>,
    tilesets: Res<Tilesets>,
    layer_defs: Res<LayerDefs>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut chunk_loaded: MessageWriter<ChunkLoaded>,
    mut commands: Commands,
) {
    // Send one of each per frame
    if let Some(chunk_id) = chunk_manager.load_queue.pop_front() {
        let world_root = world_root_query.into_inner();

        let Some(data) = chunk_data.0.get(&chunk_id) else {
            return;
        };

        let position = data.position.get_pixel_coords().as_vec2();

        let chunk_entity = commands
            .spawn((
                ChunkRoot,
                ChildOf(world_root),
                Name::new(format!("Chunk {}", data.id.0)),
                Transform::from_xyz(position.x, position.y, 0.0),
            ))
            .id();

        data.tile_layers.iter().for_each(|layer| {
            let Some(tileset) = tilesets.0.get(&layer.tileset_def_uid) else {
                return;
            };

            let Some(layer_def) = layer_defs.0.get(&layer.layer_def_uid) else {
                return;
            };

            let layer_entity = commands.spawn(ChildOf(chunk_entity)).id();

            let tilemap_size = TilemapSize {
                x: TILES_PER_CHUNK_U32,
                y: TILES_PER_CHUNK_U32,
            };

            let mut storage = TileStorage::empty(tilemap_size);

            layer.tiles.iter().for_each(|tile| {
                let pos = TilePos {
                    x: (tile.px[0] / TILE_SIZE_I64) as u32,
                    y: tilemap_size.y - 1 - (tile.px[1] / TILE_SIZE_I64) as u32,
                };

                let tile_entity = commands
                    .spawn(TileBundle {
                        position: pos,
                        tilemap_id: TilemapId(layer_entity),
                        texture_index: TileTextureIndex(tile.t as u32),
                        ..Default::default()
                    })
                    .id();

                storage.set(&pos, tile_entity);
            });

            let tile_size = TilemapTileSize {
                x: TILE_SIZE,
                y: TILE_SIZE,
            };

            commands.entity(layer_entity).insert(TilemapBundle {
                grid_size: tile_size.into(),
                tile_size,
                texture: TilemapTexture::Single(tileset.clone()),
                size: tilemap_size,
                storage,
                map_type: TilemapType::default(),
                anchor: TilemapAnchor::BottomLeft,
                transform: Transform::from_xyz(0.0, 0.0, layer_def.z),
                ..Default::default()
            });

            chunk_manager.associate_entity(chunk_id.clone(), chunk_entity);
        });

        info!("Spawning chunk {:?}", data.id);
        chunk_loaded.write(ChunkLoaded {
            entity: chunk_entity,
            chunk_id: chunk_id.clone(),
        });
    }

    if let Some(chunk_id) = chunk_manager.unload_queue.pop_front() {
        let Some(entity) = chunk_manager.chunk_entity_map.get(&chunk_id) else {
            return;
        };

        info!("Despawning chunk {:?}", chunk_id);
        commands.entity(*entity).despawn();
        chunk_manager.mark_unloaded(chunk_id.clone());
    }
}

pub fn update_world_bounds_on_player_world_id_change(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    query: Single<&CurrentWorldId, With<Player>>,
    mut world_bounds: ResMut<WorldBounds>,
) {
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    let current_world_id = query.into_inner();
    let Some(world_id) = &current_world_id.0 else {
        return;
    };

    if world_bounds.world_id == world_id.0 {
        return;
    }

    world_bounds.world_id = world_id.0.clone();

    let Some(ldtk_world) = map.project.worlds.iter().find(|w| w.iid == world_id.0) else {
        return;
    };

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    for level in &ldtk_world.levels {
        let x1 = level.world_x as f32;
        let x2 = x1 + level.px_wid as f32;

        let y_top_down = level.world_y as f32;
        let y_bot_down = y_top_down + level.px_hei as f32;

        let y_min_up = -y_bot_down;
        let y_max_up = -y_top_down;

        min_x = min_x.min(x1);
        max_x = max_x.max(x2);
        min_y = min_y.min(y_min_up);
        max_y = max_y.max(y_max_up);
    }

    world_bounds.width = max_x - min_x;
    world_bounds.height = max_y - min_y;

    world_bounds.width = max_x - min_x;
    world_bounds.height = max_y - min_y;
    world_bounds.center = Vec2::new((min_x + max_x) * 0.5, (min_y + max_y) * 0.5);

    info!(
        "World bounds updated {}, {} centered at {}, {}",
        world_bounds.width, world_bounds.height, world_bounds.center.x, world_bounds.center.y
    );
}
