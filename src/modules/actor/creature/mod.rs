prelude!();

pub struct CreaturePlugin;
game_module_build!(CreaturePlugin);

impl GameModule for CreaturePlugin {}

pub enum AspectType {
    Fire,
    Earth,
    Lightning,
    Water,
    Air,
    Void,
    Aether,
    Chaos,
}

pub struct Aspect {
    pub name: String,
    pub description: String,
    pub aspect_type: AspectType,
    pub strong_against: Option<AspectType>,
    pub weak_against: Option<AspectType>,
}

pub enum TraitType {
    Biological,
    Behavioural,
    Environmental,
    Metaphysical,
    Role,
}

pub struct Trait {
    pub name: String,
    pub description: String,
    pub trait_type: TraitType,
}
