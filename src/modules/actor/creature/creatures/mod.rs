prelude!();
public!(slime);

use crate::modules::actor::{FaceDirection, animation::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreatureAnimationState {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Component, Default)]
#[require(FaceDirection, Velocity2d)]
pub struct CreatureAnimationController;

pub fn update_creature_animation_controllers(
    mut query: Query<
        (
            &FaceDirection,
            &Velocity2d,
            &mut Animator<CreatureAnimationState>,
        ),
        With<CreatureAnimationController>,
    >,
) {
    const MOVE_EPS2: f32 = 0.01;

    for (face_direction, velocity, mut animator) in query.iter_mut() {
        let moving = velocity.velocity.length_squared() >= MOVE_EPS2;

        animator.target = match (moving, face_direction.0) {
            (false, Direction::Up) => CreatureAnimationState::IdleUp,
            (false, Direction::Down) => CreatureAnimationState::IdleDown,
            (false, Direction::Left) => CreatureAnimationState::IdleLeft,
            (false, Direction::Right) => CreatureAnimationState::IdleRight,
            (true, Direction::Up) => CreatureAnimationState::MoveUp,
            (true, Direction::Down) => CreatureAnimationState::MoveDown,
            (true, Direction::Left) => CreatureAnimationState::MoveLeft,
            (true, Direction::Right) => CreatureAnimationState::MoveRight,
        };
    }
}
