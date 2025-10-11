prelude!();
public!(assets, data, resources, components, events);
private!(systems);

use bevy::prelude::IntoScheduleConfigs;

pub struct MapPlugin;
game_module_build!(MapPlugin);

impl GameModule for MapPlugin {
    fn assets(&self, app: &mut App) {
        app.init_asset::<WorldMap>()
            .register_asset_loader(WorldMapLoader);
    }

    fn configure_loading_state(&self, app: &mut App) {
        app.load_asset_collection::<WorldAssets>();
    }

    fn resources(&self, app: &mut App) {
        app.init_resource::<TilesetDefs>()
            .init_resource::<EntityDefs>()
            .init_resource::<LayerDefs>()
            .init_resource::<Tilesets>()
            .init_resource::<GlobalMapEntities>()
            .init_resource::<ChunkDataCollection>()
            .init_resource::<ChunkPositionMap>()
            .init_resource::<ChunkManager>()
            .init_resource::<WorldIdentityMap>()
            .init_resource::<WorldBounds>()
            .init_resource::<GlobalWaterAnimation>()
            .init_resource::<LdtkComponentRegistry>();
    }

    fn types(&self, app: &mut App) {
        app.register_type::<WaterTile>();
    }

    fn messages(&self, app: &mut App) {
        app.add_message::<ChunkLoaded>();
    }

    fn plugins(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin);
    }

    fn systems(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppLoadingState::PopulateDefinitions),
            (
                populate_tileset_defs,
                populate_entity_defs,
                populate_layer_defs,
                populate_tilesets,
                populate_world_identity_map,
                populate_ldtk_component_registry,
                spawn_world_root,
            )
                .in_set(AppLoadingSystems::PopulateDefinitions),
        );

        app.add_systems(
            OnEnter(AppLoadingState::PopulateChunkData),
            (
                populate_chunk_position_map,
                populate_chunk_data_collection,
                populate_global_map_entities,
            )
                .in_set(AppLoadingSystems::PopulateChunkData),
        );

        app.on_playing_game_update((
            manage_chunks_around_player,
            process_chunk_queues,
            update_world_bounds_on_player_world_id_change,
            animate_water_tiles,
        ));
    }
}
