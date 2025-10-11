prelude!();
public!(components, data, systems, creatures);

use bevy_trait_query::RegisterExt;

use crate::modules::actor::animation::*;
use crate::modules::actor::*;
use crate::modules::world::pathfinding::*;

pub struct CreaturePlugin;
game_module_build!(CreaturePlugin);

impl GameModule for CreaturePlugin {
    fn plugins(&self, app: &mut App) {
        app.add_plugins(StateMachinePlugin::default());
    }

    fn messages(&self, app: &mut App) {
        app.add_message::<SpawnCreatureMessage>();
    }

    fn types(&self, app: &mut App) {
        app.register_component_as::<dyn SocialDrainer, CreatureNeeds>();
        app.register_component_as::<dyn SocialRestorer, IsSocialising>();

        app.register_component_as::<dyn ExplorationDrainer, CreatureNeeds>();
        app.register_component_as::<dyn ExplorationRestorer, IsExploring>();
    }

    fn systems(&self, app: &mut App) {
        app.on_playing_game_update(CreatureStateExploration::systems());

        app.on_playing_game_update(handle_spawn_creature_messages);

        app.on_playing_game_update((
            update_creature_animation_controllers,
            advance_animators::<CreatureAnimationState>,
        ));
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(
    Actor,
    CreatureBehaviour,
    CreatureNeeds,
    CreatureStats,
    CreatureRelationships
)]
pub struct Creature;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Creature)]
pub struct OwnedBy(pub Entity);

#[derive(Message)]
pub struct SpawnCreatureMessage {
    pub name: String,
    pub position: Vec2,

    pub color: Color,

    pub owner: Option<Entity>,
    pub spawn_radius: Option<f32>,
    pub behaviour_preset: Option<CreatureBehaviourPresetType>,
}

fn handle_spawn_creature_messages(
    mut commands: Commands,
    mut messages: MessageReader<SpawnCreatureMessage>,
    mut rng: Single<&mut WyRand, With<GlobalRng>>,
    archipelago_id: Res<ArchipelagoId>,

    assets: Res<ActorAssets>,
) {
    let Some(archipelago_id) = archipelago_id.0 else {
        return;
    };

    for message in messages.read() {
        info!("Spawning creature {}", message.name);

        let mut position = message.position;

        if let Some(radius) = message.spawn_radius {
            let angle = rand::random::<f32>() * std::f32::consts::TAU;
            let offset = rand::random::<f32>() * radius;
            position += Vec2::new(angle.cos() * offset, angle.sin() * offset);
        }

        let mut creature = commands.spawn((
            Name::new(message.name.clone()),
            Creature,
            Transform::from_xyz(position.x, position.y, 0.0),
            rng.fork_seed(),
            get_state_machine(),
        ));

        creature.insert((
            Sprite {
                image: assets.images["textures/entities/slime.png"].clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.slime_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            FaceDirection::default(),
            CreatureAnimationController,
            Animator::<CreatureAnimationState>::new(CreatureAnimationState::IdleDown),
            AnimationBank::<CreatureAnimationState> {
                clips: HashMap::from_iter(vec![
                    AnimationBank::idle_entry(CreatureAnimationState::IdleDown, 0),
                    AnimationBank::idle_entry(CreatureAnimationState::IdleUp, 3),
                    AnimationBank::idle_entry(CreatureAnimationState::IdleRight, 6),
                    AnimationBank::idle_entry(CreatureAnimationState::IdleLeft, 9),
                    AnimationBank::linear_entry(CreatureAnimationState::MoveDown, 0, 3, 0.15),
                    AnimationBank::linear_entry(CreatureAnimationState::MoveUp, 3, 3, 0.15),
                    AnimationBank::linear_entry(CreatureAnimationState::MoveRight, 6, 3, 0.15),
                    AnimationBank::linear_entry(CreatureAnimationState::MoveLeft, 9, 3, 0.15),
                ]),
            },
            FootPosition(-TILE_SIZE / 2.0),
        ));

        creature.insert((
            ENTITY_COLLISION,
            RigidBody::KinematicPositionBased,
            Collider::ball(DEFAULT_ACTOR_COLLIDER_RADIUS),
            Restitution::coefficient(DEFAULT_ACTOR_RESTITUTION),
            KinematicCharacterController {
                apply_impulse_to_dynamic_bodies: false,
                filter_groups: Some(ENTITY_COLLISION),
                ..Default::default()
            },
        ));

        creature.insert((
            Agent2dBundle {
                agent: Default::default(),
                settings: AgentSettings {
                    radius: DEFAULT_ACTOR_COLLIDER_RADIUS,
                    desired_speed: DEFAULT_ACTOR_SPEED,
                    max_speed: DEFAULT_ACTOR_SPEED * 1.5,
                },
                archipelago_ref: ArchipelagoRef2d::new(archipelago_id),
            },
            AgentTracker::default(),
        ));

        if let Some(owner) = message.owner {
            creature.insert(OwnedBy(owner));
        }

        if let Some(roam_behaviour) = message.behaviour_preset {
            creature.insert(roam_behaviour.get_behaviour());
        }
    }
}

fn get_state_machine() -> impl Bundle {
    let builders: Vec<Box<dyn StateMachineBuilder + Send + Sync>> = vec![
        Box::new(|sm: StateMachine| {
            // Clear agent data when we transition to any state
            sm.on_enter::<AnyState>(|entity| {
                entity.insert((AgentTarget2d::None, AgentDesiredVelocity2d::default()));
            })
            .set_trans_logging(true)
        }),
        Box::new(CreatureStateIdle),
        Box::new(CreatureStateRoamingAroundOwnerIdle::default()),
        Box::new(CreatureStateRoamingAroundOwnerRoaming::default()),
        Box::new(CreatureStateCatchingUpToOwner),
        Box::new(CreatureStateInvestigatingPointOfInterestMoving::default()),
        Box::new(CreatureStateInvestigatingPointOfInterestInvestigating::default()),
    ];

    (
        CreatureStateIdle,
        StateMachineFactory::from_many(builders).build(StateMachine::default()),
    )
}
