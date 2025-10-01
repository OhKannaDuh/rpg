use crate::{
    data::{
        config::*,
        world::{LevelTransition, TileSize},
    },
    modules::world::{RpgWorld, WorldPlugin, nav_grid::NavGrid},
    prelude::*,
};
use ldtk_rust::Level;

#[derive(Event, Clone, Copy)]
#[add_event(plugin = WorldPlugin)]
pub struct LevelChanged;

#[derive(Debug)]
pub struct RpgLevel {
    pub level: Level,
    pub size: TileSize,
    pub origin: Vec2,
    pub nav: NavGrid,
    pub level_transitions: Vec<LevelTransition>,
}

impl RpgLevel {
    pub fn new(world: &RpgWorld, level: Level) -> Self {
        let width = level.px_wid as u32;
        let height = level.px_hei as u32;
        let size = TileSize::from_pixels(width, height);
        let origin = Vec2::new(-(width as f32) * 0.5, -(height as f32) * 0.5);
        let nav = generate_nav(size, origin, &level);
        let level_transitions = generate_level_transitions(world, &level);

        RpgLevel {
            level,
            size,
            origin,
            nav,
            level_transitions,
        }
    }

    pub fn tile_origin(&self, t: IVec2) -> Vec2 {
        self.origin + Vec2::new(t.x as f32, t.y as f32) * TILE_SIZE
    }

    pub fn tile_center(&self, t: IVec2) -> Vec2 {
        self.tile_origin(t) + Vec2::splat(TILE_SIZE * 0.5)
    }

    pub fn world_to_tile(&self, w: Vec2) -> IVec2 {
        let p = (w - self.origin) / TILE_SIZE;
        IVec2::new(p.x.floor() as i32, p.y.floor() as i32)
    }
}

fn generate_nav(size: TileSize, origin: Vec2, level: &Level) -> NavGrid {
    let layer = match level.layer_instances.as_ref().and_then(|layers| {
        layers
            .iter()
            .find(|l| l.identifier == LAYER_COLLISION_ID && l.layer_instance_type == "IntGrid")
    }) {
        Some(l) => l,
        None => return NavGrid::default(),
    };

    let tiles = size.tiles();
    let (w, h) = (tiles.x, tiles.y);

    if (w as usize) * (h as usize) != layer.int_grid_csv.len() {
        return NavGrid::default();
    }

    let mut solid = vec![false; (w * h) as usize];

    for (i, &val) in layer.int_grid_csv.iter().enumerate() {
        let x = (i as u32) % w;
        let y_top = (i as u32) / w;
        let y = (h - 1) - y_top;
        let idx = (y * w + x) as usize;
        solid[idx] = val != 0;
        // solid[idx] = false;
    }

    NavGrid::new(UVec2::new(w, h), solid, origin)
}

fn generate_level_transitions(world: &RpgWorld, level: &Level) -> Vec<LevelTransition> {
    let mut transitions = Vec::new();

    for layer in level.layer_instances.as_ref().unwrap().iter().rev() {
        if layer.layer_instance_type != "Entities" {
            continue;
        }

        for entity in layer.entity_instances.iter() {
            info!("Found entity {:?}", entity.identifier);

            if entity.identifier == ENTITY_MAP_TRANSITION_ID {
                let Some(transition) = LevelTransition::new(entity, world, level) else {
                    warn!("Failed to create level transition from entity {:?}", entity);
                    continue;
                };

                info!(
                    "Created level transition at {:?}",
                    transition.position.pixels()
                );

                transitions.push(transition);
            }
        }
    }

    transitions
}

#[add_system(schedule = Update, plugin = WorldPlugin, run_if = in_state(RpgState::InGame))]
fn render_map_transitions(mut gizmos: Gizmos, world_query: Query<&RpgWorld>) {
    let Ok(world) = world_query.single() else {
        warn!("No RpgWorld entity found");
        return;
    };

    let Some(rpg_level) = &world.active_level else {
        warn!("No active level found");
        return;
    };

    for transition in &rpg_level.level_transitions {
        let size_px = transition.size.pixels().as_vec2();

        let origin_px = transition.position.pixels().as_vec2();
        let center = origin_px + size_px * 0.5;

        let iso = Isometry2d::from_translation(center);

        gizmos.rect_2d(iso, size_px, Color::srgb(1.0, 0.5, 0.0));
    }
}
