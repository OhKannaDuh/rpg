prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateCatchingUpToOwner;

impl CreatureStateCatchingUpToOwner {
    pub fn process(mut creatures: Query<(&OwnedBy, &mut AgentTarget2d), With<Self>>) {
        for (owner, mut agent_target) in creatures.iter_mut() {
            if *agent_target != AgentTarget2d::Entity(owner.0) {
                *agent_target = AgentTarget2d::Entity(owner.0);
            }
        }
    }

    pub fn is_outside_of_wander_range() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>,
               creatures: Query<(&OwnedBy, &Transform, &CreatureBehaviour)>,
               transforms: Query<&Transform>| {
            let Ok((owner, transform, behaviour)) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            let Ok(owner_transform) = transforms.get(owner.0) else {
                error!("Unable to get owner transform of {:?}", owner.0);
                return Err(());
            };

            let distance = transform
                .translation
                .truncate()
                .distance_squared(owner_transform.translation.truncate());

            if distance > behaviour.roaming.max_wander_distance_squared {
                Ok(())
            } else {
                Err(())
            }
        })
        .into_trigger()
    }

    pub fn has_caught_up_to_owner() -> impl EntityTrigger<Out = Result<(), ()>> {
        (move |In(entity): In<Entity>,
               creatures: Query<(&OwnedBy, &Transform, &CreatureBehaviour)>,
               transforms: Query<&Transform>| {
            let Ok((owner, transform, behaviour)) = creatures.get(entity) else {
                error!("Unable to get creature data of {}", entity);
                return Err(());
            };

            let Ok(owner_transform) = transforms.get(owner.0) else {
                error!("Unable to get owner transform of {:?}", owner.0);
                return Err(());
            };

            let distance = transform
                .translation
                .truncate()
                .distance_squared(owner_transform.translation.truncate());

            if distance <= behaviour.roaming.min_wander_distance_squared {
                Ok(())
            } else {
                Err(())
            }
        })
        .into_trigger()
    }
}

impl StateMachineBuilder for CreatureStateCatchingUpToOwner {
    fn build(&self, sm: StateMachine) -> StateMachine {
        // Transition to catching up state if outside wander range
        sm.trans::<NotState<CreatureStateCatchingUpToOwner>, _>(
            CreatureStateCatchingUpToOwner::is_outside_of_wander_range(),
            CreatureStateCatchingUpToOwner,
        )
        // Transition to idle state if caught up to owner
        .trans::<CreatureStateCatchingUpToOwner, _>(
            CreatureStateCatchingUpToOwner::has_caught_up_to_owner(),
            CreatureStateIdle,
        )
    }
}
