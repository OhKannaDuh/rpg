prelude!();
public!(idle, roaming_around_owner, catching_up_to_owner);

use crate::modules::actor::creature::*;

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

    pub fn outside_distance_to_owner(
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

            if distance > range_squared {
                Ok(distance)
            } else {
                Err(distance)
            }
        })
        .into_trigger()
    }
}
