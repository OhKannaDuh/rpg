prelude!();
public!(exploration, social);

use crate::modules::actor::creature::*;

#[derive(Clone, Component)]
#[component(storage = "SparseSet")]
pub struct CreatureStateIdle;

impl StateMachineBuilder for CreatureStateIdle {
    fn build(&self, sm: StateMachine) -> StateMachine {
        sm.trans::<Self, _>(
            // Dummy, to be replaced later by ai logic
            CreatureStateHelpers::within_distance_to_owner(f32::INFINITY),
            CreatureStateRoamingAroundOwnerIdle::default(),
        )
    }
}

pub struct CreatureStateHelpers;

impl CreatureStateHelpers {
    pub fn within_distance_to_owner(
        range_squared: f32,
    ) -> impl EntityTrigger<Out = Result<f32, f32>> {
        (move |In(entity): In<Entity>,
               creatures_query: Query<(&Transform, &OwnedBy)>,
               owners_query: Query<&Transform>| {
            let Some((creature_transform, owner)) = creatures_query.get(entity).ok() else {
                error!("Unable to get creature transform of {}", entity);
                return Err(f32::INFINITY);
            };

            let Some(owner_transform) = owners_query.get(owner.0).ok() else {
                error!("Unable to get owner transform of {}", entity);
                return Err(f32::INFINITY);
            };

            let distance = creature_transform
                .translation
                .truncate()
                .distance_squared(owner_transform.translation.truncate());

            if distance <= range_squared {
                Ok(distance)
            } else {
                Err(distance)
            }
        })
        .into_trigger()
    }
}
