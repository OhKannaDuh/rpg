pub mod prelude;
use crate::prelude::*;

pub mod data;

mod components;
mod entities;
mod modules;

pub struct Core;

#[butler_plugin]
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        info!("Loading default plugins");

        // .init_state::<JrpgState>()
        // .add_loading_state(
        //     LoadingState::new(JrpgState::LoadingAssets).continue_to_state(JrpgState::InGame),
        // );

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((
        //         bevy::remote::RemotePlugin::default(),
        //         bevy::remote::http::RemoteHttpPlugin::default(),
        //         bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        //         // bevy::diagnostic::EntityCountDiagnosticsPlugin,
        //         // bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        //         // bevy::render::diagnostic::RenderDiagnosticsPlugin,
        //         iyes_perf_ui::PerfUiPlugin,
        //     ));

        //     fn add_performance_ui(mut commands: Commands) {
        //         commands.spawn((
        //             iyes_perf_ui::prelude::PerfUiEntryFPS::default(),
        //             iyes_perf_ui::prelude::PerfUiEntryFPSAverage::default(),
        //             iyes_perf_ui::prelude::PerfUiEntryFPSWorst::default(),
        //         ));
        //     }

        //     app.add_systems(Startup, add_performance_ui);
        // }

        // app.add_plugins(LdtkPlugin);
    }
}
