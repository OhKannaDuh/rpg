prelude!();

pub trait FromLdtkTile: Component + Sized + 'static {
    const NAME: &'static str;

    fn from_ldtk(tile: &TileInstance, data: &HashMap<String, String>) -> Option<Self>;
}

#[derive(Component, Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
#[reflect(Component)]
#[require(Transform, GlobalTransform, Visibility, InheritedVisibility)]
pub struct WaterTile {
    pub base_texture_index: i64,
    pub stride: i64,
}

impl FromLdtkTile for WaterTile {
    const NAME: &'static str = "WaterTile";

    fn from_ldtk(tile: &TileInstance, data: &HashMap<String, String>) -> Option<Self> {
        let stride = data
            .get("stride")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap();

        Some(WaterTile {
            base_texture_index: tile.t,
            stride,
        })
    }
}
