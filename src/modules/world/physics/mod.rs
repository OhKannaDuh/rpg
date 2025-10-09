prelude!();

use bevy_rapier2d::rapier::prelude::IntegrationParameters;

use crate::modules::world::map::*;

pub struct PhysicsPlugin;
game_module_build!(PhysicsPlugin);

impl GameModule for PhysicsPlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins(
            RapierPhysicsPlugin::<NoUserData>::default().with_custom_initialization(
                RapierContextInitialization::InitializeDefaultRapierContext {
                    integration_parameters: IntegrationParameters {
                        length_unit: PIXELS_PER_METER,
                        ..default()
                    },
                    rapier_configuration: RapierConfiguration {
                        gravity: Vect::ZERO,
                        physics_pipeline_active: true,
                        scaled_shape_subdivision: 10,
                        force_update_from_transform_changes: true,
                    },
                },
            ),
        )
        .add_plugins(RapierDebugRenderPlugin::default());
    }

    fn systems(&self, app: &mut App) {
        app.on_playing_game_update(on_chunk_loaded);
    }
}

fn on_chunk_loaded(
    mut commands: Commands,
    chunk_data: Res<ChunkDataCollection>,
    mut chunk_loaded: MessageReader<ChunkLoaded>,
) {
    for event in chunk_loaded.read() {
        let Some(data) = chunk_data.0.get(&event.chunk_id) else {
            continue;
        };

        if let Some(collider) = data.get_collider() {
            info!("Spawning collider for chunk {:?}", data.id);
            commands.spawn((ChildOf(event.entity), collider));
        }
    }
}
