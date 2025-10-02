use crate::prelude::*;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct MoveIntent(pub Option<Direction>);

impl MoveIntent {
    pub fn is_valid(&self) -> bool {
        self.0.is_some()
    }

    pub fn stop(&mut self) {
        self.0 = None;
    }
}

#[derive(Component, Clone, Copy, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct MovementState {
    pub from: IVec2,
    pub to: IVec2,
    pub progress: f32,
    pub speed_cells_per_sec: f32,
}

impl MovementState {
    pub fn new(from: IVec2, to: IVec2, speed_cells_per_sec: f32) -> Self {
        Self {
            from,
            to,
            progress: 0.0,
            speed_cells_per_sec,
        }
    }
}
