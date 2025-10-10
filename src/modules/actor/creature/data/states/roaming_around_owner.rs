prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateRoamingAroundOwner {
    pub state: CreatureStateRoamingAroundOwnerState,
    pub time_in_state: f32,

    pub wander_target: Option<Vec2>,
    pub min_wander_distance: f32, // Based on owner
    pub max_wander_distance: f32, // Based on owner
}

#[derive(Clone, Default)]
pub enum CreatureStateRoamingAroundOwnerState {
    #[default]
    Idling,
    Wandering,
}

impl CreatureStateRoamingAroundOwner {
    pub fn process(
        mut creatures: Query<
            (
                &Transform,
                &OwnedBy,
                &mut KinematicCharacterController,
                &mut Self,
            ),
            With<OwnedBy>,
        >,
        owner_position_query: Query<&Transform, Without<OwnedBy>>,
        time: Res<Time>,
    ) {
        const MIN_IDLE_TIME: f32 = 1.0;
        const MAX_IDLE_TIME: f32 = 3.0;

        const MIN_WANDER_CHANCE: f32 = 0.1;
        const MAX_WANDER_CHANCE: f32 = 0.8;

        let delta = time.delta_secs();
        let distance_per_frame = DEFAULT_ACTOR_SPEED * delta;

        for (transform, owner, mut controller, mut state) in creatures.iter_mut() {
            state.time_in_state += delta;

            if state.is_idling() && state.time_in_state >= MIN_IDLE_TIME {
                let idle_progress = ((state.time_in_state - MIN_IDLE_TIME)
                    / (MAX_IDLE_TIME - MIN_IDLE_TIME))
                    .clamp(0.0, 1.0);

                let p_per_sec =
                    MIN_WANDER_CHANCE + idle_progress * (MAX_WANDER_CHANCE - MIN_WANDER_CHANCE);

                let p_frame = 1.0 - (1.0 - p_per_sec).powf(delta.max(0.0));

                if rand::random::<f32>() < p_frame {
                    let owner_id = owner.0;

                    let Some(owner_transform) = owner_position_query.get(owner_id).ok() else {
                        continue;
                    };

                    let owner_position = owner_transform.translation;

                    use rand::Rng;

                    let mut rng = rand::rng();

                    let theta = rng.random_range(0.0..std::f32::consts::TAU);

                    let min_r = state.min_wander_distance;
                    let max_r = state.max_wander_distance;
                    let r = (rng.random_range(min_r * min_r..max_r * max_r)).sqrt();

                    let dir = Vec2::new(theta.cos(), theta.sin());
                    let wander_position = owner_position.truncate() + dir * r;

                    state.wander(wander_position);
                }
            }

            if state.is_wandering() {
                let Some(target) = state.wander_target else {
                    state.idle();
                    continue;
                };

                let distance = transform.translation.truncate().distance(target);
                let arrival_radius = (DEFAULT_ACTOR_SPEED * delta) * 2.0;

                if distance <= arrival_radius {
                    state.idle();
                    continue;
                }

                let direction = (target - transform.translation.truncate()).normalize_or_zero();
                controller.translation = Some(direction * DEFAULT_ACTOR_SPEED * delta);
            }
        }
    }

    fn transition(&mut self, new_state: CreatureStateRoamingAroundOwnerState) {
        self.state = new_state;
        self.time_in_state = 0.0;
        self.wander_target = None;
    }

    fn is_idling(&self) -> bool {
        matches!(self.state, CreatureStateRoamingAroundOwnerState::Idling)
    }

    fn idle(&mut self) {
        self.transition(CreatureStateRoamingAroundOwnerState::Idling);
    }

    pub fn is_wandering(&self) -> bool {
        matches!(self.state, CreatureStateRoamingAroundOwnerState::Wandering)
    }

    fn wander(&mut self, target: Vec2) {
        self.transition(CreatureStateRoamingAroundOwnerState::Wandering);
        self.wander_target = Some(target);
    }
}

impl StateDebug for CreatureStateRoamingAroundOwner {
    fn debug(&self) -> String {
        let mut output = String::new();
        output.write_str("State: Roaming Around Owner").ok();
        output
            .write_fmt(format_args!(
                "\n  Time in Substate: {:.2}",
                self.time_in_state
            ))
            .ok();

        if self.is_idling() {
            output.write_str("\n  Substate: Idle").ok();
        }

        if self.is_wandering() {
            output.write_str("\n  Substate: Wandering").ok();

            if let Some(target) = self.wander_target {
                output
                    .write_fmt(format_args!(
                        "\n  Destination: {:.2}, {:.2}",
                        target.x, target.y
                    ))
                    .ok();
            }
        }

        output
    }
}

impl CreatureStateRoamingAroundOwner {
    pub fn render_path_gizmos(
        creatures: Query<(&Transform, &Self), With<OwnedBy>>,
        mut gizmos: Gizmos,
    ) {
        for (transform, state) in creatures.iter() {
            if !state.is_wandering() {
                continue;
            }

            let from = transform.translation.truncate();
            let Some(to) = state.wander_target else {
                continue;
            };

            gizmos.line_2d(from, to, Color::srgb(0.0, 0.0, 1.0));
        }
    }
}
