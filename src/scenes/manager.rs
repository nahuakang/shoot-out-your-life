use tetra::window;
use tetra::{Context, State};

/// Scene is a trait that all scenes such as TitleScene and GameScene will implement
pub trait Scene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition>;
}

/// Transition is wrapped in tetra::Result for managing scene transitions
pub enum Transition {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Quit,
}

/// SceneManager implements State and manages the transition of different scenes.
pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
}

impl SceneManager {
    /// Returns a new instance of SceneManager.
    pub fn new(initial_scene: Box<dyn Scene>) -> Self {
        SceneManager {
            scenes: vec![initial_scene],
        }
    }
}

impl State for SceneManager {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.update(ctx)? {
                Transition::None => {}
                Transition::Push(scene) => {
                    self.scenes.push(scene);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
                Transition::Quit => {
                    window::quit(ctx)
                }
            },
            None => window::quit(ctx)
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match self.scenes.last_mut() {
            Some(active_scene) => match active_scene.draw(ctx)? {
                Transition::None => {}
                Transition::Push(scene) => {
                    self.scenes.push(scene);
                }
                Transition::Pop => {
                    self.scenes.pop();
                }
                Transition::Quit => {
                    window::quit(ctx)
                }
            },

            None => window::quit(ctx)
        }
        
        Ok(())
    }
}
