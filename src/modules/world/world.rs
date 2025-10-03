use super::*;

#[derive(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct RpgWorld {
    pub entity_definitions: HashMap<String, EntityDef>,
    pub layer_definitions: HashMap<String, LayerDef>,
    pub tilesets: HashMap<i64, Handle<Image>>,
    pub levels: HashMap<String, Level>,
    pub grid_size: i64,
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
                " - Loaded layer definition: {} of type {:?}, z: {}.",
                def.identifier, def.layer_type, def.z
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
        }
    }

    pub fn get_levels_from_ids(&self, ids: Vec<String>) -> Vec<&Level> {
        ids.iter().filter_map(|id| self.levels.get(id)).collect()
    }

    pub fn get_level_at_point(&self, point: Vec2) -> Option<&Level> {
        self.levels.values().find(|level| level.contains(point))
    }
}
