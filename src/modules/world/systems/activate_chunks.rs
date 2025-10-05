use crate::components::*;
use crate::modules::world::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(GameState::Playing))]
fn active_chunks(
    world_query: Query<&RpgWorld>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let Some(chunk_iid) = chunk_manager.to_activate() else {
        return;
    };

    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld found");
        return;
    };

    let Some(data) = world.chunks.get(&chunk_iid) else {
        warn!("Chunk ID not found in world: {}", chunk_iid);
        return;
    };

    let mut nav: Nav = Nav::new(data);

    let world_pos = data.transform.bottom_left_bevy().as_vec2();
    let chunk_entity = commands
        .spawn((
            ChunkRoot {
                id: chunk_iid.clone(),
            },
            Name::new(format!("Chunk {}", chunk_iid)),
            Transform::from_xyz(world_pos.x, world_pos.y, 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
        ))
        .id();

    data.layer_groups.iter_tile_layers().for_each(|layer| {
        let map_entity: Entity = commands.spawn(ChildOf(chunk_entity)).id();
        let def = layer.get_def();
        let tileset_uid = layer.get_tileset_uid();
        if tileset_uid == 0 {
            return;
        }

        info!("Spawning tile layer: {}", def.identifier);
        let texture = world.tilesets[&tileset_uid].clone();
        let tile_size = TilemapTileSize {
            x: def.grid_size as f32,
            y: def.grid_size as f32,
        };

        let size = data.size.get_tilemap_size();

        let mut storage = TileStorage::empty(size);

        layer.iter_tiles().for_each(|tile| {
            let pos = TilePos {
                x: (tile.px[0] / def.grid_size) as u32,
                y: size.y - 1 - (tile.px[1] / def.grid_size) as u32,
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
            transform: Transform::from_xyz(0.0, 0.0, def.z as f32),
            ..Default::default()
        });
    });

    for layer in data.layer_groups.int_grid_layers.values() {
        if layer.def.identifier == "TILE_FLAGS" {
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
        } else {
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
    }

    nav.bake_collision();
    commands.spawn((nav, ChildOf(chunk_entity)));

    info!("Activating level: {}", chunk_iid);
    chunk_manager.set_active(chunk_iid.clone(), chunk_entity);
}
