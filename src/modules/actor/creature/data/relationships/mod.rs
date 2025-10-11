prelude!();

pub const MIN_FRIENDSHIP: f32 = -255.0;
pub const MAX_FRIENDSHIP: f32 = 255.0;

#[derive(Reflect, Debug)]
pub struct CreatureRelationship {
    pub other: Entity,
    pub friendship: f32,
}

impl CreatureRelationship {
    pub fn increase(&mut self, amount: f32) {
        self.friendship = (self.friendship + amount).clamp(MIN_FRIENDSHIP, MAX_FRIENDSHIP);
    }

    pub fn decrease(&mut self, amount: f32) {
        self.friendship = (self.friendship - amount).clamp(MIN_FRIENDSHIP, MAX_FRIENDSHIP);
    }
}

#[derive(Component, Reflect, Debug, Default)]
pub struct CreatureRelationships {
    pub relationships: HashMap<Entity, CreatureRelationship>,
}
