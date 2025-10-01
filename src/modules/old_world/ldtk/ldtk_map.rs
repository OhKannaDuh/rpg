use crate::prelude::*;

#[derive(TypePath, Asset)]
pub struct LdtkMap {
    pub project: ldtk_rust::Project,
    pub tilesets: HashMap<i64, Handle<Image>>,
}
