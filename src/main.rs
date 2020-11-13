use tetra::{Context, ContextBuilder, State};
use tetra::audio::{Sound, SoundInstance};
use tetra::graphics::{self, Color, DrawParams, Texture};


#[macro_use]
extern crate lazy_static;

struct TitleScene {
    title: Texture,
    background_music_instance: SoundInstance,
}

impl TitleScene {
    fn new(ctx: &mut Context) -> tetra::Result<Self> {
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

impl State for TitleScene {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
        // Texture implements the Drawable trait
        graphics::draw(ctx, &self.title, DrawParams::default());

        Ok(())
    }
}

struct Window {
    width: u32,
    height: u32,
}

struct GameInformation {
    window: Window,
    version: String,
}

impl GameInformation {
    fn new() -> Self {
        let version = env!("CARGO_PKG_VERSION").to_owned();
        GameInformation {
            window: Window {
                width: 480,
                height: 920,
            },
            version,
        }
    }
}

// Initialize GameInformation at runtime on first access in main
lazy_static! {
    static ref GAMEINFO: GameInformation = GameInformation::new();
}

fn main() -> tetra::Result {
    ContextBuilder::new(
        format!("Shooting Clone v{}", GAMEINFO.version).as_str(),
        GAMEINFO.window.width as i32,
        GAMEINFO.window.height as i32,
    )
    .quit_on_escape(false)
    .build()?
    .run(|ctx| {
        let title_scene = TitleScene::new(ctx)?;
        Ok(title_scene)
    })
}
