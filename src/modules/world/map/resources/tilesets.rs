use crate::modules::world::map::FromLdtkTile;

prelude!();
module!(data);

#[derive(Resource, Clone, Default, Debug)]
pub struct TilesetDefs(pub HashMap<i64, TilesetDef>);

#[derive(Resource, Clone, Default, Debug)]
pub struct Tilesets(pub HashMap<i64, Handle<Image>>);

pub type ComponentFactory =
    fn(tile: &TileInstance, data: &HashMap<String, String>) -> Option<Box<dyn Reflect>>;

#[derive(Resource, Clone, Default, Debug)]
pub struct LdtkComponentRegistry {
    map: HashMap<String, ComponentFactory>,
}

impl LdtkComponentRegistry {
    pub fn register<T: FromLdtkTile + Reflect + Component>(&mut self) {
        self.map.insert(T::NAME.to_string(), |tile, data| {
            T::from_ldtk(tile, data).map(|c| Box::new(c) as Box<dyn Reflect>)
        });
    }

    pub fn create(
        &self,
        name: &str,
        tile: &TileInstance,
        data: &HashMap<String, String>,
    ) -> Option<Box<dyn Reflect>> {
        self.map.get(name).and_then(|f| f(tile, data))
    }
}
