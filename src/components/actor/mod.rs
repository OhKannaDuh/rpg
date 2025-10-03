use crate::prelude::*;
use bevy::math::I64Vec2;

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct FaceDirection(pub Direction);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct GridPosition {
    pub world_cell: Option<I64Vec2>,
    pub local_cell: Option<I64Vec2>,
    pub level_iid: Option<String>,
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
#[require(GridPosition, FaceDirection)]
pub struct GridMover {
    pub grid_size: i64,
    pub seconds_per_cell: f32,
    pub intent: Option<Direction>,
}

#[derive(Component, Debug)]
#[component(storage = "SparseSet")]
pub struct GridMoveState {
    pub from: I64Vec2,
    pub to: I64Vec2,
    pub t: f32,
}
