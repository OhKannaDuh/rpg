use bevy::{prelude::*, window::WindowMode};
use bevy_asset_loader::prelude::*;
use rpg::data::state::rpg_state::RpgState;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("VS Alpha"),
                        present_mode: bevy::window::PresentMode::Immediate,
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Index(1)),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<RpgState>()
        .add_loading_state(
            LoadingState::new(RpgState::Loading).continue_to_state(RpgState::AssetsLoaded),
        )
        .add_plugins(rpg::Core)
        .run();
}
