prelude!();

#[derive(Reflect, Debug)]
pub struct CreatureStat {
    pub max: f32,
    pub current: f32,
}

impl CreatureStat {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn increase(&mut self, amount: f32) {
        self.current = (self.current + amount).clamp(0.0, self.max);
    }

    pub fn decrease(&mut self, amount: f32) {
        self.current = (self.current - amount).clamp(0.0, self.max);
    }
}

#[derive(Component, Reflect, Debug)]
pub struct CreatureStats {
    pub health: CreatureStat,
    pub energy: CreatureStat,
}

impl Default for CreatureStats {
    fn default() -> Self {
        Self {
            health: CreatureStat::new(100.0),
            energy: CreatureStat::new(100.0),
        }
    }
}
