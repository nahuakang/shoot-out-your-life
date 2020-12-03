use crate::scenes::manager::{Scene, Transition};

use tetra::{Context};

pub struct GameScene {

}

impl GameScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self{})
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }
}