use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct TileFlags: i32 {
        const SOLID         = 1 << 0;
        const SHALLOW_WATER = 1 << 1;
        const DEEP_WATER    = 1 << 2;
        const COLLIDE_NORTH = 1 << 3;
    }
}

impl TileFlags {
    pub fn from_ldtk_key(key: &str) -> Option<Self> {
        match key {
            "SOLID" => Some(Self::SOLID),
            "SHALLOW_WATER" => Some(Self::SHALLOW_WATER),
            "DEEP_WATER" => Some(Self::DEEP_WATER),
            "COLLIDE_NORTH" => Some(Self::COLLIDE_NORTH),
            _ => None,
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.contains(Self::SOLID) || self.contains(Self::DEEP_WATER)
    }
}
