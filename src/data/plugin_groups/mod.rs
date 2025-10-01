use crate::prelude::*;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct CorePlugins;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct EntityPlugins;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct RenderingPlugins;

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct WorldPlugins;

#[cfg(debug_assertions)]
#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct DevPlugins;
