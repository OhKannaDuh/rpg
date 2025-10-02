use crate::prelude::*;

mod systems;

#[derive(Resource, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[insert_resource(plugin = DevPlugin)]
pub struct DebugState {
    pub show_collision: bool,
}

#[add_plugin(to_group = DevPlugins)]
#[butler_plugin]
pub struct DevPlugin;
