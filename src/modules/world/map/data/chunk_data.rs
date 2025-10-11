prelude!();
module!(resources);

use super::layers::*;
use crate::modules::world::map::TileFlags;
use bevy::math::I64Vec2;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
pub struct ChunkId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
pub struct WorldId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
pub struct ChunkPosition {
    pub chunk_x: i64,
    pub chunk_y: i64,
    pub pixel_x: i64,
    pub pixel_y: i64,
}

impl ChunkPosition {
    pub fn from_level(level: &Level) -> Self {
        // Flip from LDTK to Bevy coordinate system
        let pixel_y = -(level.world_y) - level.px_hei;

        Self {
            chunk_x: level.world_x.div_euclid(PIXELS_PER_CHUNK_I64),
            chunk_y: pixel_y.div_euclid(PIXELS_PER_CHUNK_I64),
            pixel_x: level.world_x,
            pixel_y,
        }
    }

    pub fn get_chunk_coords(&self) -> I64Vec2 {
        I64Vec2::new(self.chunk_x, self.chunk_y)
    }

    pub fn get_pixel_coords(&self) -> I64Vec2 {
        I64Vec2::new(self.pixel_x, self.pixel_y)
    }

    pub fn get_tile_position(&self) -> TilePosition {
        let world_x = self.pixel_x.div_euclid(TILE_SIZE_I64);
        let world_y = self.pixel_y.div_euclid(TILE_SIZE_I64);

        let local_x = world_x.rem_euclid(TILES_PER_CHUNK_I64);
        let local_y = world_y.rem_euclid(TILES_PER_CHUNK_I64);

        TilePosition {
            local_x,
            local_y,
            world_x,
            world_y,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, PartialOrd, Ord)]
pub struct TilePosition {
    pub local_x: i64,
    pub local_y: i64,
    pub world_x: i64,
    pub world_y: i64,
}

#[derive(Clone, Debug)]
pub struct ChunkData {
    pub id: ChunkId,
    pub world_id: WorldId,

    pub position: ChunkPosition,

    pub name: String,

    pub tile_layers: Vec<TileLayer>,
    pub entity_layers: Vec<EntityLayer>,
    pub data_layers: Vec<DataLayer>,

    pub flags: Vec<TileFlags>,

    pub neighbors: Vec<ChunkId>,
}

impl ChunkData {
    pub fn from_level(
        world_id: &WorldId,
        level: &Level,
        tileset_defs: &TilesetDefs,
        entity_defs: &EntityDefs,
    ) -> Self {
        let chunk_id = ChunkId(level.iid.clone());

        let tile_layers: Vec<TileLayer> = level
            .layer_instances
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|layer| TileLayer::is(layer))
            .map(TileLayer::from_instance)
            .collect();

        let entity_layers: Vec<EntityLayer> = level
            .layer_instances
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|layer| EntityLayer::is(layer))
            .map(|l| EntityLayer::from_instance(l, world_id, &chunk_id, entity_defs))
            .collect();

        let data_layers: Vec<DataLayer> = level
            .layer_instances
            .as_ref()
            .unwrap_or(&Vec::new())
            .iter()
            .filter(|layer| DataLayer::is(layer))
            .map(DataLayer::from_instance)
            .collect();

        let neighbors: Vec<ChunkId> = level
            .neighbours
            .iter()
            .map(|n| ChunkId(n.level_iid.clone()))
            .collect();

        let w = TILES_PER_CHUNK as usize;
        let h = TILES_PER_CHUNK as usize;
        let len = w * h;
        let mut flags: Vec<TileFlags> = vec![TileFlags::empty(); len];

        for tile_layer in &tile_layers {
            let Some(tileset_def) = tileset_defs.0.get(&tile_layer.tileset_def_uid) else {
                continue;
            };

            for tile in &tile_layer.tiles {
                let x = tile.px[0] / TILE_SIZE_I64;
                let mut y = tile.px[1] / TILE_SIZE_I64;

                y = TILES_PER_CHUNK_I64 - 1 - y;

                if x < 0 || y < 0 || x >= TILES_PER_CHUNK_I64 || y >= TILES_PER_CHUNK_I64 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;
                let index = y * w + x;

                flags[index] |= tileset_def.flags(tile.t);
            }
        }

        Self {
            id: chunk_id,
            world_id: world_id.clone(),
            position: ChunkPosition::from_level(level),

            name: level.identifier.clone(),

            tile_layers,
            entity_layers,
            data_layers,

            flags,

            neighbors,
        }
    }

    pub fn get_collider(&self) -> Option<Collider> {
        let w = TILES_PER_CHUNK as usize;
        let h = TILES_PER_CHUNK as usize;
        let g = TILE_SIZE;

        let at = |x: usize, y: usize| -> TileFlags { self.flags[y * w + x] };

        let mut shapes: Vec<(Vec2, f32, Collider)> = Vec::new();
        let mut visited = vec![false; self.flags.len()];

        let origin_world_tile = self.position.get_tile_position();

        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if visited[i] {
                    continue;
                }
                let f = at(x, y);
                if !f.is_blocked() {
                    continue;
                }

                let mut x2 = x;
                while x2 < w && !visited[y * w + x2] && at(x2, y).is_blocked() {
                    x2 += 1;
                }

                let mut y2 = y + 1;
                'grow_down: while y2 < h {
                    for xx in x..x2 {
                        let idx = y2 * w + xx;
                        if visited[idx] || !at(xx, y2).is_blocked() {
                            break 'grow_down;
                        }
                    }
                    y2 += 1;
                }

                for yy in y..y2 {
                    for xx in x..x2 {
                        visited[yy * w + xx] = true;
                    }
                }

                let tiles_w = (x2 - x) as f32;
                let tiles_h = (y2 - y) as f32;
                let half_w = tiles_w * g * 0.5;
                let half_h = tiles_h * g * 0.5;

                let left_px = (origin_world_tile.world_x + x as i64) as f32 * g;
                let bottom_px = (origin_world_tile.world_y + y as i64) as f32 * g;

                let center = Vec2::new(left_px + tiles_w * g * 0.5, bottom_px + tiles_h * g * 0.5);

                shapes.push((center, 0.0, Collider::cuboid(half_w, half_h)));
            }
        }

        const EDGE_FRACTION: f32 = 0.2;
        let band_half = g * EDGE_FRACTION * 0.5;

        for y in 0..h {
            let mut x = 0;
            while x < w {
                while x < w {
                    let f = at(x, y);
                    if f.contains(TileFlags::COLLIDE_NORTH) && !f.is_blocked() {
                        break;
                    }
                    x += 1;
                }
                if x >= w {
                    break;
                }

                let start = x;
                x += 1;
                while x < w {
                    let f = at(x, y);
                    if f.contains(TileFlags::COLLIDE_NORTH) && !f.is_blocked() {
                        x += 1;
                    } else {
                        break;
                    }
                }
                let end = x;

                let tiles_w = (end - start) as f32;
                let half_w = tiles_w * g * 0.5;

                let left_px = (origin_world_tile.world_x + start as i64) as f32 * g;
                let top_edge_px = (origin_world_tile.world_y + (y as i64 + 1)) as f32 * g;

                let center = Vec2::new(left_px + tiles_w * g * 0.5, top_edge_px - band_half);

                shapes.push((center, 0.0, Collider::cuboid(half_w, band_half)));
            }
        }

        if shapes.is_empty() {
            None
        } else {
            Some(Collider::compound(shapes))
        }
    }

    pub fn get_nav_mesh(&self) -> NavigationMesh2d {
        let w = TILES_PER_CHUNK_I64;
        let h = TILES_PER_CHUNK_I64;
        let g = TILE_SIZE;

        let origin = self.position.get_tile_position();

        let mut vmap: HashMap<(i64, i64), u32> = HashMap::new();
        let mut vertices: Vec<Vec2> = Vec::new();
        let mut polygons: Vec<Vec<usize>> = Vec::new();
        let mut poly_types: Vec<usize> = Vec::new();

        let mut corner = |tx: i64, ty: i64| -> u32 {
            let px = (tx as f32) * g;
            let py = (ty as f32) * g;

            *vmap.entry((tx, ty)).or_insert_with(|| {
                let idx = vertices.len() as u32;
                vertices.push(Vec2::new(px, py));
                idx
            })
        };

        let at = |x: i64, y: i64| -> TileFlags {
            let xi = x as usize;
            let yi = y as usize;
            self.flags[yi * (w as usize) + xi]
        };

        for y in 0..h {
            for x in 0..w {
                let flags = at(x, y);
                if flags.is_blocked() {
                    continue;
                }

                let wx = origin.world_x + x;
                let wy = origin.world_y + y;

                let v0 = corner(wx, wy);
                let v1 = corner(wx + 1, wy);
                let v2 = corner(wx + 1, wy + 1);
                let v3 = corner(wx, wy + 1);

                polygons.push(vec![v0 as usize, v1 as usize, v2 as usize, v3 as usize]);
                poly_types.push(0);
            }
        }

        NavigationMesh2d {
            vertices,
            polygons,
            polygon_type_indices: poly_types,
            height_mesh: None,
        }
    }
}
