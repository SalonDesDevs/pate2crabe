use ggez::{self, GameResult, Context};
use ggez::graphics::{Drawable, DrawParam, Rect, BlendMode, Image};

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
}

impl Drawable for Player {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.sprite.draw(ctx, param)
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
