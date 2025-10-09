pub mod prelude;
use crate::prelude::*;

#[macro_use]
mod macros;

mods!(states, system_sets, modules, config, extensions);

pub struct Core;

impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("VS Alpha"),
                        present_mode: bevy::window::PresentMode::Immediate,
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            MonitorSelection::Index(1),
                        ),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );

        app.add_plugins((StatesPlugin, SystemSetsPlugin, ModulePlugin));
    }
}
