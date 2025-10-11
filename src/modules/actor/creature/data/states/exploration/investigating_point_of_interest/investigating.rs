prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateInvestigatingPointOfInterestInvestigating {
    pub time_in_state: f32,
    pub target_time: Option<f32>,
}

impl CreatureStateInvestigatingPointOfInterestInvestigating {
    pub fn process(
        mut creatures: Query<(&CreatureBehaviour, &mut WyRand, &mut Self)>,
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

    pub fn is_done_investigating() -> impl EntityTrigger<Out = Result<(), ()>> {
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

impl StateMachineBuilder for CreatureStateInvestigatingPointOfInterestInvestigating {
    fn build(&self, sm: StateMachine) -> StateMachine {
        // go back to CreatureStateIdle when we have finished investigating
        sm.explorer_events_for::<Self>()
            .trans::<Self, _>(Self::is_done_investigating(), CreatureStateIdle)
    }
}
