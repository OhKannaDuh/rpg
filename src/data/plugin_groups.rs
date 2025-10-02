use crate::{Core, prelude::*};

#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct CorePlugins;

#[cfg(debug_assertions)]
#[butler_plugin_group]
#[add_plugin(to_plugin = Core)]
pub(crate) struct DevPlugins;
