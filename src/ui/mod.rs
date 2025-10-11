prelude!();

use bevy::ecs::relationship;

use crate::prelude::actor::creature::*;
use std::fmt::Write;

pub struct UiPlugin;
game_module_build!(UiPlugin);

impl GameModule for UiPlugin {
    fn systems(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            (setup_ui, setup_owned_creatures_panel),
        );

        app.on_playing_game_update((sync_owned_creatures_ui, update_owned_creatures_ui));
    }
}

fn setup_ui(mut commands: Commands) {
    info!("Setting up UI...");
    commands.spawn((Node {
        width: percent(100),
        height: percent(100),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },));
}

#[derive(Component)]
struct OwnedCreaturesUiRoot;

#[derive(Component)]
struct OwnedCreaturesUiEntry(pub Entity);

fn setup_owned_creatures_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(6),
            width: px(320),
            ..default()
        },
        BackgroundColor(tailwind::GRAY_700.into()),
        OwnedCreaturesUiRoot,
    ));
}

fn sync_owned_creatures_ui(
    mut commands: Commands,
    root_query: Single<Entity, With<OwnedCreaturesUiRoot>>,
    creatures_q: Query<(Entity, &Name), (With<Creature>, (With<OwnedBy>, Added<Creature>))>,
) {
    let root = root_query.into_inner();

    if creatures_q.is_empty() {
        return;
    }

    let mut rows: Vec<(Entity, String)> = Vec::new();

    for (entity, name) in creatures_q.iter() {
        rows.push((entity, name.into()));
    }

    commands.entity(root).despawn_children();
    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Node {
                margin: UiRect::bottom(px(4)),
                padding: UiRect::axes(px(8), px(8)),
                ..default()
            },
            BackgroundColor(tailwind::GRAY_900.into()),
            children![(
                Node { ..default() },
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Text::new("Creatures"),
            )],
        ));

        let is_empty = rows.is_empty();

        // Entries
        for (entity, label) in rows {
            parent.spawn((
                Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    column_gap: px(6),
                    padding: UiRect::axes(px(8), px(8)),
                    ..default()
                },
                OwnedCreaturesUiEntry(entity),
                children![(
                    Node { ..default() },
                    Text::new(label),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgba(0.95, 0.95, 1.0, 0.95)),
                )],
            ));
        }

        if is_empty {
            parent.spawn((
                Node { ..default() },
                Text::new("No creatures"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 1.0, 1.0, 0.6)),
            ));
        }
    });
}

fn agent_state_to_str(state: &AgentState) -> &'static str {
    match state {
        AgentState::Idle => "Idle",
        AgentState::ReachedTarget => "Reached Target",
        AgentState::ReachedAnimationLink => "Reached Animation Link",
        AgentState::UsingAnimationLink => "Using Animation Link",
        AgentState::Moving => "Moving",
        AgentState::AgentNotOnNavMesh => "Agent Not On Nav Mesh",
        AgentState::TargetNotOnNavMesh => "Target Not On Nav Mesh",
        AgentState::NoPath => "No Path",
        AgentState::Paused => "Paused",
    }
}

fn update_owned_creatures_ui(
    creature_query: Query<
        (
            Entity,
            &Name,
            &Transform,
            &AgentState,
            &AgentTarget2d,
            &CreatureStats,
            &CreatureNeeds,
            &CreatureRelationships,
        ),
        (With<Creature>, With<OwnedBy>),
    >,
    creature_state_query: Query<
        (
            Option<&CreatureStateIdle>,
            // Exploration states
            Option<&CreatureStateRoamingAroundOwnerIdle>,
            Option<&CreatureStateRoamingAroundOwnerRoaming>,
            Option<&CreatureStateCatchingUpToOwner>,
            Option<&CreatureStateInvestigatingPointOfInterestMoving>,
            Option<&CreatureStateInvestigatingPointOfInterestInvestigating>,
        ),
        With<Creature>,
    >,
    entry_query: Query<&OwnedCreaturesUiEntry>,
    mut text_query: Query<(&ChildOf, &mut Text)>,
    transforms: Query<&Transform>,
) {
    for (child, mut text) in text_query.iter_mut() {
        let Ok(entry) = entry_query.get(child.parent()) else {
            continue;
        };

        let Ok((creature, name, transform, agent_state, agent_target, stats, needs, relationships)) =
            creature_query.get(entry.0)
        else {
            continue;
        };

        let mut data: String = name.into();
        data.write_str("\n").ok();

        data.write_fmt(format_args!("ID: {:?}\n", creature)).ok();

        data.write_fmt(format_args!(
            "Position: {:.2}, {:.2}\n",
            transform.translation.x, transform.translation.y
        ))
        .ok();

        data.write_fmt(format_args!(
            "Agent State: {}\n",
            agent_state_to_str(agent_state)
        ))
        .ok();

        data.write_fmt(format_args!(
            "Stats:\n Health({}/{})\n Energy({}/{})\n",
            stats.health.current, stats.health.max, stats.energy.current, stats.energy.max
        ))
        .ok();

        data.write_fmt(format_args!(
            "Needs:\n Exploration({:.2}/{:.2} - {:.1})\n Social({:.2}/{:.2} - {:.1}%)\n",
            needs.exploration.current,
            needs.exploration.max,
            needs.exploration.percentage() * 100.0,
            needs.social.current,
            needs.social.max,
            needs.social.percentage() * 100.0
        ))
        .ok();

        for (other, relationship) in &relationships.relationships {
            data.write_fmt(format_args!(
                "Relationship with({}) {}\n",
                other, relationship.friendship
            ))
            .ok();
        }

        match agent_target {
            AgentTarget2d::Point(target_position) => {
                data.write_fmt(format_args!(
                    "Agent Target: Point({:.2}, {:.2})\n",
                    target_position.x, target_position.y
                ))
                .ok();
            }
            AgentTarget2d::Entity(target_entity) => {
                data.write_fmt(format_args!("Agent Target: Entity({})\n", target_entity))
                    .ok();

                if let Ok(target_transform) = transforms.get(*target_entity) {
                    data.write_fmt(format_args!(
                        "  Point({:.2}, {:.2})\n",
                        target_transform.translation.x, target_transform.translation.y
                    ))
                    .ok();
                }
            }
            _ => {}
        }

        if let Ok((
            idle,
            roaming_around_owner_idle,
            roaming_around_owner_roaming,
            catching_up_to_owner,
            investigating_point_of_interest_moving,
            investigating_point_of_interest_investigating,
        )) = creature_state_query.get(creature)
        {
            if idle.is_some() {
                data.write_str("State: Idle\n").ok();
            }

            if let Some(state) = roaming_around_owner_idle {
                data.write_str("State: Roaming Around Owner (Idle)\n").ok();
                data.write_str(format!(" Time Idle: {:.2}\n", state.time_in_state).as_str())
                    .ok();

                if let Some(target_time) = state.target_time {
                    data.write_str(format!(" Target Idle Time: {:.2}\n", target_time).as_str())
                        .ok();
                }
            }

            if roaming_around_owner_roaming.is_some() {
                data.write_str("State: Roaming Around Owner (Roaming)\n")
                    .ok();
            }

            if catching_up_to_owner.is_some() {
                data.write_str("State: Catching Up To Owner\n").ok();
            }

            if investigating_point_of_interest_moving.is_some() {
                data.write_str("State: Investigating Point Of Interest (Moving)\n")
                    .ok();
            }

            if investigating_point_of_interest_investigating.is_some() {
                data.write_str("State: Investigating Point Of Interest (Investigating)\n")
                    .ok();
            }
        }

        text.0 = data;
    }
}
