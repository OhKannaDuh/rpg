prelude!();
private!(data);

use std::u32::MAX;

use bevy::state;

use crate::modules::{
    actor::{player::Player, *},
    dev::StateDebug,
};

pub struct CreaturePlugin;
game_module_build!(CreaturePlugin);

impl GameModule for CreaturePlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins(StateMachinePlugin::default());
    }

    fn messages(&self, app: &mut App) {
        app.add_message::<SpawnCreatureMessage>();
    }

    fn systems(&self, app: &mut App) {
        app.on_playing_game_update((
            CreatureStateRoamingAroundOwner::process,
            CreatureStateCatchingUpToOwner::process,
        ));

        app.on_playing_game_update((
            spawn_creature,
            update_creature_debug_resolver,
            CreatureStateRoamingAroundOwner::render_path_gizmos,
        ));
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(Actor)]
pub struct Creature;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Creature)]
pub struct OwnedBy(pub Entity);

#[derive(Message)]
pub struct SpawnCreatureMessage {
    pub name: String,
    pub position: Vec2,
    pub owner: Option<Entity>,
    pub spawn_radius: Option<f32>,
}

fn spawn_creature(mut commands: Commands, mut messages: MessageReader<SpawnCreatureMessage>) {
    const WANDER_DISTANCE_MAX: f32 = (TILE_SIZE * 8.0) * (TILE_SIZE * 8.0);
    const WANDER_DISTANCE_MIN: f32 = (TILE_SIZE * 3.0) * (TILE_SIZE * 3.0);
    const CATCHUP_START_DISTANCE: f32 =
        ((TILE_SIZE * 8.0) + TILE_SIZE) * ((TILE_SIZE * 8.0) + TILE_SIZE);

    const CATCHUP_STOP_DISTANCE: f32 = (TILE_SIZE * 2.0) * (TILE_SIZE * 2.0);

    for message in messages.read() {
        info!("Spawning creature {}", message.name);

        let mut position = message.position;

        if let Some(radius) = message.spawn_radius {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let offset = rand::random::<f32>() * radius;
            position += Vec2::new(angle.cos() * offset, angle.sin() * offset);
        }

        let creature = commands
            .spawn((
                Name::new(message.name.clone()),
                Creature,
                Transform::from_xyz(position.x, position.y, 0.0),
                Sprite {
                    color: Color::srgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..Default::default()
                },
                CreatureDebugResolver,
                // State
                CreatureStateIdle,
                StateMachine::default()
                    // If we are idle and within range of owner, start roaming around owner
                    .trans::<CreatureStateIdle, _>(
                        CreatureStateHelpers::within_distance_to_owner(WANDER_DISTANCE_MAX),
                        CreatureStateRoamingAroundOwner {
                            min_wander_distance: ops::sqrt(WANDER_DISTANCE_MIN),
                            max_wander_distance: ops::sqrt(WANDER_DISTANCE_MAX) - 1.0,
                            ..Default::default()
                        },
                    )
                    // If we every get too far from our owner, catch up to them
                    .trans::<NotState<CreatureStateCatchingUpToOwner>, _>(
                        CreatureStateHelpers::outside_distance_to_owner(CATCHUP_START_DISTANCE),
                        CreatureStateCatchingUpToOwner,
                    )
                    // If we have caught up to our owner, return to idle
                    .trans::<CreatureStateCatchingUpToOwner, _>(
                        CreatureStateHelpers::within_distance_to_owner(CATCHUP_STOP_DISTANCE),
                        CreatureStateIdle,
                    )
                    .set_trans_logging(true),
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
            ))
            .id();

        if let Some(owner) = message.owner {
            commands.entity(creature).insert(OwnedBy(owner));
        }
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(ActorDebug)]
pub struct CreatureDebugResolver;

fn update_creature_debug_resolver(
    mut child_query: Query<(&ChildOf, &mut ActorDebugRoot)>,
    parent_query: Query<
        (
            &Transform,
            &Name,
            Option<&OwnedBy>,
            Option<&CreatureStateIdle>,
            Option<&CreatureStateRoamingAroundOwner>,
            Option<&CreatureStateCatchingUpToOwner>,
        ),
        With<CreatureDebugResolver>,
    >,
    owner_query: Query<&Transform>,
) {
    for (child, mut debug) in child_query.iter_mut() {
        let Ok((transform, name, owned_by_opt, idle_opt, roam_opt, catch_opt)) =
            parent_query.get(child.parent())
        else {
            continue;
        };

        debug.0.clear();

        debug.0.write_fmt(format_args!("Name: {}\n", name)).ok();

        let position = transform.translation.truncate();

        if let Some(owner) = owned_by_opt {
            debug.0.write_fmt(format_args!("Owner: {}\n", owner.0)).ok();

            if let Ok(owner_transform) = owner_query.get(owner.0) {
                let distance = position.distance(owner_transform.translation.truncate());

                debug
                    .0
                    .write_fmt(format_args!("Distance to Owner: {:.2}\n", distance))
                    .ok();
            }
        }

        fn push_state<TState>(opt: Option<&TState>, buf: &mut String)
        where
            TState: StateDebug,
        {
            if let Some(s) = opt {
                buf.write_fmt(format_args!("{}\n", s.debug())).ok();
            }
        }

        push_state(idle_opt, &mut debug.0);
        push_state(roam_opt, &mut debug.0);
        push_state(catch_opt, &mut debug.0);
    }
}
