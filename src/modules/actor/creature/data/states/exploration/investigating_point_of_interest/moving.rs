prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateInvestigatingPointOfInterestMoving {
    pub target: Option<Entity>,
}

impl CreatureStateInvestigatingPointOfInterestMoving {
    pub fn process(mut creatures: Query<(&Self, &mut AgentTarget2d)>) {
        for (state, mut agent_target) in creatures.iter_mut() {
            let Some(target) = state.target else {
                continue;
            };

            if *agent_target != AgentTarget2d::Entity(target) {
                *agent_target = AgentTarget2d::Entity(target);
            }
        }
    }

    pub fn has_reached_target() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>,
               creatures: Query<(&Self, &Transform)>,
               transforms: Query<(&Transform, &PointOfInterest)>| {
            let Ok((state, transform)) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            let Some(target) = state.target else {
                return Err(());
            };

            let Ok((target_transform, point_of_interest)) = transforms.get(target) else {
                error!("Unable to get target transform of {:?}", target);
                return Err(());
            };

            let distance = transform
                .translation
                .truncate()
                .distance_squared(target_transform.translation.truncate());

            if distance < point_of_interest.observation_radius_squared {
                Ok(())
            } else {
                Err(())
            }
        })
        .into_trigger()
    }

    pub fn point_of_interest_exists() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>,
               creatures: Query<&Self>,
               points_of_interest: Query<&PointOfInterest>| {
            let Ok(state) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            let Some(target) = state.target else {
                return Err(());
            };

            if points_of_interest.get(target).is_ok() {
                Ok(())
            } else {
                Err(())
            }
        })
        .into_trigger()
    }
}

impl StateMachineBuilder for CreatureStateInvestigatingPointOfInterestMoving {
    fn build(&self, sm: StateMachine) -> StateMachine {
        // Transition to idle state if poi no longer exists
        sm.explorer_events_for::<Self>()
            .trans::<Self, _>(Self::point_of_interest_exists().not(), CreatureStateIdle)
            // Transition to investigating state if we have reached the target
            .trans::<Self, _>(
                Self::has_reached_target(),
                CreatureStateInvestigatingPointOfInterestInvestigating::default(),
            )
    }
}
