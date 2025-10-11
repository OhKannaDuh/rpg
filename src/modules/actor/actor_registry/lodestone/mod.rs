prelude!();

use bevy::camera::primitives::Aabb;

use crate::modules::actor::actor_registry::*;
use crate::modules::actor::animation::*;
use crate::modules::actor::player::Player;
use crate::modules::actor::*;

pub struct LodestonePlugin;
game_module_build!(LodestonePlugin);

impl GameModule for LodestonePlugin {
    fn messages(&self, app: &mut App) {
        app.add_message::<TeleportToLodestone>();
    }

    fn systems(&self, app: &mut App) {
        app.register_actor_factory("LODESTONE".to_string(), Lodestone::spawn);

        app.on_playing_game_update((debug_lodestone_teleports,));

        app.add_systems(
            PostUpdate,
            handle_lodestone_teleport_messages
                .after(PhysicsSet::Writeback)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn debug_lodestone_teleports(
    mut messages: MessageWriter<TeleportToLodestone>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player: Single<Entity, With<Player>>,
) {
    let entity = player.into_inner();

    if keyboard.just_pressed(KeyCode::Digit1) {
        info!("Teleporting player to lodestone 1");
        messages.write(TeleportToLodestone {
            target: entity,
            lodestone_identifier: "TESTING_WORLD_LODESTONE_1".to_string(),
        });
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        info!("Teleporting player to lodestone 2");
        messages.write(TeleportToLodestone {
            target: entity,
            lodestone_identifier: "TESTING_WORLD_LODESTONE_2".to_string(),
        });
    }
}

#[derive(Message)]
pub struct TeleportToLodestone {
    pub target: Entity,
    pub lodestone_identifier: String,
}

#[derive(Component, Clone, Copy, Debug, Default)]
#[require(Actor)]
pub struct Lodestone;

impl Lodestone {
    fn spawn(commands: &mut Commands, source: &MapEntity, assets: &ActorAssets) -> Entity {
        let size = source.size();
        let transform = source.transform();

        info!(
            "Spawning lodestone at {}, {} of size {}, {}",
            transform.translation.x, transform.translation.y, size.x, size.y
        );

        let lodestone = commands
            .spawn((
                Lodestone,
                source.transform(),
                FootPosition(-21.0),
                Sprite {
                    image: assets.images["textures/entities/lodestone.png"].clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: assets.lodestone_layout.clone(),
                        index: 0,
                    }),
                    custom_size: Some(source.size()),
                    ..Default::default()
                },
                Animator::<String>::new("default".to_string()),
                AnimationBank::<String> {
                    clips: HashMap::from_iter(vec![(
                        "default".to_string(),
                        ActorAnimation {
                            frames: vec![
                                AnimationFrameMeta::new(0, 0.10),
                                AnimationFrameMeta::new(1, 0.11),
                                AnimationFrameMeta::new(2, 0.14),
                                AnimationFrameMeta::new(3, 0.22),
                                AnimationFrameMeta::new(4, 0.17),
                                AnimationFrameMeta::new(5, 0.14),
                                AnimationFrameMeta::new(6, 0.13),
                                AnimationFrameMeta::new(7, 0.17),
                            ],
                            frame: 0,
                        },
                    )]),
                },
            ))
            .id();

        commands.spawn((
            ChildOf(lodestone),
            RigidBody::Fixed,
            Collider::capsule_x(8.0, 6.0),
            Transform::from_xyz(0.0, -17.0, 0.0),
        ));

        lodestone
    }
}

fn handle_lodestone_teleport_messages(
    mut messages: MessageReader<TeleportToLodestone>,
    mut targets: Query<(&mut Transform, &Aabb), With<Actor>>,
    global_map_entities: Res<GlobalMapEntities>,
) {
    for teleport in messages.read() {
        let Ok((mut transform, aabb)) = targets.get_mut(teleport.target) else {
            warn!("Unable to get teleport target {}", teleport.target);
            continue;
        };

        let lodestones = global_map_entities.by_def_identifier("LODESTONE".to_string());

        let Some(lodestone) =
            find_lodestone_by_identifier(&lodestones, &teleport.lodestone_identifier)
        else {
            continue;
        };

        // Destination south of the lodestone
        let lodestone_half_height = 24.0;
        let padding = 4.0;
        let mut target_position = lodestone.center().as_vec2();
        target_position.y -= lodestone_half_height + aabb.half_extents.y + padding;

        transform.translation = target_position.extend(transform.translation.z);
    }
}

fn find_lodestone_by_identifier<'a>(
    lodestones: &'a [&'a MapEntity],
    identifier: &str,
) -> Option<&'a MapEntity> {
    lodestones.iter().copied().find(|e| {
        e.fields
            .get("IDENTIFIER")
            .is_some_and(|f| f.as_str() == identifier)
    })
}
