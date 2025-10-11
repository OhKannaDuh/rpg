prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateRoamingAroundOwnerIdle {
    pub time_in_state: f32,
    pub target_time: Option<f32>,
}

impl CreatureStateRoamingAroundOwnerIdle {
    pub fn process(
        mut creatures: Query<(&CreatureBehaviour, &mut WyRand, &mut Self), With<OwnedBy>>,
        time: Res<Time>,
    ) {
        let delta = time.delta_secs();

        for (behaviour, mut rng, mut state) in creatures.iter_mut() {
            if state.target_time.is_none() {
                let min = behaviour.roaming.min_idle_time;
                let max = behaviour.roaming.max_idle_time;

                state.target_time = Some(rng.random_range(min..max));
            }

            state.time_in_state += delta;
        }
    }

    pub fn should_transition_to_roaming() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>, creatures: Query<&Self>| {
            let Ok(state) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            let Some(target_time) = state.target_time else {
                return Err(());
            };

            if state.time_in_state >= target_time {
                Ok(())
            } else {
                Err(())
            }
        })
        .into_trigger()
    }
}

impl StateMachineBuilder for CreatureStateRoamingAroundOwnerIdle {
    fn build(&self, sm: StateMachine) -> StateMachine {
        // go back to CreatureStateRoamingAroundOwnerRoaming when we have idled long enough
        sm.trans::<Self, _>(
            Self::should_transition_to_roaming(),
            CreatureStateRoamingAroundOwnerRoaming::default(),
        )
    }
}
