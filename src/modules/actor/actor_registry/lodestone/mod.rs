prelude!();

use crate::modules::actor::actor_registry::*;
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
                Sprite {
                    image: assets.lodestone.clone(),
                    custom_size: Some(source.size()),
                    ..Default::default()
                },
                source.transform(),
                FootPosition(-21.0),
            ))
            .id();

        commands.spawn((
            ChildOf(lodestone),
            RigidBody::Fixed,
            Collider::capsule_x(8.0, 6.0),
            Transform::from_xyz(0.0, -14.5, 0.0),
        ));

        lodestone
    }
}

fn handle_lodestone_teleport_messages(
    mut messages: MessageReader<TeleportToLodestone>,
    mut targets: Query<&mut Transform, With<Actor>>,
    global_map_entities: Res<GlobalMapEntities>,
) {
    for teleport in messages.read() {
        let Ok(mut transform) = targets.get_mut(teleport.target) else {
            warn!("Unable to get teleport target {}", teleport.target);
            continue;
        };

        let lodestones = global_map_entities.by_def_identifier("LODESTONE".to_string());

        let Some(lodestone) =
            find_lodestone_by_identifier(&lodestones, &teleport.lodestone_identifier)
        else {
            continue;
        };

        let z = transform.translation.z;
        transform.translation = lodestone.center().as_vec2().extend(z);
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
