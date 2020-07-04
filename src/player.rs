use ggez::{self, GameResult, Context};
use ggez::graphics::{self, Drawable, DrawParam, Rect, BlendMode, Image};
use ggez::nalgebra as na;

pub struct Player {
    pos: (f32, f32),
    sprite: Image,
}

impl Player {
    pub fn new(sprite: Image) -> Player {
        Player {
            pos: (0.0, 0.0), // start
            sprite,
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
                param.dest.x + self.pos.0,
                param.dest.y + self.pos.1
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
