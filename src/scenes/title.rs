use crate::scenes::manager::{Scene, Transition};

use tetra::{Context};
use tetra::audio::{Sound, SoundInstance};
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key, GamepadButton};

pub struct TitleScene {
    title: Texture,
    background_music_instance: SoundInstance,
}

impl TitleScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        // Load game title scene
        let title = Texture::new(ctx, "./assets/art/large_title.png")?;
        // Load theme music
        let background_music = Sound::new("./assets/music/014.mp3")?;
        let background_music_instance = background_music.spawn(ctx)?;
        background_music_instance.play();
        background_music_instance.set_repeating(true);
        background_music_instance.set_volume(0.1);

        Ok(Self{
            title,
            background_music_instance,
        })
    }
}

impl Scene for TitleScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        if input::is_key_released(ctx, Key::Space) || input::is_key_released(ctx, Key::Enter) || input::is_gamepad_button_released(ctx, 0, GamepadButton::A) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Start) {
            // TODO: Implement GameScene
            Ok(Transition::Push(Box::new(GameScene::new(ctx)?)))
        } else if input::is_key_released(ctx, Key::Escape) || input::is_gamepad_button_released(ctx, 0, GamepadButton::Back) {
            Ok(Transition::Quit)
        } else {
            Ok(Transition::None)
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
        // Texture implements the Drawable trait
        graphics::draw(ctx, &self.title, DrawParams::default());

        Ok(Transition::None)
    }
}