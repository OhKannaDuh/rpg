use crate::prelude::*;

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Fader {
    pub rate: f32,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
#[require(Sprite, Fader)]
pub struct FadeIn;

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
#[require(Sprite, Fader)]
pub struct FadeOut;
