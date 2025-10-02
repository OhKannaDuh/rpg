use super::*;

#[derive(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct RpgWorld {
    pub entity_definitions: HashMap<String, EntityDef>,
    pub layer_definitions: HashMap<String, LayerDef>,
    pub tilesets: HashMap<i64, Handle<Image>>,
    pub levels: HashMap<String, Level>,
    pub grid_size: i64,
    pub active_level_key: Option<String>,
    pub level_request: Option<String>,
}

impl RpgWorld {
    pub fn from_asset(asset: &LdtkMap) -> Self {
        let project = &asset.project;

        info!("Loading entity definitions");
        let mut entity_definitions = HashMap::new();
        for entity_def in &project.defs.entities {
            let def = EntityDef::from_instance(entity_def);
            info!(
                " - Loaded entity definition: {}. Had {} fields.",
                def.identifier,
                def.fields.len()
            );

            entity_definitions.insert(entity_def.identifier.clone(), def);
        }

        info!("Loading layer definitions");
        let mut layer_definitions = HashMap::new();
        for layer_def in &project.defs.layers {
            let Some(def) = LayerDef::from_instance(layer_def) else {
                warn!(
                    " - Could not parse layer definition: {}",
                    layer_def.identifier
                );
                continue;
            };

            info!(
                " - Loaded layer definition: {} of type {:?}",
                def.identifier, def.layer_type
            );

            layer_definitions.insert(layer_def.identifier.clone(), def);
        }

        info!("Loading levels");
        let mut levels = HashMap::new();
        for level in &project.levels {
            let level =
                Level::from_instance(level, project, &layer_definitions, &entity_definitions);
            info!(" - Loaded level: {} ({}).", level.identifier, level.iid);

            levels.insert(level.iid.clone(), level);
        }

        Self {
            entity_definitions,
            layer_definitions,
            tilesets: asset.tilesets.clone(),
            levels,
            grid_size: project.default_grid_size,
            active_level_key: None,
            level_request: None,
        }
    }

    pub fn get_active_level(&self) -> Option<&Level> {
        if let Some(active_key) = &self.active_level_key {
            return self.levels.get(active_key);
        }

        None
    }

    pub fn get_entity_def(&self, identifier: &str) -> Option<&EntityDef> {
        self.entity_definitions.get(identifier)
    }

    pub fn get_layer_def(&self, identifier: &str) -> Option<&LayerDef> {
        self.layer_definitions.get(identifier)
    }

    pub fn get_tile_layers(&self) -> HashMap<String, &TileLayer> {
        if let Some(active_key) = &self.active_level_key
            && let Some(level) = self.levels.get(active_key)
        {
            return level
                .tile_layers
                .iter()
                .map(|(k, v)| (k.clone(), v))
                .collect();
        }

        HashMap::new()
    }

    pub fn request_level_change(&mut self, iid: &str) {
        if iid == self.level_request.as_deref().unwrap_or("") {
            return;
        }

        if iid == self.active_level_key.as_deref().unwrap_or("") {
            warn!(
                "Requested level change to the currently active level: {}.",
                iid
            );
            return;
        }

        self.level_request = Some(iid.to_string());
    }
}
