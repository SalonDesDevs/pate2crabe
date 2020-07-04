use ggez::{self, GameResult, Context};
use ggez::graphics::{self, Drawable, DrawParam, Rect, BlendMode, Image};
use ggez::nalgebra as na;
use crate::assets::Assets;

pub struct Player {
    pub pos: (f32, f32),
    sprite: Image,
}

impl Player {
    pub fn new(assets: &Assets) -> Player {
        Player {
            pos: (0.0, 0.0), // start
            sprite: assets.idle_1.clone(),
        }
    }

    pub fn translate(&mut self, vec: (f32, f32)) {
        self.pos = (
            self.pos.0 + vec.0,
            self.pos.1 + vec.1
        );
    }
}

impl Drawable for Player {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.sprite.draw(ctx, param.clone().dest(
            na::Point2::new(
                param.dest.x + self.pos.0 * 16.0,
                param.dest.y + self.pos.1 * 16.0
            )
        ))
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.sprite.dimensions())
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.sprite.set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.sprite.blend_mode()
    }
}
