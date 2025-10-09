prelude!();
module!(assets, resources, data);

pub fn populate_tileset_defs(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut tileset_defs: ResMut<TilesetDefs>,
) {
    info!("Populating tileset definitions...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for tileset_def in &map.project.defs.tilesets {
        info!(
            "Loading tileset def {} -> {}",
            tileset_def.uid,
            tileset_def.identifier.clone()
        );

        tileset_defs
            .0
            .insert(tileset_def.uid, TilesetDef::from_instance(tileset_def));
    }
}

pub fn populate_entity_defs(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut entity_defs: ResMut<EntityDefs>,
) {
    info!("Populating entity definitions...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for entity_def in &map.project.defs.entities {
        info!(
            "Loading entity def {} -> {}",
            entity_def.uid,
            entity_def.identifier.clone()
        );

        entity_defs.insert(entity_def.uid, EntityDef::from_instance(entity_def));
    }
}

pub fn populate_layer_defs(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut layer_defs: ResMut<LayerDefs>,
) {
    info!("Populating layer definitions...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for layer_def in &map.project.defs.layers {
        info!(
            "Loading layer def {} -> {}",
            layer_def.uid,
            layer_def.identifier.clone()
        );

        layer_defs
            .0
            .insert(layer_def.uid, LayerDef::from_instance(layer_def));
    }
}

pub fn populate_tilesets(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut tilesets: ResMut<Tilesets>,
) {
    info!("Populating tilesets...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    tilesets.0 = map.tilesets.clone();
}

pub fn populate_world_identity_map(
    world: Res<WorldAssets>,
    maps: Res<Assets<WorldMap>>,
    mut world_identity_map: ResMut<WorldIdentityMap>,
) {
    info!("Populating world identity map...");
    let Some(map) = maps.get(&world.handle) else {
        error!("World map handle present but asset not loaded yet");
        return;
    };

    for world in &map.project.worlds {
        world_identity_map.insert(WorldId(world.iid.clone()), world.identifier.clone());
    }
}
