use bevy::{prelude::*, window::WindowMode};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use rpg::data::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
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
        .init_state::<AppState>()
        .add_sub_state::<AppLoadingState>()
        .add_sub_state::<GameState>()
        .add_loading_state(
            LoadingState::new(AppLoadingState::LoadingAssets)
                .continue_to_state(AppLoadingState::ProcessingWorldFile),
        )
        .add_plugins(rpg::Core)
        .run();
}
