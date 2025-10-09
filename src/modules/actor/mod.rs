prelude!();
public!(components, assets);
plugins!(
    (npc_ai, NpcAiPlugin),
    (player, PlayerPlugin),
    (actor_registry, ActorRegistryPlugin)
);

use crate::modules::world::map::*;
use bevy::sprite::Anchor;

pub struct ActorPlugin;
game_module_build!(ActorPlugin);

impl GameModule for ActorPlugin {
    fn configure_loading_state(&self, app: &mut App) {
        app.load_asset_collection::<ActorAssets>();
    }

    fn systems(&self, app: &mut App) {
        app.add_plugins((NpcAiPlugin, PlayerPlugin, ActorRegistryPlugin));

        app.on_playing_game_update((add_actor_debug_root, update_actor_debug_text, z_sort_actors));
    }
}

fn add_actor_debug_root(
    mut commands: Commands,
    query: Query<Entity, Added<ActorDebug>>,
    assets: Res<AssetServer>,
) {
    for entity in &query {
        commands.spawn((
            ActorDebugRoot,
            Text2d("Debug Info".into()),
            Transform::from_xyz(12.0, 8.0, 1.0).with_scale(Vec3::splat(0.3)),
            Anchor::TOP_LEFT,
            TextFont {
                font: assets.load("fonts/FiraSans-Bold.ttf"),
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            ChildOf(entity),
        ));
    }
}

fn update_actor_debug_text(
    mut child_query: Query<(&ChildOf, &mut Text2d), With<ActorDebugRoot>>,
    parent_query: Query<(&GridPosition, &Transform, &CurrentWorldId), With<ActorDebug>>,
    chunk_positions: Res<ChunkPositionMap>,
) {
    for (child, mut text) in child_query.iter_mut() {
        let Ok((grid_position, transform, current_world_id)) = parent_query.get(child.parent())
        else {
            continue;
        };

        let Some(world_id) = &current_world_id.0 else {
            continue;
        };

        let chunk_position = grid_position.get_chunk(transform);
        let tile_position = grid_position.get_position_in_chunk(transform);

        let mut level_id = "None";
        if let Some(chunk_id) = chunk_positions.get(world_id, &chunk_position.get_chunk_coords()) {
            level_id = chunk_id.0.as_str();
        }

        text.0 = format!(
            "Chunk: {}, {}\nLocal Tile: {}, {}\nWorld Tile: {}, {}\nLevel: {}",
            chunk_position.chunk_x,
            chunk_position.chunk_y,
            tile_position.local_x,
            tile_position.local_y,
            tile_position.world_x,
            tile_position.world_y,
            level_id
        );
    }
}

fn z_sort_actors(
    mut query: Query<(&mut Transform, Option<&FootPosition>), With<Actor>>,
    world_bounds: Res<WorldBounds>,
) {
    const DOMAIN_MIN: f32 = 0.0;
    const DOMAIN_MAX: f32 = 1.0;

    let min_y = world_bounds.center.y - world_bounds.height * 0.5;
    let max_y = world_bounds.center.y + world_bounds.height * 0.5;

    let range_y = max_y - min_y;

    for (mut transform, opt_foot_position) in &mut query {
        let mut y = transform.translation.y;

        if let Some(foot_position) = opt_foot_position {
            y += foot_position.0;
        }

        let mut t = (y - min_y) / range_y;
        t = t.clamp(DOMAIN_MIN, DOMAIN_MAX);

        transform.translation.z = DOMAIN_MAX - t;
    }
}
