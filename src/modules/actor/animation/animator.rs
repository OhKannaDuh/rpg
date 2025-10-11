prelude!();

use bevy::app::Animation;

use crate::modules::actor::animation::*;

#[derive(Component)]
pub struct AnimationBank<Key>
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub clips: HashMap<Key, ActorAnimation>,
}

impl<Key> AnimationBank<Key>
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    // pub fn entry_from_animation(key: Key, animation: ActorAnimation) -> AnimationBank<Key> {

    // }

    pub fn idle_entry(key: Key, atlas_index: usize) -> (Key, ActorAnimation) {
        (
            key,
            ActorAnimation::new(vec![AnimationFrameMeta::new(atlas_index, 0.0)]),
        )
    }

    pub fn linear_entry(
        key: Key,
        start: usize,
        length: usize,
        frame_duration: f32,
    ) -> (Key, ActorAnimation) {
        (
            key,
            ActorAnimation::new(
                (start..start + length)
                    .map(|i| AnimationFrameMeta::new(i, frame_duration))
                    .collect(),
            ),
        )
    }
}

#[derive(Component)]
pub struct Animator<Key>
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub current: Key,
    pub target: Key,
    pub elapsed: f32,
    pub speed: f32,
    pub looping: bool,
}

impl<Key> Animator<Key>
where
    Key: Eq + Hash + Clone + Send + Sync + 'static,
{
    pub fn new(initial: Key) -> Self {
        Self {
            current: initial.clone(),
            target: initial,
            elapsed: 0.0,
            speed: 1.0,
            looping: true,
        }
    }

    pub fn reset_current(&mut self, bank: &mut AnimationBank<Key>) {
        if let Some(clip) = bank.clips.get_mut(&self.current) {
            clip.reset();
        }
        self.elapsed = 0.0;
    }
}

impl<E> Default for Animator<E>
where
    E: Default + Clone + Eq + Hash + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new(E::default())
    }
}
