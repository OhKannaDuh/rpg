use crate::{modules::old_world::plugin::WorldPlugin, prelude::*};

#[derive(Resource, Debug, Clone, Default)]
#[insert_resource(plugin = WorldPlugin)]
pub struct LdtkIidMap(pub HashMap<String, usize>);
