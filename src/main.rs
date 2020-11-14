mod scenes;

use crate::scenes::title::TitleScene;

use tetra::{ContextBuilder};


#[macro_use]
extern crate lazy_static;

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
