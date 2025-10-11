prelude!();

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct CreatureBehaviour {
    pub roaming: RoamingBehaviour,
}

#[derive(Clone, Copy, Debug, Default)]
pub enum CreatureBehaviourPresetType {
    Clingy,
    #[default]
    Independent,
    Curious,
    Lazy,
    Hyperactive,
}

impl CreatureBehaviourPresetType {
    pub fn get_behaviour(&self) -> CreatureBehaviour {
        match self {
            Self::Clingy => CreatureBehaviour {
                roaming: RoamingBehaviour::clingy(),
            },
            Self::Independent => CreatureBehaviour {
                roaming: RoamingBehaviour::independent(),
            },
            Self::Curious => CreatureBehaviour {
                roaming: RoamingBehaviour::curious(),
            },
            Self::Lazy => CreatureBehaviour {
                roaming: RoamingBehaviour::lazy(),
            },
            Self::Hyperactive => CreatureBehaviour {
                roaming: RoamingBehaviour::hyperactive(),
            },
        }
    }
}

impl From<CreatureBehaviourPresetType> for CreatureBehaviour {
    fn from(t: CreatureBehaviourPresetType) -> Self {
        t.get_behaviour()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RoamingBehaviour {
    pub min_idle_time: f32,
    pub max_idle_time: f32,

    pub min_wander_distance_squared: f32,
    pub max_wander_distance_squared: f32,
}

impl RoamingBehaviour {
    pub fn clingy() -> Self {
        Self {
            min_idle_time: 0.5,
            max_idle_time: 2.0,
            min_wander_distance_squared: tile_size_squared(1.0),
            max_wander_distance_squared: tile_size_squared(3.0),
        }
    }

    pub fn independent() -> Self {
        Self {
            min_idle_time: 3.0,
            max_idle_time: 6.0,
            max_wander_distance_squared: tile_size_squared(12.0),
            min_wander_distance_squared: tile_size_squared(6.0),
        }
    }

    pub fn curious() -> Self {
        Self {
            min_idle_time: 1.0,
            max_idle_time: 3.0,
            max_wander_distance_squared: tile_size_squared(8.0),
            min_wander_distance_squared: tile_size_squared(4.0),
        }
    }

    pub fn lazy() -> Self {
        Self {
            min_idle_time: 5.0,
            max_idle_time: 10.0,
            max_wander_distance_squared: tile_size_squared(2.0),
            min_wander_distance_squared: tile_size_squared(1.0),
        }
    }

    pub fn hyperactive() -> Self {
        Self {
            min_idle_time: 0.1,
            max_idle_time: 0.5,
            max_wander_distance_squared: tile_size_squared(10.0),
            min_wander_distance_squared: tile_size_squared(5.0),
        }
    }
}

impl Default for RoamingBehaviour {
    fn default() -> Self {
        Self::independent()
    }
}
