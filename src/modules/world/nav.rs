use crate::data::Rect;

use super::*;

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
    pub width: usize,
    pub height: usize,
    pub flags: Vec<NavFlag>,
    pub baked_collision: Option<Vec<Rect>>,
}

impl Nav {
    pub fn new(level: &Level) -> Self {
        let size = level.size.tiles();

        Nav {
            width: size.x as usize,
            height: size.y as usize,
            flags: vec![NavFlag::Walkable; (size.x * size.y) as usize],
            baked_collision: None,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_flag(&self, x: usize, y: usize) -> Option<NavFlag> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.flags[self.index(x, y)])
    }

    pub fn set_flag(&mut self, pos: BevyPosition, flag: NavFlag) {
        if pos.x as usize >= self.width || pos.y as usize >= self.height {
            return;
        }
        let index = self.index(pos.x as usize, pos.y as usize);
        self.flags[index] = flag;
    }

    pub fn bake_collision(&mut self) {
        if self.baked_collision.is_some() {
            return;
        }

        let w = self.width;
        let h = self.height;
        let mut visited = vec![false; w * h];
        let mut rects: Vec<Rect> = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;

                if visited[i] || !self.is_solid(x, y) {
                    continue;
                }

                // 1)  width on this row
                let mut max_w = 0usize;
                while x + max_w < w && !visited[y * w + (x + max_w)] && self.is_solid(x + max_w, y)
                {
                    max_w += 1;
                }

                let mut max_h = 1usize;
                'grow_down: loop {
                    if y + max_h >= h {
                        break;
                    }
                    for dx in 0..max_w {
                        let xx = x + dx;
                        let yy = y + max_h;
                        if visited[yy * w + xx] || !self.is_solid(xx, yy) {
                            break 'grow_down;
                        }
                    }
                    max_h += 1;
                }

                for dy in 0..max_h {
                    for dx in 0..max_w {
                        visited[(y + dy) * w + (x + dx)] = true;
                    }
                }

                rects.push(Rect {
                    x: x as i64,
                    y: y as i64,
                    width: max_w as i64,
                    height: max_h as i64,
                });
            }
        }

        self.baked_collision = Some(rects);
    }

    fn is_solid(&self, x: usize, y: usize) -> bool {
        self.is_blocked(x, y) || self.is_water(x, y)
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        matches!(self.get_flag(x, y), Some(NavFlag::Walkable))
    }

    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        matches!(self.get_flag(x, y), Some(NavFlag::Blocked))
    }

    pub fn is_water(&self, x: usize, y: usize) -> bool {
        matches!(self.get_flag(x, y), Some(NavFlag::Water))
    }
}
