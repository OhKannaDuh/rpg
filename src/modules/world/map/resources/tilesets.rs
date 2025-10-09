prelude!();
module!(data);

#[derive(Resource, Clone, Default, Debug)]
pub struct TilesetDefs(pub HashMap<i64, TilesetDef>);

#[derive(Resource, Clone, Default, Debug)]
pub struct Tilesets(pub HashMap<i64, Handle<Image>>);
