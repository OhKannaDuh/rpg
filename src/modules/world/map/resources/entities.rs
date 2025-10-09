prelude!();
module!(data);

#[derive(Resource, Clone, Default, Debug)]
pub struct EntityDefs {
    uid_to_def: HashMap<i64, EntityDef>,
    identifier_to_uid: HashMap<String, i64>,
}

impl EntityDefs {
    pub fn insert(&mut self, uid: i64, def: EntityDef) {
        let identifier = def.identifier.clone();

        self.uid_to_def.insert(uid, def);
        self.identifier_to_uid.insert(identifier, uid);
    }

    pub fn by_uid(&self, uid: i64) -> Option<&EntityDef> {
        self.uid_to_def.get(&uid)
    }

    pub fn by_identifier(&self, identifier: String) -> Option<&EntityDef> {
        if let Some(&uid) = self.identifier_to_uid.get(&identifier) {
            self.uid_to_def.get(&uid)
        } else {
            None
        }
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct GlobalMapEntities {
    entities: Vec<MapEntity>,
}

impl GlobalMapEntities {
    pub fn add(&mut self, entity: MapEntity) {
        self.entities.push(entity);
    }

    pub fn by_def_identifier(&self, identifier: String) -> Vec<&MapEntity> {
        self.entities
            .iter()
            .filter(|e| e.def.identifier == identifier)
            .collect()
    }

    pub fn by_def_identifier_in_world(
        &self,
        identifier: String,
        world_id: WorldId,
    ) -> Vec<&MapEntity> {
        self.entities
            .iter()
            .filter(|e| e.def.identifier == identifier && e.world_id == world_id)
            .collect()
    }

    pub fn by_def_uid(&self, uid: i64) -> Option<&MapEntity> {
        self.entities.iter().find(|e| e.def.uid == uid)
    }

    pub fn by_def_uid_in_world(&self, uid: i64, world_id: WorldId) -> Option<&MapEntity> {
        self.entities
            .iter()
            .find(|e| e.def.uid == uid && e.world_id == world_id)
    }

    pub fn in_world(&self, world_id: WorldId) -> Vec<&MapEntity> {
        self.entities
            .iter()
            .filter(|e| e.world_id == world_id)
            .collect()
    }
}
