use crate::{data::config::TILE_SIZE, modules::world::plugin::WorldPlugin, prelude::*};
use bevy::math::{Isometry2d, UVec2, Vec2};

const TILES_VISIBLE_EACH_DIR: u32 = 32; // total grid = 64x64 cells
const MAJOR_EVERY: u32 = 8;

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn draw_tile_grid(mut gizmos: Gizmos, q_cam: Query<&GlobalTransform, With<Camera>>) {
    let center = q_cam
        .single()
        .map(|t| t.translation())
        .unwrap_or(Vec3::ZERO);

    let snapped_center = Vec2::new(
        (center.x / TILE_SIZE).round() * TILE_SIZE,
        (center.y / TILE_SIZE).round() * TILE_SIZE,
    );

    let cell_count = UVec2::splat(TILES_VISIBLE_EACH_DIR * 2);

    gizmos
        .grid_2d(
            Isometry2d::from_translation(snapped_center),
            cell_count,
            Vec2::splat(TILE_SIZE),
            Color::srgba(1.0, 1.0, 1.0, 0.06),
        )
        .outer_edges();

    gizmos
        .grid_2d(
            Isometry2d::from_translation(snapped_center),
            // same world coverage, but larger spacing:
            cell_count / MAJOR_EVERY.max(1),
            Vec2::splat(TILE_SIZE * MAJOR_EVERY as f32),
            Color::srgba(1.0, 1.0, 1.0, 0.16),
        )
        .outer_edges();
}
