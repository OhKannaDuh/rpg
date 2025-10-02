use crate::prelude::*;

mod systems;

#[add_plugin(to_group = CorePlugins)]
#[butler_plugin]
pub struct PlayerPlugin;
