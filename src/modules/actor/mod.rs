prelude!();
public!(components, assets);
plugins!(
    (npc_ai, NpcAiPlugin),
    (player, PlayerPlugin),
    (actor_registry, ActorRegistryPlugin),
    (animation, AnimationPlugin),
    (creature, CreaturePlugin)
);

use std::fmt::Write;

use crate::modules::{actor::player::Player, world::map::*};
use bevy::sprite::Anchor;

pub struct ActorPlugin;
game_module_build!(ActorPlugin);

impl GameModule for ActorPlugin {
    fn configure_loading_state(&self, app: &mut App) {
        app.load_asset_collection::<ActorAssets>();
    }

    fn systems(&self, app: &mut App) {
        app.add_plugins((
            NpcAiPlugin,
            PlayerPlugin,
            ActorRegistryPlugin,
            AnimationPlugin,
            CreaturePlugin,
        ));

        app.on_playing_game_update((
            add_actor_debug_root,
            update_actor_debug_text,
            update_actor_debug_resolver,
            z_sort_actors,
        ));
    }
}

fn add_actor_debug_root(
    mut commands: Commands,
    query: Query<Entity, Added<ActorDebug>>,
    assets: Res<AssetServer>,
) {
    for entity in &query {
        commands.spawn((
            ActorDebugRoot::default(),
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

fn update_actor_debug_resolver(
    mut child_query: Query<(&ChildOf, &mut ActorDebugRoot)>,
    parent_query: Query<(&Transform, Option<&GridPosition>), With<ActorDebugResolver>>,
    player_query: Single<&Transform, With<Player>>,
) {
    let player_position = player_query.into_inner().translation.truncate();

    for (child, mut debug) in child_query.iter_mut() {
        let Ok((transform, grid_position_opt)) = parent_query.get(child.parent()) else {
            continue;
        };

        debug.0.clear();

        let position = transform.translation.truncate();
        let distance = player_position.distance(transform.translation.truncate());

        debug
            .0
            .write_fmt(format_args!("Distance: {:.2}\n", distance))
            .ok();

        debug
            .0
            .write_fmt(format_args!(
                "Position: {:.2}, {:.2}\n",
                position.x, position.y
            ))
            .ok();

        if let Some(grid_position) = grid_position_opt {
            let chunk_pos = grid_position.get_chunk(transform);
            let tile_pos = grid_position.get_position_in_chunk(transform);

            debug
                .0
                .write_fmt(format_args!(
                    "Chunk: {}, {}\n",
                    chunk_pos.chunk_x, chunk_pos.chunk_y
                ))
                .ok();

            debug
                .0
                .write_fmt(format_args!(
                    "Local Tile: {}, {}\n",
                    tile_pos.local_x, tile_pos.local_y
                ))
                .ok();

            debug
                .0
                .write_fmt(format_args!(
                    "World Tile: {}, {}\n",
                    tile_pos.world_x, tile_pos.world_y
                ))
                .ok();
        }
    }
}

fn update_actor_debug_text(mut query: Query<(&ActorDebugRoot, &mut Text2d)>) {
    for (root, mut text) in query.iter_mut() {
        text.0 = root.0.clone();
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
