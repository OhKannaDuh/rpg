prelude!();

use crate::modules::actor::creature::SpawnCreatureMessage;
use crate::modules::actor::*;
use crate::modules::camera::*;
use bevy::math::I64Vec2;
use rand::rng;
use rand::seq::IndexedRandom;

pub struct PlayerPlugin;
game_module_build!(PlayerPlugin);

impl GameModule for PlayerPlugin {
    fn systems(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player);

        app.on_playing_game_update((handle_input, update_player_debug_resolver));
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    global_map_entities: Res<GlobalMapEntities>,
    world_identity_map: Res<WorldIdentityMap>,
    mut spawn_creature: MessageWriter<SpawnCreatureMessage>,
) {
    info!("Spawning player...");

    let mut start = I64Vec2::ZERO;

    let world_id = world_identity_map
        .get_id(DEFAULT_WORLD_IDENTIFIER)
        .cloned()
        .unwrap_or(WorldId("".into()));

    let spawn_points = global_map_entities
        .by_def_identifier_in_world("INITIAL_PLAYER_SPAWN".into(), world_id.clone());

    if !spawn_points.is_empty() {
        let mut rng = rng();
        let spawn = spawn_points.choose(&mut rng).unwrap();

        start.x = spawn.world_x;
        start.y = spawn.world_y;
    }

    let player = commands
        .spawn((
            Name::new("Player"),
            Player,
            Sprite {
                color: Color::srgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            FootPosition(-TILE_SIZE / 2.0),
            CurrentWorldId(Some(world_id)),
            Transform::from_xyz(start.x as f32, start.y as f32, 0.0),
            PlayerDebugResolver,
            CameraFocus,
            // Physics
            ENTITY_COLLISION,
            RigidBody::KinematicPositionBased,
            Collider::ball(DEFAULT_ACTOR_COLLIDER_RADIUS),
            Restitution::coefficient(DEFAULT_ACTOR_RESTITUTION),
            KinematicCharacterController {
                apply_impulse_to_dynamic_bodies: false,
                filter_groups: Some(ENTITY_COLLISION),
                ..Default::default()
            },
            Sleeping::disabled(),
        ))
        .id();

    let center = Vec2::new(start.x as f32, start.y as f32);
    let r = 64.0;
    let sqrt3_over_2 = 0.866_025_4;

    let p1 = center + Vec2::new(0.0, r);
    let p2 = center + Vec2::new(sqrt3_over_2 * r, -0.5 * r);
    let p3 = center + Vec2::new(-sqrt3_over_2 * r, -0.5 * r);

    spawn_creature.write(SpawnCreatureMessage {
        name: "Steve".into(),
        position: p1,
        owner: Some(player),
        spawn_radius: Some(128.0),
    });

    spawn_creature.write(SpawnCreatureMessage {
        name: "Anabelle".into(),
        position: p2,
        owner: Some(player),
        spawn_radius: Some(128.0),
    });

    spawn_creature.write(SpawnCreatureMessage {
        name: "Jorge".into(),
        position: p3,
        owner: Some(player),
        spawn_radius: Some(128.0),
    });
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Single<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
) {
    let mut movement = Vec2::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    } else if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    } else if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }

    if movement == Vec2::ZERO {
        return;
    }

    movement = movement.normalize();
    movement *= DEFAULT_ACTOR_SPEED * time.delta_secs();

    let mut controller = query.into_inner();
    controller.translation = Some(movement);
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(ActorDebug)]
pub struct PlayerDebugResolver;

fn update_player_debug_resolver(
    mut child_query: Query<(&ChildOf, &mut ActorDebugRoot)>,
    parent_query: Query<(&GridPosition, &Transform, &CurrentWorldId), With<PlayerDebugResolver>>,
    chunk_positions: Res<ChunkPositionMap>,
) {
    for (child, mut debug) in child_query.iter_mut() {
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

        debug.0 = format!(
            "Chunk: {}, {}\nLocal Tile: {}, {}\nWorld Tile: {}, {}\nPixel Position: {}, {}\nLevel: {}",
            chunk_position.chunk_x,
            chunk_position.chunk_y,
            tile_position.local_x,
            tile_position.local_y,
            tile_position.world_x,
            tile_position.world_y,
            transform.translation.x,
            transform.translation.y,
            level_id
        );
    }
}
