use tetra::Context;
use tetra::audio::Sound;
use tetra::graphics::animation::Animation;
use tetra::graphics::text::Text;
use tetra::graphics::{Rectangle, Texture};

use core::time::Duration;
use std::collections::HashMap;

pub struct Assets {
    // sounds: HashMap<SoundName, Sound>,
    animations: HashMap<AnimationName, Animation>,
    // textures: HashMap<TextureName, Texture>,
    // texts: Vec<Text>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Assets{
            // sounds: build_sounds(),
            animations: build_animations(ctx)?,
            // textures: build_textures(),
            // texts: build_texts(),
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        for animation in self.animations.values_mut() {
            animation.advance(ctx);
        }
    }

    pub fn get_animation(&self, name: &AnimationName) -> &Animation {
        &self.animations[name]
    }
}

fn build_animations(ctx: &mut Context) -> tetra::Result<HashMap<AnimationName, Animation>> {
    let titleSet = Texture::new(ctx, "./assets/art/shootOutYourLife.png")?;
    let line = Texture::new(ctx, "./assets/art/line.png")?;

    let animations: HashMap<AnimationName, Animation> = [
        (AnimationName::Player, Animation::new(titleSet.clone(), Rectangle::row(0.0, 0.0, 32.0, 32.0).take(4).collect(), Duration::new(10, 0))),
        (AnimationName::Enemy1, Animation::new(titleSet.clone(), Rectangle::row(128.0, 0.0, 32.0, 32.0).take(4).collect(), Duration::new(10, 0))),
        (AnimationName::Enemy2, Animation::new(titleSet.clone(), Rectangle::row(128.0, 32.0, 32.0, 32.0).take(4).collect(), Duration::new(10, 0))),
        (AnimationName::Enemy3, Animation::new(titleSet.clone(), Rectangle::row(0.0, 32.0, 32.0, 32.0).take(4).collect(), Duration::new(10, 0))),
        (AnimationName::Line, Animation::new(titleSet.clone(), Rectangle::row(40.0, 0.0, 5.0, 412.0).take(4).collect(), Duration::new(10, 0))),
    ].iter().cloned().collect();

    Ok(animations)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SoundName {
    ShootSlow,
    ShootFast,
    Hurt,
    Hurt2,
    Pickup,
    NewLevel,
    Charge,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnimationName {
    Player,
    Enemy1,
    Enemy2,
    Enemy3,
    Line,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureName {
    Life,
    Level,
    Bullet1Up,
    Bullet1Down,
    Bullet2Up,
    Bullet2Down,
    Bullet3Up,
    Bullet3Down,
    Background,
    Particle0,
    Particle1,
    Enemy1a,
    Enemy1b,
    Enemy1c,
    Enemy1d,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextName {
    Life,
    Level,
    Score,
    GameOver,
    ScoreGui,
    Pause,
}
