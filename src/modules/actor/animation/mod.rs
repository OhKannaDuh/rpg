prelude!();

use std::hash::Hash;

pub struct AnimationPlugin;
game_module_build!(AnimationPlugin);

impl GameModule for AnimationPlugin {
    fn systems(&self, app: &mut App) {
        app.on_playing_game_update((animate_actors,));
    }
}

fn animate_actors(time: Res<Time>, mut query: Query<(&mut ActorAnimator, &mut Sprite)>) {
    let delta = time.delta_secs();

    for (mut animator, mut sprite) in query.iter_mut() {
        let next = animator.controller.get_current_state();

        animator.elapsed += delta;

        if next != animator.current {
            animator.reset_current();
            animator.current = next;

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 0;
            }
        }

        let current_key = animator.current.clone();
        let (frame_duration, maybe_len) = match animator.animation.get(&current_key) {
            Some(anim) => {
                if anim.frames.is_empty() {
                    continue;
                }
                let frame = &anim.frames[anim.frame];
                (frame.duration, Some(anim.frames.len()))
            }
            None => continue,
        };

        if animator.elapsed >= frame_duration {
            animator.elapsed -= frame_duration;

            if let Some(anim) = animator.animation.get_mut(&current_key) {
                let len = match maybe_len {
                    Some(x) => x,
                    None => anim.frames.len(),
                };
                if len == 0 {
                    continue;
                }

                anim.frame = (anim.frame + 1) % len;

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = anim.frames[anim.frame].atlas_index;
                }
            }
        }
    }
}

pub trait ActorAnimationController<Key>: Send + Sync
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    fn get_current_state(&self) -> Key;
}

pub struct StaticAnimationController {
    state: String,
}

impl StaticAnimationController {
    pub fn new(state: String) -> Self {
        Self { state }
    }
}

impl ActorAnimationController<String> for StaticAnimationController {
    fn get_current_state(&self) -> String {
        self.state.clone()
    }
}

#[derive(Component)]
pub struct ActorAnimator<Key = String>
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub current: Key,
    pub elapsed: f32,
    pub animation: HashMap<Key, ActorAnimation>,
    pub controller: Box<dyn ActorAnimationController<Key>>,
}

impl ActorAnimator<String> {
    pub fn static_animation(animation: ActorAnimation) -> Self {
        Self {
            current: "default".to_string(),
            elapsed: 0.0,
            animation: {
                let mut map = HashMap::new();
                map.insert("default".to_string(), animation);
                map
            },
            controller: Box::new(StaticAnimationController::new("default".to_string())),
        }
    }

    pub fn reset_current(&mut self) {
        if let Some(anim) = self.animation.get_mut(&self.current) {
            anim.reset();
        }

        self.elapsed = 0.0;
    }
}

pub struct ActorAnimation {
    pub frame: usize,
    pub frames: Vec<AnimationFrameMeta>,
}

impl ActorAnimation {
    pub fn new(frames: Vec<AnimationFrameMeta>) -> Self {
        Self { frame: 0, frames }
    }

    pub fn from_many_frames(atlas_index_start: usize, count: usize, duration: f32) -> Self {
        Self {
            frame: 0,
            frames: AnimationFrameMeta::many(atlas_index_start, count, duration),
        }
    }

    pub fn reset(&mut self) {
        self.frame = 0;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Reflect, Default)]
pub struct AnimationFrameMeta {
    pub atlas_index: usize,
    pub duration: f32,
}

impl AnimationFrameMeta {
    pub fn new(atlas_index: usize, duration: f32) -> Self {
        Self {
            atlas_index,
            duration,
        }
    }

    pub fn many(atlas_index_start: usize, count: usize, duration: f32) -> Vec<Self> {
        (0..count)
            .map(|i| AnimationFrameMeta {
                atlas_index: atlas_index_start + i,
                duration,
            })
            .collect()
    }
}
