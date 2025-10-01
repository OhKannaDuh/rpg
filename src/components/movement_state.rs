use crate::prelude::*;

#[derive(Component, Clone, Copy, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct MovementState {
    pub from: Vec2,
    pub to: Vec2,
    pub progress: f32,
    pub speed_cells_per_sec: f32,
}

impl MovementState {
    pub fn new(from: Vec2, to: Vec2, speed_cells_per_sec: f32) -> Self {
        Self {
            from,
            to,
            progress: 0.0,
            speed_cells_per_sec,
        }
    }
}
