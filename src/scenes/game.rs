use crate::assets::{AnimationName, Assets, TextureName};
use crate::scenes::manager::{Scene, Transition};
use crate::{GAMEINFO};

use tetra::{Context};
use tetra::graphics::{self, Color, DrawParams};
use tetra::math::Vec2;

pub struct GameScene {
    player_position: DrawParams,
    assets: Assets,
}

impl GameScene {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self{
            player_position: DrawParams::new().position(
                Vec2::new(
                    (GAMEINFO.window.width / 2) as f32,
                    (GAMEINFO.window.height - 56) as f32,
                )
            ),
            assets: Assets::new(ctx)?,
        })
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        Ok(Transition::None)
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result<Transition> {
        graphics::clear(ctx, Color::rgb(0.122, 0.055, 0.11));
		graphics::draw(
            ctx,self.assets.get_texture(&TextureName::Background),
            DrawParams::default(),
        );
		// graphics::draw(
        //     ctx,self.assets.get_animation(&AnimationName::Line),
        //     DrawParams::new().position(Vec2::new(
        //         self.player_position.position[0] - 3.0,
        //         self.player_position.position[1] - 379.0,
        //     )),
        // );
        Ok(Transition::None)
    }
}
