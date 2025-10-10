prelude!();

use crate::modules::actor::creature::*;

#[derive(Clone, Component, Default)]
#[component(storage = "SparseSet")]
pub struct CreatureStateCatchingUpToOwner;

impl CreatureStateCatchingUpToOwner {
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
        let delta = time.delta_secs();

        for (transform, owner, mut controller, mut _state) in creatures.iter_mut() {
            let owner_id = owner.0;

            let Some(owner_transform) = owner_position_query.get(owner_id).ok() else {
                continue;
            };

            let owner_position = owner_transform.translation;

            let distance = transform
                .translation
                .truncate()
                .distance(owner_position.truncate());
            if distance <= TILE_SIZE * 2.0 {
                continue;
            }

            let direction =
                (owner_position.truncate() - transform.translation.truncate()).normalize_or_zero();
            controller.translation = Some(direction * DEFAULT_ACTOR_SPEED * delta);
        }
    }

    pub fn debug(&self) -> String {
        let mut output = String::new();
        output.write_str("State: Catching up to Owner").ok();

        output
    }
}

impl StateDebug for CreatureStateCatchingUpToOwner {
    fn debug(&self) -> String {
        let mut output = String::new();
        output.write_str("State: Catching up to Owner").ok();

        output
    }
}
