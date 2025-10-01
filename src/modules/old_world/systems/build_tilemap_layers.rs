use crate::modules::world::ldtk::*;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn build_tilemap_layers(
    mut commands: Commands,
    mut map_changed: EventReader<MapAssetChanged>,
    maps: Res<Assets<LdtkMap>>,
    mut q_maps: Query<(Entity, &LdtkMapHandle, &LdtkMapConfig)>,
) {
    for MapAssetChanged(changed_id) in map_changed.read().copied() {
        info!("Building tilemap layers for changed map: {:?}", changed_id);
        for (entity, handle, cfg) in q_maps.iter_mut() {
            if handle.0.id() != changed_id {
                continue;
            }

            let Some(ldtk_map) = maps.get(&handle.0) else {
                continue;
            };

            let mut tilesets = HashMap::new();
            for def in &ldtk_map.project.defs.tilesets {
                if let Some(tex) = ldtk_map.tilesets.get(&def.uid) {
                    tilesets.insert(def.uid, (tex.clone(), def));
                }
            }

            let default_grid = ldtk_map.project.default_grid_size;
            let level = &ldtk_map.project.levels[cfg.selected_level];
            let size = TilemapSize {
                x: (level.px_wid / default_grid) as u32,
                y: (level.px_hei / default_grid) as u32,
            };

            for (layer_id, layer) in level
                .layer_instances
                .as_ref()
                .unwrap()
                .iter()
                .rev()
                .enumerate()
            {
                if let Some(uid) = layer.tileset_def_uid {
                    let (texture, tileset) = tilesets[&uid].clone();
                    let tile_size = TilemapTileSize {
                        x: tileset.tile_grid_size as f32,
                        y: tileset.tile_grid_size as f32,
                    };

                    let map_entity = commands.spawn(ChildOf(entity)).id();
                    let mut storage = TileStorage::empty(size);

                    // let w: u32 = size.x;
                    let h = size.y;
                    for tile in layer.grid_tiles.iter().chain(layer.auto_layer_tiles.iter()) {
                        let x = (tile.px[0] / default_grid) as u32;
                        let y = h - 1 - (tile.px[1] / default_grid) as u32;
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
                        map_type: TilemapType::default(),
                        anchor: TilemapAnchor::Center,
                        transform: Transform::from_xyz(0.0, 0.0, layer_id as f32),
                        ..Default::default()
                    });
                }
            }
        }
    }
}
