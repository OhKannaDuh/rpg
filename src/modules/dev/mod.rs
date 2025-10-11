prelude!();

use bevy::ecs::query;
use bevy::remote::RemotePlugin;
use bevy::remote::http::RemoteHttpPlugin;

use crate::modules::actor::player::*;
use crate::modules::actor::*;
use crate::modules::world::map::*;

const DEV_ENTITY_GIZMO_MAX_DISTANCE: f32 = 2_000.0;

pub struct DevPlugin;
game_module_build!(DevPlugin);

impl GameModule for DevPlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));
        // app.add_plugins(RapierDebugRenderPlugin::default());
    }

    fn systems(&self, app: &mut App) {
        app.on_playing_game_update((
            // render_global_entities_for_player,
            draw_agent_radius,
            draw_character_radius,
        ));
    }
}

fn render_global_entities_for_player(
    player_data: Single<(&Transform, &CurrentWorldId), With<Player>>,
    global_map_entities: Res<GlobalMapEntities>,
    mut gizmos: Gizmos,
) {
    let (transform, current_world_id) = player_data.into_inner();

    let Some(world_id) = &current_world_id.0 else {
        return;
    };

    let player_pos = transform.translation.truncate();

    for entity in global_map_entities.in_world(world_id.clone()) {
        let center = Vec2::new(entity.world_x as f32, entity.world_y as f32);
        let size = Vec2::new(entity.width_px as f32, entity.height_px as f32);

        if center.distance(player_pos) > DEV_ENTITY_GIZMO_MAX_DISTANCE {
            continue;
        }

        let color = Color::srgb(0.0, 0.7, 0.55);

        gizmos.rect_2d(Isometry2d::from_translation(center), size, color);
    }
}

pub trait StateDebug {
    fn debug(&self) -> String;
}

fn draw_agent_radius(query: Query<(&Transform, &AgentSettings)>, mut gizmos: Gizmos) {
    for (transform, settings) in query.iter() {
        let radius = settings.radius;
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            radius,
            Color::srgb(0.0, 0.7, 0.55),
        );
    }
}

fn draw_character_radius(query: Query<(&Transform, &CharacterSettings)>, mut gizmos: Gizmos) {
    for (transform, settings) in query.iter() {
        let radius = settings.radius;
        gizmos.circle_2d(
            Isometry2d::from_translation(transform.translation.truncate()),
            radius,
            Color::srgb(1.0, 0.5, 0.5),
        );
    }
}
