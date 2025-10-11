prelude!();
module!(data);

#[derive(Resource, Clone, Default, Debug)]
pub struct WorldIdentityMap {
    id_to_identifier: HashMap<WorldId, String>,
    identifier_to_id: HashMap<String, WorldId>,
}

impl WorldIdentityMap {
    pub fn insert(&mut self, world_id: WorldId, identifier: String) {
        self.id_to_identifier
            .insert(world_id.clone(), identifier.clone());
        self.identifier_to_id.insert(identifier, world_id);
    }

    pub fn get_id(&self, identifier: &str) -> Option<&WorldId> {
        self.identifier_to_id.get(identifier)
    }

    pub fn get_identifier(&self, world_id: &WorldId) -> Option<&String> {
        self.id_to_identifier.get(world_id)
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct WorldBounds {
    pub world_id: String,
    pub width: f32,
    pub height: f32,
    pub center: Vec2,
}

#[derive(Resource, Clone, Default, Debug)]
pub struct GlobalWaterAnimation {
    pub frame_time: f32,
    pub frame: usize,
    pub forward: bool,
}

impl GlobalWaterAnimation {
    const FRAME_TIME: f32 = 0.5;
    const FRAME_COUNT: usize = 3;

    pub fn update(&mut self, delta: f32) {
        self.frame_time += delta;
        if self.frame_time >= Self::FRAME_TIME {
            self.frame_time -= Self::FRAME_TIME;

            if self.forward {
                if self.frame + 1 >= Self::FRAME_COUNT {
                    self.forward = false;
                    self.frame = Self::FRAME_COUNT - 2;
                } else {
                    self.frame += 1;
                }
            } else if self.frame == 0 {
                self.forward = true;
                self.frame = 1;
            } else {
                self.frame -= 1;
            }
        }
    }
}
