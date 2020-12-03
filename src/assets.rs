use tetra::audio::Sound;
use tetra::graphics::animation::Animation;
use tetra::graphics::text::Text;
use tetra::graphics::{Texture};

use std::collections::HashMap;

pub struct Assets {
    sounds: HashMap<SoundName, Sound>,
    animations: HashMap<AnimationName, Animation>,
    textures: HashMap<TextureName, Texture>,
    texts: Vec<Text>,
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
