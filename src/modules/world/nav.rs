use super::*;
use crate::data::Rect;
use bevy::math::I64Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavFlag {
    Walkable,
    Blocked,
    Water,
}

impl NavFlag {
    pub fn from_id(id: i64) -> Option<Self> {
        match id {
            3 => Some(NavFlag::Water),
            4 | 5 => Some(NavFlag::Blocked),
            _ => None,
        }
    }
}

#[derive(Component)]
pub struct Nav {
    pub width: i64,
    pub height: i64,
    pub grid_size: i64,
    pub origin_world_cell: I64Vec2,
    pub flags: Vec<NavFlag>,
    pub baked_collision: Option<Vec<Rect>>,
}

impl Nav {
    pub fn new(chunk: &Chunk) -> Self {
        let size = chunk.size.tiles();
        let g = chunk.grid.grid_size;

        let bl_px = chunk.transform.bottom_left_bevy();

        fn div_floor_i64(a: i64, b: i64) -> i64 {
            let (q, r) = (a / b, a % b);
            if (r != 0) && ((r > 0) != (b > 0)) {
                q - 1
            } else {
                q
            }
        }
        let origin_world_cell = I64Vec2::new(div_floor_i64(bl_px.x, g), div_floor_i64(bl_px.y, g));

        Self {
            width: size.x,
            height: size.y,
            grid_size: g,
            origin_world_cell,
            flags: vec![NavFlag::Walkable; (size.x * size.y) as usize],
            baked_collision: None,
        }
    }

    fn index(&self, x: i64, y: i64) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            Some((y as usize) * (self.width as usize) + (x as usize))
        }
    }

    pub fn world_to_local(&self, world: I64Vec2) -> Option<I64Vec2> {
        let l = world - self.origin_world_cell;
        if l.x < 0 || l.y < 0 || l.x >= self.width || l.y >= self.height {
            None
        } else {
            Some(l)
        }
    }

    pub fn set_flag(&mut self, x: i64, y: i64, flag: NavFlag) {
        if let Some(i) = self.index(x, y) {
            self.flags[i] = flag;
        }
    }

    pub fn get_flag_local(&self, x: i64, y: i64) -> Option<NavFlag> {
        self.index(x, y).map(|i| self.flags[i])
    }

    pub fn get_flag_world(&self, world: I64Vec2) -> Option<NavFlag> {
        self.world_to_local(world)
            .and_then(|l| self.get_flag_local(l.x, l.y))
    }

    pub fn is_blocked_world(&self, world: I64Vec2) -> bool {
        matches!(self.get_flag_world(world), Some(NavFlag::Blocked))
    }

    pub fn is_water_world(&self, world: I64Vec2) -> bool {
        matches!(self.get_flag_world(world), Some(NavFlag::Water))
    }

    pub fn is_oob_world(&self, world: I64Vec2) -> bool {
        self.world_to_local(world).is_none()
    }

    pub fn is_solid_world(&self, world: I64Vec2) -> bool {
        match self.world_to_local(world) {
            Some(l) => self.is_solid_local(l.x, l.y),
            None => false,
        }
    }

    pub fn is_blocked_local(&self, x: i64, y: i64) -> bool {
        matches!(self.get_flag_local(x, y), Some(NavFlag::Blocked))
    }

    pub fn is_water_local(&self, x: i64, y: i64) -> bool {
        matches!(self.get_flag_local(x, y), Some(NavFlag::Water))
    }

    pub fn is_solid_local(&self, x: i64, y: i64) -> bool {
        self.is_blocked_local(x, y) || self.is_water_local(x, y)
    }

    pub fn bake_collision(&mut self) {
        if self.baked_collision.is_some() {
            return;
        }

        let w = self.width as usize;
        let h = self.height as usize;
        let mut visited = vec![false; w * h];
        let mut rects: Vec<Rect> = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if visited[i] || !self.is_solid_local(x as i64, y as i64) {
                    continue;
                }

                let mut max_w = 0usize;
                while x + max_w < w
                    && !visited[y * w + (x + max_w)]
                    && self.is_solid_local((x + max_w) as i64, y as i64)
                {
                    max_w += 1;
                }
                let mut max_h = 1usize;
                'grow: loop {
                    if y + max_h >= h {
                        break;
                    }
                    for dx in 0..max_w {
                        let xx = (x + dx) as i64;
                        let yy = (y + max_h) as i64;
                        if visited[(y + max_h) * w + (x + dx)] || !self.is_solid_local(xx, yy) {
                            break 'grow;
                        }
                    }
                    max_h += 1;
                }
                for dy in 0..max_h {
                    for dx in 0..max_w {
                        visited[(y + dy) * w + (x + dx)] = true;
                    }
                }

                let pos_world_px = I64Vec2::new(
                    (x as i64 + self.origin_world_cell.x) * self.grid_size,
                    (y as i64 + self.origin_world_cell.y) * self.grid_size,
                );
                let size = Size::new(max_w as i64, max_h as i64, self.grid_size);
                rects.push(Rect {
                    position: pos_world_px,
                    size,
                });
            }
        }

        self.baked_collision = Some(rects);
    }
}
