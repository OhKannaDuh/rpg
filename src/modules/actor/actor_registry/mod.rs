prelude!();
plugins!((lodestone, LodestonePlugin),);

use bevy::ecs::system::SystemId;

use crate::modules::{actor::ActorAssets, world::map::*};

pub struct ActorRegistryPlugin;
game_module_build!(ActorRegistryPlugin);

impl GameModule for ActorRegistryPlugin {
    fn resources(&self, app: &mut App) {
        app.init_resource::<ActorManager>()
            .init_resource::<ActorRegistry>();
    }

    fn plugins(&self, app: &mut App) {
        app.add_plugins(LodestonePlugin);
    }

    fn systems(&self, app: &mut App) {
        app.on_playing_game_update(on_chunk_loaded);
    }
}

fn on_chunk_loaded(
    mut commands: Commands,
    chunk_data: Res<ChunkDataCollection>,
    mut chunk_loaded: MessageReader<ChunkLoaded>,
    registry: Res<ActorRegistry>,
    mut manager: ResMut<ActorManager>,
    actor_assets: Res<ActorAssets>,
) {
    for event in chunk_loaded.read() {
        let Some(data) = chunk_data.0.get(&event.chunk_id) else {
            continue;
        };

        for layer in data.entity_layers.iter() {
            for data in layer.entities.values() {
                if manager.has(&data.id) {
                    continue;
                }

                let Some(factory) = registry.get_factory(data.def.identifier.clone()) else {
                    continue;
                };

                let entity = factory(&mut commands, data, &actor_assets);
                manager.mark_spawned(&data.id, entity);
            }
        }
    }
}

type ActorSpawner = fn(&mut Commands, &MapEntity, &ActorAssets) -> Entity;

#[derive(Resource, Clone, Default, Debug)]
pub struct ActorManager(pub HashMap<MapEntityId, Entity>);

impl ActorManager {
    pub fn has(&self, id: &MapEntityId) -> bool {
        self.0.contains_key(id)
    }

    pub fn mark_spawned(&mut self, id: &MapEntityId, entity: Entity) {
        self.0.insert(id.clone(), entity);
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct ActorRegistry(HashMap<String, ActorSpawner>);

impl ActorRegistry {
    pub fn register(&mut self, name: String, spawner: ActorSpawner) {
        self.0.insert(name, spawner);
    }

    pub fn get_factory(&self, name: String) -> Option<ActorSpawner> {
        self.0.get(&name).copied()
    }
}

pub trait ActorRegistryExt {
    fn register_actor_factory(&mut self, id: String, spawner: ActorSpawner) -> &mut Self;
}

impl ActorRegistryExt for App {
    fn register_actor_factory(&mut self, id: String, spawner: ActorSpawner) -> &mut Self {
        self.add_systems(
            OnEnter(AppLoadingState::PopulateDefinitions),
            (move |mut actor_registry: ResMut<ActorRegistry>| {
                info!("Registering actor factory for {}", id);
                actor_registry.register(id.clone(), spawner);
            },)
                .in_set(AppLoadingSystems::PopulateDefinitions),
        )
    }
}
