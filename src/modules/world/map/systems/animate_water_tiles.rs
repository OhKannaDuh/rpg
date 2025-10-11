prelude!();

use crate::modules::world::map::*;

pub fn animate_water_tiles(
    mut query: Query<(&mut TileTextureIndex, &WaterTile)>,
    mut animation: ResMut<GlobalWaterAnimation>,
    time: Res<Time>,
) {
    let current_frame = animation.frame;
    animation.update(time.delta_secs());

    if current_frame == animation.frame {
        return;
    }

    for (mut texture_index, water_tile) in query.iter_mut() {
        let base = water_tile.base_texture_index as i64;
        let next = base + (animation.frame as i64 * water_tile.stride);
        let current = texture_index.0 as i64;

        if current == next {
            continue;
        }

        info!("Updating water tile from {} to {}", current, next);

        texture_index.0 = next as u32;
    }
}
