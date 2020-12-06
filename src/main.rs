mod assets;
mod scenes;
mod utils;

use crate::scenes::manager::SceneManager;
use crate::scenes::title::TitleScene;
use crate::utils::GameInformation;

use tetra::{ContextBuilder};


#[macro_use]
extern crate lazy_static;

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
    .resizable(false) // Remove later
    .quit_on_escape(false)
    .build()?
    .run(|ctx| {
        let title_scene = TitleScene::new(ctx)?;
        Ok(SceneManager::new(Box::new(title_scene)))
    })
}
