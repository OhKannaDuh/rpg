use crate::modules::world::ldtk::LdtkMap;
use crate::prelude::*;

#[derive(Default, Component)]
pub struct LdtkMapHandle(pub Handle<LdtkMap>);
