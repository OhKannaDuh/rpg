prelude!();

#[derive(Reflect, Debug)]
pub struct CreatureNeed {
    pub max: f32,
    pub current: f32,
}

impl CreatureNeed {
    pub fn new(max: f32) -> Self {
        Self { max, current: max }
    }

    pub fn increase(&mut self, amount: f32) {
        self.current = (self.current + amount).clamp(0.0, self.max);
    }

    pub fn decrease(&mut self, amount: f32) {
        self.current = (self.current - amount).clamp(0.0, self.max);
    }

    pub fn is_depleted(&self) -> bool {
        self.current <= 0.0
    }

    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
}

#[derive(Component, Reflect, Debug)]
pub struct CreatureNeeds {
    pub social: CreatureNeed,
    pub exploration: CreatureNeed,
}

impl Default for CreatureNeeds {
    fn default() -> Self {
        Self {
            exploration: CreatureNeed::new(100.0),
            social: CreatureNeed::new(100.0),
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct IsSocialising;

#[queryable]
pub trait SocialDrainer: Send + Sync + 'static {
    fn amount(&self) -> f32;
}

#[queryable]
pub trait SocialRestorer: Send + Sync + 'static {
    fn amount(&self) -> f32;
}

impl SocialRestorer for IsSocialising {
    fn amount(&self) -> f32 {
        0.1
    }
}

// Natural drain
impl SocialDrainer for CreatureNeeds {
    fn amount(&self) -> f32 {
        1.0
    }
}

#[derive(Component, Reflect, Debug)]
pub struct IsExploring;

#[queryable]
pub trait ExplorationDrainer: Send + Sync + 'static {
    fn amount(&self) -> f32;
}

#[queryable]
pub trait ExplorationRestorer: Send + Sync + 'static {
    fn amount(&self) -> f32;
}

impl ExplorationRestorer for IsExploring {
    fn amount(&self) -> f32 {
        0.1
    }
}

// Natural drain
impl ExplorationDrainer for CreatureNeeds {
    fn amount(&self) -> f32 {
        0.01
    }
}
