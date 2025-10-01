use bevy::ecs::error::info;

use crate::components::move_intent::{self, MoveIntent};
use crate::components::movement_state::MovementState;
use crate::entities::player::Player;
use crate::modules::player::player_entered_transition::PlayerEnteredTransition;
use crate::modules::world::ldtk::*;
use crate::modules::world::map_transitions::MapTransitions;
use crate::modules::world::plugin::WorldPlugin;
use crate::prelude::*;

#[add_system(schedule = OnEnter(RpgState::MapTransition), plugin = WorldPlugin)]
fn on_map_transition(
    mut commands: Commands,
    mut player_entered_transition: EventReader<PlayerEnteredTransition>,
    mut map_changed: EventWriter<MapAssetChanged>,
    mut q_map: Query<(
        Entity,
        &LdtkMapHandle,
        &mut LdtkMapConfig,
        &mut MapTransitions,
    )>,
    mut q_player: Query<(Entity, &mut Transform, &mut MoveIntent), With<Player>>,
    mut next_state: ResMut<NextState<RpgState>>,
) {
    let Some(evt) = player_entered_transition.read().last().cloned() else {
        next_state.set(RpgState::InGame);
        return;
    };

    let Ok((entity, handle, mut cfg, mut transitions)) = q_map.single_mut() else {
        warn!("No active LdtkMap entity found when handling map transition");
        next_state.set(RpgState::InGame);
        return;
    };

    commands.entity(entity).despawn_related::<Children>();
    transitions.0.clear();

    cfg.set_selected_level_by_iid(&evt.transition.destination.map);
    map_changed.write(MapAssetChanged(handle.0.id()));

    if let Ok((player, mut transform, mut move_intent)) = q_player.single_mut() {
        let dest = evt.transition.destination.position;
        info!(
            "Transitioning to map: {} at position: ({}, {})",
            evt.transition.destination.map, dest.x, dest.y
        );

        commands.entity(player).remove::<MovementState>();
        move_intent.stop();
        transform.translation = evt
            .transition
            .destination
            .position
            .extend(transform.translation.z);
    }

    next_state.set(RpgState::InGame);
}
