prelude!();
module!(assets, resources, data, components);

pub fn populate_chunk_position_map(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut chunk_position_map: ResMut<ChunkPositionMap>,
) {
    info!("Populating chunk position map...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for world in &map.project.worlds {
        let world_id = WorldId(world.iid.clone());
        for level in &world.levels {
            let chunk_position = ChunkPosition::from_level(level);

            debug!(
                "Chunk {} at {}, {} in {}",
                level.iid, chunk_position.chunk_x, chunk_position.chunk_y, world.iid
            );

            chunk_position_map.insert(
                &world_id,
                chunk_position.get_chunk_coords(),
                ChunkId(level.iid.clone()),
            );
        }
    }
}

pub fn populate_chunk_data_collection(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut chunk_data_collection: ResMut<ChunkDataCollection>,
    tileset_defs: Res<TilesetDefs>,
    entity_defs: Res<EntityDefs>,
) {
    info!("Populating chunk data collection...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for world in &map.project.worlds {
        let world_id = WorldId(world.iid.clone());
        debug!(
            "Processing world data: {} ({})",
            world.iid, world.identifier
        );

        for level in &world.levels {
            debug!(
                "Processing level data: {} ({}) in {}",
                level.iid, level.identifier, world.iid
            );
            let chunk = ChunkData::from_level(&world_id, level, &tileset_defs, &entity_defs);
            chunk_data_collection.0.insert(chunk.id.clone(), chunk);
        }
    }
}

pub fn spawn_world_root(mut commands: Commands) {
    commands.spawn((WorldRoot, Name::new("World Root")));
}

pub fn populate_global_map_entities(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut global_map_entities: ResMut<GlobalMapEntities>,
    defs: Res<EntityDefs>,
) {
    info!("Populating global map entities...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    let worlds = &map.project.worlds;

    for entry in &map.project.toc {
        let Some(def) = defs.by_identifier(entry.identifier.clone()) else {
            warn!(
                "No entity definition found for identifier: {}",
                entry.identifier
            );
            continue;
        };

        let mut count = 0;
        for entity in MapEntity::many_from_toc(entry, def, worlds) {
            global_map_entities.add(entity);
            count += 1;
        }

        info!(
            "Added {} global entiies of type {}",
            count, entry.identifier
        );
    }
}
