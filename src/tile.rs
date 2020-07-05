use ggez::graphics::{BlendMode, DrawParam, Drawable, Image, Rect};
use ggez::{Context, GameResult};

#[derive(Debug, Clone)]
pub enum Tile {
    Wall(Option<Image>),
    Ground,
}

impl Tile {
    pub fn is_wall(&self) -> bool {
        matches!(self, Tile::Wall(_))
    }
}

impl Drawable for Tile {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        const WALL_SCALING: f32 = 1.3;

        if let Tile::Wall(Some(img)) = self {
            img.draw(
                ctx,
                param
                    .clone()
                    .scale([param.scale.x * WALL_SCALING, param.scale.y * WALL_SCALING])
                    .dest([
                        param.dest.x - (32. * WALL_SCALING - 32.) * param.scale.x / 2.,
                        param.dest.y - (32. * WALL_SCALING - 32.) * param.scale.y / 2.,
                    ]),
            )?;
        }
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Option::from(Rect::new(0., 0., 16., 16.))
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}
