pub mod prelude;
use crate::prelude::*;

#[macro_use]
mod macros;

pub mod data;

mod components;
mod entities;
mod modules;

pub struct Core;

#[butler_plugin]
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        info!("Loading default plugins");
    }
}
