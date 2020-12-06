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
    textures: HashMap<TextureName, Texture>,
    // texts: Vec<Text>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Assets{
            // sounds: build_sounds(),
            animations: build_animations(ctx)?,
            textures: build_textures(ctx)?,
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

    pub fn get_texture(&self, name: &TextureName) -> &Texture {
        &self.textures[name]
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
        // (AnimationName::Line, Animation::new(titleSet.clone(), Rectangle::row(40.0, 0.0, 5.0, 412.0).take(4).collect(), Duration::new(10, 0))),
    ].iter().cloned().collect();

    Ok(animations)
}

fn build_textures(ctx: &mut Context) -> tetra::Result<HashMap<TextureName, Texture>> {
    let textures: HashMap<TextureName, Texture> = [
		(TextureName::Life, Texture::new(ctx, "./assets/art/life_art.png")?),
		(TextureName::Level, Texture::new(ctx, "./assets/art/level_art.png")?),
		(TextureName::Bullet1Down, Texture::new(ctx, "./assets/art/shot_0.png")?),
		(TextureName::Bullet1Up, Texture::new(ctx, "./assets/art/shot_1.png")?),
		(TextureName::Bullet2Down, Texture::new(ctx, "./assets/art/shot_2.png")?),
		(TextureName::Bullet2Up, Texture::new(ctx, "./assets/art/shot_3.png")?),
		(TextureName::Bullet3Down, Texture::new(ctx, "./assets/art/shot_4.png")?),
		(TextureName::Bullet3Up, Texture::new(ctx, "./assets/art/shot_5.png")?),
		(TextureName::Background, Texture::new(ctx, "./assets/art/background.png")?),
		(TextureName::Particle0, Texture::new(ctx, "./assets/art/particle_04.png")?),
		(TextureName::Particle1, Texture::new(ctx, "./assets/art/particle_05.png")?),
		(TextureName::Enemy1a, Texture::new(ctx, "./assets/art/particle_06.png")?),
		(TextureName::Enemy1b, Texture::new(ctx, "./assets/art/particle_07.png")?),
		(TextureName::Enemy1c, Texture::new(ctx, "./assets/art/particle_08.png")?),
		(TextureName::Enemy1d, Texture::new(ctx, "./assets/art/particle_09.png")?),
	].iter().cloned().collect();
	Ok(textures)
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
    // Line,
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
