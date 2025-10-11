prelude!();

use std::sync::Arc;

use bevy_landmass::{NavMeshHandle, debug::Landmass2dDebugPlugin};

use crate::modules::{actor::FaceDirection, world::map::*};

pub struct PathfindingPlugin;
game_module_build!(PathfindingPlugin);

impl GameModule for PathfindingPlugin {
    fn resources(&self, app: &mut App) {
        app.init_resource::<ArchipelagoId>();
    }

    fn plugins(&self, app: &mut App) {
        app.add_plugins(Landmass2dPlugin::default());

        // app.add_plugins(Landmass2dDebugPlugin::default());
    }

    fn systems(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppLoadingState::PopulateChunkData),
            (attach_archipelago_to_world_root,).in_set(AppLoadingSystems::PopulateChunkData),
        );

        app.on_playing_game_update((
            on_chunk_loaded,
            update_agent_velocity,
            apply_velocities_to_controllers,
            update_agent_tracker,
            detect_stuck_agents,
        ));
    }
}

#[derive(Resource, Clone, Default, Debug)]
pub struct ArchipelagoId(pub Option<Entity>);

fn attach_archipelago_to_world_root(
    mut commands: Commands,
    roots: Query<Entity, Added<WorldRoot>>,
    mut archipelago_id: ResMut<ArchipelagoId>,
) {
    for root in roots {
        let id = commands
            .spawn((
                ChildOf(root),
                Archipelago2d::new(ArchipelagoOptions::from_agent_radius(
                    DEFAULT_ACTOR_COLLIDER_RADIUS,
                )),
            ))
            .id();

        info!("Archipelago id set to {}", id);
        archipelago_id.0 = Some(id);
    }
}

fn on_chunk_loaded(
    mut commands: Commands,
    chunk_data: Res<ChunkDataCollection>,
    mut chunk_loaded: MessageReader<ChunkLoaded>,
    mut nav_meshes: ResMut<Assets<NavMesh2d>>,
    archipelago_id: Res<ArchipelagoId>,
) {
    let Some(archipelago_id) = archipelago_id.0 else {
        return;
    };

    for event in chunk_loaded.read() {
        let Some(data) = chunk_data.0.get(&event.chunk_id) else {
            continue;
        };

        let nav_mesh_handle = nav_meshes.reserve_handle();

        commands.spawn(Island2dBundle {
            island: Island,
            archipelago_ref: ArchipelagoRef2d::new(archipelago_id),
            nav_mesh: NavMeshHandle(nav_mesh_handle.clone()),
        });

        let nav_mesh = Arc::new(data.get_nav_mesh().validate().expect("is valid"));
        let _ = nav_meshes.insert(&nav_mesh_handle, NavMesh2d { nav_mesh });
    }
}

fn update_agent_velocity(
    mut agent_query: Query<(
        &mut Velocity2d,
        &AgentDesiredVelocity2d,
        Option<&mut FaceDirection>,
    )>,
) {
    for (mut velocity, desired_velocity, face_direction_opt) in agent_query.iter_mut() {
        velocity.velocity = desired_velocity.velocity();

        if let Some(mut face_direction) = face_direction_opt {
            let velocity_len = velocity.velocity.length();
            if velocity_len < 0.1 {
                continue;
            }

            face_direction.0 = Direction::from_vec2(velocity.velocity.normalize_or_zero());
        }
    }
}

fn apply_velocities_to_controllers(
    mut query: Query<(
        &Velocity2d,
        &GlobalTransform,
        &mut KinematicCharacterController,
    )>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (velocity, global_transform, mut controller) in query.iter_mut() {
        let local_velocity = global_transform
            .affine()
            .inverse()
            .transform_vector3(velocity.velocity.extend(0.0))
            .truncate();

        controller.translation = Some(local_velocity * delta);
    }
}

#[derive(Component, Clone, Debug)]
pub struct AgentTracker {
    pub last_position: Vec2,
    pub recent_speeds: Vec<(f32, f32)>,
}

impl Default for AgentTracker {
    fn default() -> Self {
        Self {
            last_position: Vec2::NAN,
            recent_speeds: Vec::new(),
        }
    }
}

impl AgentTracker {
    pub fn number_of_samples(&self) -> usize {
        self.recent_speeds.len()
    }

    pub fn average_speed(&self, now: f32) -> f32 {
        let cutoff = now - 1.0;
        let valid = self
            .recent_speeds
            .iter()
            .filter(|(t, _)| *t >= cutoff)
            .map(|(_, s)| *s)
            .collect::<Vec<_>>();

        if valid.is_empty() {
            0.0
        } else {
            valid.iter().sum::<f32>() / valid.len() as f32
        }
    }
}

fn update_agent_tracker(
    time: Res<Time>,
    mut query: Query<(&Transform, &AgentState, &mut AgentTracker)>,
) {
    let now = time.elapsed_secs();
    let delta = time.delta_secs();

    for (transform, state, mut tracker) in query.iter_mut() {
        if tracker.last_position.is_nan() {
            tracker.last_position = transform.translation.truncate();
            continue;
        }

        if state != &AgentState::Moving {
            tracker.recent_speeds.clear();
            tracker.last_position = transform.translation.truncate();
            continue;
        }

        let current_pos = transform.translation.truncate();
        let distance = current_pos.distance(tracker.last_position);
        let speed = distance / delta;

        tracker.recent_speeds.push((now, speed));
        tracker.recent_speeds.retain(|(t, _)| *t >= now - 1.0);

        tracker.last_position = current_pos;
    }
}

fn detect_stuck_agents(
    mut query: Query<(&AgentTracker, &AgentState, &mut AgentTarget2d)>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();

    for (tracker, state, mut target) in query.iter_mut() {
        if tracker.number_of_samples() < 15 {
            continue;
        }

        if let AgentTarget2d::None = *target {
            continue;
        }

        if state != &AgentState::Moving {
            continue;
        }

        let average_speed = tracker.average_speed(now);

        if average_speed < 5.0 {
            *target = AgentTarget2d::None;
        }
    }
}
