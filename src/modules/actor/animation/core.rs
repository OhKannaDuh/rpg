prelude!();

#[derive(Clone, Debug)]
pub struct ActorAnimation {
    pub frame: usize,
    pub frames: Vec<AnimationFrameMeta>,
}

impl ActorAnimation {
    pub fn new(frames: Vec<AnimationFrameMeta>) -> Self {
        Self { frame: 0, frames }
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
}
