use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

#[macro_use]
extern crate lazy_static;

struct GameState;

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

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
    .resizable(false)
    .maximized(false)
    .fullscreen(false)
    .quit_on_escape(false)
    .build()?
    .run(|_| Ok(GameState))
}
