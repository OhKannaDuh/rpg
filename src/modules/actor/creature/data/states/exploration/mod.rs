prelude!();
public!(
    catching_up_to_owner,
    roaming_around_owner,
    investigating_point_of_interest
);

use bevy::ecs::system::ScheduleSystem;

use crate::modules::actor::creature::IsExploring;

pub struct CreatureStateExploration;

impl CreatureStateExploration {
    pub fn attach_is_exploring(entity: &mut EntityCommands) {
        entity.insert((IsExploring,));
    }

    pub fn deattach_is_exploring(entity: &mut EntityCommands) {
        entity.remove::<IsExploring>();
    }

    pub fn systems() -> impl IntoScheduleConfigs<ScheduleSystem, ()> {
        (
            CreatureStateRoamingAroundOwnerIdle::process,
            CreatureStateRoamingAroundOwnerRoaming::process,
            CreatureStateCatchingUpToOwner::process,
            CreatureStateInvestigatingPointOfInterestMoving::process,
            CreatureStateInvestigatingPointOfInterestInvestigating::process,
        )
            .into_configs()
    }
}

pub trait StateMachineExplorerExt {
    fn explorer_events_for<S: EntityState>(self) -> Self;
}

impl StateMachineExplorerExt for StateMachine {
    fn explorer_events_for<S: EntityState>(self) -> Self {
        self.on_enter::<S>(CreatureStateExploration::attach_is_exploring)
            .on_exit::<S>(CreatureStateExploration::deattach_is_exploring)
    }
}
