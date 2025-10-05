use super::*;

#[derive(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct RpgWorld {
    pub entity_definitions: HashMap<String, EntityDef>,
    pub layer_definitions: HashMap<String, LayerDef>,
    pub tilesets: HashMap<i64, Handle<Image>>,
    pub chunks: HashMap<ChunkId, Chunk>,
    pub chunk_positions: HashMap<ChunkPosition, ChunkId>,
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

        info!("Loading chunks");
        let mut chunks = HashMap::new();
        let mut chunk_positions = HashMap::new();

        for level in &project.levels {
            let chunk =
                Chunk::from_instance(level, project, &layer_definitions, &entity_definitions);
            info!(" - Loaded chunk: {} ({}).", level.identifier, level.iid);

            let position = chunk.position.clone();
            chunks.insert(chunk.id.clone(), chunk);

            info!("   - Position: ({}, {})", position.x, position.y);

            chunk_positions.insert(position, ChunkId::from_level(level));
        }

        Self {
            entity_definitions,
            layer_definitions,
            tilesets: asset.tilesets.clone(),
            chunks,
            chunk_positions,
            grid_size: project.default_grid_size,
        }
    }

    pub fn get_chunk_from_position(&self, position: &ChunkPosition) -> Option<&Chunk> {
        let id = self.chunk_positions.get(position)?;
        self.chunks.get(id)
    }
}
