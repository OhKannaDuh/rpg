use crate::{data::direction::Direction, prelude::*};

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct MoveIntent(pub Option<Direction>);

impl MoveIntent {
    pub fn is_moving(&self) -> bool {
        self.0.is_some()
    }

    pub fn stop(&mut self) {
        self.0 = None;
    }
}
