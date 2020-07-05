use crate::assets::Assets;
use ggez::graphics::{BlendMode, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub struct Reward {
    texture: Image,
    malus: bool,
    pos: Point2<usize>,
}

impl Reward {
    pub fn new(assets: &Assets<Image>, pos: Point2<usize>, malus: bool) -> Reward {
        Reward {
            malus,
            texture: if malus {
                assets["/game/pan_death.png"].clone()
            } else {
                assets["/game/pan_apple.png"].clone()
            },
            pos,
        }
    }

    pub fn pos(&self) -> &Point2<usize> {
        &self.pos
    }
}

impl Drawable for Reward {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.texture.draw(
            ctx,
            param,
        )
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Rect::new(0., 0., 32., 32.).into()
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}
