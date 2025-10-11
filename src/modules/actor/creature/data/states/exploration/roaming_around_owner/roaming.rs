prelude!();

use bevy_rapier2d::na::ComplexField;

use crate::modules::actor::creature::*;
use std::f32::consts::TAU;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateRoamingAroundOwnerRoaming {
    pub started_roaming: bool,
}

impl CreatureStateRoamingAroundOwnerRoaming {
    pub fn on_enter() {}

    pub fn process(
        mut creatures: Query<
            (
                &OwnedBy,
                &CreatureBehaviour,
                &AgentState,
                &mut WyRand,
                &mut AgentTarget2d,
                &mut Self,
            ),
            With<OwnedBy>,
        >,
        transforms: Query<&Transform>,
        archipelago: Single<&Archipelago2d>,
        time: Res<Time>,
    ) {
        let archipelago = archipelago.into_inner();

        for (owner, behaviour, agent_state, mut rng, mut target, mut state) in creatures.iter_mut()
        {
            if *target != AgentTarget2d::None {
                if !state.started_roaming && agent_state == &AgentState::Moving {
                    state.started_roaming = true;
                }

                continue;
            }

            let Some(owner_transform) = transforms.get(owner.0).ok() else {
                error!("Unable to get owner transform of {:?}", owner.0);
                continue;
            };

            let min = behaviour.roaming.min_wander_distance_squared;
            let max = behaviour.roaming.max_wander_distance_squared;
            let radius = rng.random_range(min..max).sqrt();

            let theta = rng.random_range(0.0..TAU);
            let dir = Vec2::new(theta.cos(), theta.sin());

            let destination = owner_transform.translation.truncate() + dir * radius;
            if let Ok(sampled) = archipelago.sample_point(destination, &TILE_SIZE) {
                *target = AgentTarget2d::Point(sampled.point());
            }
        }
    }

    pub fn has_reached_destination() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>, creatures: Query<(&AgentState, &AgentTarget2d, &Self)>| {
            let Ok((agent_state, target, state)) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            if state.started_roaming && agent_state != &AgentState::Moving {
                return Ok(());
            }

            if *target == AgentTarget2d::None {
                return Ok(());
            }

            Err(())
        })
        .into_trigger()
    }
}

impl StateMachineBuilder for CreatureStateRoamingAroundOwnerRoaming {
    fn build(&self, sm: StateMachine) -> StateMachine {
        sm.explorer_events_for::<Self>()
            // go back to CreatureStateIdle when we our roaming destination is reached
            .trans::<Self, _>(Self::has_reached_destination(), CreatureStateIdle)
    }
}
