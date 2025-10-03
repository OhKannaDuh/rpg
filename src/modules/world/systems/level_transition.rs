// use ldtk_rust::Flag;

// use crate::{components::*, modules::world::*};

// #[add_system(schedule = OnEnter(LevelTransitionState::FadingOut), plugin = WorldPlugin)]
// fn start_fade_out(
//     world_query: Query<&RpgWorld>,
//     fader_query: Query<Entity, With<Fader>>,
//     mut transition_state: ResMut<NextState<LevelTransitionState>>,
//     mut commands: Commands,
// ) {
//     let Ok(world) = world_query.single() else {
//         error!("Expected exactly one RpgWorld entity");
//         return;
//     };

//     info!("Fading out for level transition...");
//     if world.active_level_key.is_none() {
//         info!("No active level, skipping fade out...");
//         transition_state.set(LevelTransitionState::Loading);
//         return;
//     }

//     for entity in fader_query.iter() {
//         commands.entity(entity).remove::<FadeIn>();
//         commands.entity(entity).insert(FadeOut);
//     }
// }

// #[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(LevelTransitionState::FadingOut))]
// fn update_fade_out(
//     mut fader_query: Query<(Entity, &mut Sprite, &Fader), With<FadeOut>>,
//     mut transition_state: ResMut<NextState<LevelTransitionState>>,
//     mut commands: Commands,
//     time: Res<Time>,
// ) {
//     if fader_query.is_empty() {
//         transition_state.set(LevelTransitionState::Loading);
//         return;
//     }

//     let delta = time.delta_secs();

//     for (entity, mut sprite, fader) in fader_query.iter_mut() {
//         let a = sprite.color.alpha();
//         if a >= 1.0 {
//             commands.entity(entity).remove::<FadeOut>();
//             continue;
//         }

//         sprite.color.set_alpha(a + (fader.rate * delta).min(1.0));
//     }
// }

// #[add_system(schedule = OnEnter(LevelTransitionState::Loading), plugin = WorldPlugin)]
// fn transition(
//     mut commands: Commands,
//     mut query: Query<(Entity, &mut RpgWorld)>,
//     mut transition_state: ResMut<NextState<LevelTransitionState>>,
// ) {
//     let Ok((entity, mut world)) = query.single_mut() else {
//         error!("Expected exactly one RpgWorld entity");
//         return;
//     };

//     info!("Transitioning levels...");

//     if world.active_level_key.is_some() {
//         info!("Despawning current level...");
//         commands.entity(entity).despawn_related::<Children>();
//     }

//     let Some(requested) = &world.level_request else {
//         error!("Level transition state entered but no level request present");
//         return;
//     };

//     world.active_level_key = Some(requested.clone());
//     world.level_request = None;

//     let Some(level) = world.get_active_level() else {
//         panic!(
//             "Level with IID {} not found in world",
//             world
//                 .active_level_key
//                 .as_ref()
//                 .unwrap_or(&String::from("<unknown>"))
//         );
//     };

//     let world_pos = level.transform.bottom_left_bevy().as_vec2();

//     let mut nav: Nav = Nav::new(level);

//     // Load tile layers
//     for layer in level.tile_layers.values() {
//         let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

//         info!("Spawning tile layer: {}", layer.def.identifier);
//         let texture = world.tilesets[&layer.tileset_uid].clone();
//         let tile_size = TilemapTileSize {
//             x: layer.def.grid_size as f32,
//             y: layer.def.grid_size as f32,
//         };

//         let size = level.size.get_tilemap_size();

//         let mut storage = TileStorage::empty(size);

//         layer.instance.grid_tiles.iter().for_each(|tile| {
//             let pos = TilePos {
//                 x: (tile.px[0] / layer.def.grid_size) as u32,
//                 y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
//             };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: pos,
//                     tilemap_id: TilemapId(map_entity),
//                     texture_index: TileTextureIndex(tile.t as u32),
//                     ..Default::default()
//                 })
//                 .id();

//             storage.set(&pos, tile_entity);
//         });

//         commands.entity(map_entity).insert(TilemapBundle {
//             grid_size: tile_size.into(),
//             tile_size,
//             texture: TilemapTexture::Single(texture),
//             size,
//             storage,
//             map_type: TilemapType::default(),
//             anchor: TilemapAnchor::BottomLeft,
//             transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
//             ..Default::default()
//         });
//     }

//     // Load auto layers
//     for layer in level.auto_layers.values() {
//         let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

//         info!("Spawning auto layer: {}", layer.def.identifier);
//         let texture = world.tilesets[&layer.tileset_uid].clone();
//         let tile_size = TilemapTileSize {
//             x: layer.def.grid_size as f32,
//             y: layer.def.grid_size as f32,
//         };

//         let size = level.size.get_tilemap_size();

//         let mut storage = TileStorage::empty(size);

//         layer.instance.auto_layer_tiles.iter().for_each(|tile| {
//             let pos = TilePos {
//                 x: (tile.px[0] / layer.def.grid_size) as u32,
//                 y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
//             };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: pos,
//                     tilemap_id: TilemapId(map_entity),
//                     texture_index: TileTextureIndex(tile.t as u32),
//                     ..Default::default()
//                 })
//                 .id();

//             storage.set(&pos, tile_entity);
//         });

//         commands.entity(map_entity).insert(TilemapBundle {
//             grid_size: tile_size.into(),
//             tile_size,
//             texture: TilemapTexture::Single(texture),
//             size,
//             storage,
//             map_type: TilemapType::default(),
//             anchor: TilemapAnchor::BottomLeft,
//             transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
//             ..Default::default()
//         });
//     }

//     // Load auto layers
//     for layer in level.int_grid_layers.values() {
//         if layer.instance.auto_layer_tiles.is_empty() {
//             continue;
//         }

//         let map_entity: Entity = commands.spawn(ChildOf(entity)).id();

//         info!("Spawning (auto) int grid layer: {}", layer.def.identifier);
//         let texture = world.tilesets[&layer.tileset_uid].clone();
//         let tile_size = TilemapTileSize {
//             x: layer.def.grid_size as f32,
//             y: layer.def.grid_size as f32,
//         };

//         let size = level.size.get_tilemap_size();

//         let mut storage = TileStorage::empty(size);

//         layer.instance.auto_layer_tiles.iter().for_each(|tile| {
//             let pos = TilePos {
//                 x: (tile.px[0] / layer.def.grid_size) as u32,
//                 y: size.y - 1 - (tile.px[1] / layer.def.grid_size) as u32,
//             };

//             let tile_entity = commands
//                 .spawn(TileBundle {
//                     position: pos,
//                     tilemap_id: TilemapId(map_entity),
//                     texture_index: TileTextureIndex(tile.t as u32),
//                     ..Default::default()
//                 })
//                 .id();

//             storage.set(&pos, tile_entity);
//         });

//         commands.entity(map_entity).insert(TilemapBundle {
//             grid_size: tile_size.into(),
//             tile_size,
//             texture: TilemapTexture::Single(texture),
//             size,
//             storage,
//             map_type: TilemapType::default(),
//             anchor: TilemapAnchor::BottomLeft,
//             transform: Transform::from_xyz(world_pos.x, world_pos.y, layer.def.z as f32),
//             ..Default::default()
//         });
//     }

//     for layer in level
//         .int_grid_layers
//         .values()
//         .filter(|l| l.def.identifier != "TILE_FLAGS")
//     {
//         layer
//             .instance
//             .int_grid_csv
//             .iter()
//             .enumerate()
//             .for_each(|(i, v)| {
//                 let Some(flag) = NavFlag::from_id(*v) else {
//                     return;
//                 };

//                 let local = level.grid.get_bevy_position(i as i64);
//                 nav.set_flag(local.x, local.y, flag);
//             });
//     }

//     for layer in level
//         .int_grid_layers
//         .values()
//         .filter(|l| l.def.identifier == "TILE_FLAGS")
//     {
//         layer
//             .instance
//             .int_grid_csv
//             .iter()
//             .enumerate()
//             .for_each(|(i, v)| {
//                 if *v != 1 {
//                     return;
//                 }

//                 let local = level.grid.get_bevy_position(i as i64);
//                 nav.set_flag(local.x, local.y, NavFlag::Blocked);
//             });
//     }

//     nav.bake_collision();
//     commands.spawn((nav, ChildOf(entity)));

//     // Load entities

//     transition_state.set(LevelTransitionState::FadingIn);
// }

// #[add_system(schedule = OnEnter(LevelTransitionState::FadingIn), plugin = WorldPlugin)]
// fn start_fade_in(fader_query: Query<Entity, With<Fader>>, mut commands: Commands) {
//     info!("Fading in for level transition...");

//     for entity in fader_query.iter() {
//         commands.entity(entity).remove::<FadeOut>();
//         commands.entity(entity).insert(FadeIn);
//     }
// }

// #[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(LevelTransitionState::FadingIn))]
// fn update_fade_in(
//     mut fader_query: Query<(Entity, &mut Sprite, &Fader), With<FadeIn>>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut commands: Commands,
//     time: Res<Time>,
// ) {
//     if fader_query.is_empty() {
//         game_state.set(GameState::Playing);
//         return;
//     }

//     let delta = time.delta_secs();

//     for (entity, mut sprite, fader) in fader_query.iter_mut() {
//         let a = sprite.color.alpha();
//         if a <= 0.0 {
//             commands.entity(entity).remove::<FadeIn>();
//             continue;
//         }

//         sprite.color.set_alpha(a - (fader.rate * delta).max(0.0));
//     }
// }
