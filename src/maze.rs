use ggez::graphics::{Drawable, Rect, BlendMode, DrawParam, Color};
use ggez::{Context, GameResult, graphics};
use ggez::nalgebra as na;

pub enum Tile {
    Wall,
    Ground,
}

pub struct Maze {
    /// (width, height)
    dim: (usize, usize),
    tiles: Vec<Tile>,
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        match self {
            Tile::Wall => Tile::Wall,
            Tile::Ground => Tile::Ground,
        }
    }
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            dim: (width, height),
            tiles: vec![Tile::Ground; width * height],
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[self.dim.0 * y + x] = tile;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[self.dim.0 * y + x]
    }
}

impl Tile {
    fn drawable(&self, ctx: &mut Context) -> GameResult<impl Drawable> {
        match self {
            Tile::Ground => graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0., 0., 16., 16.), graphics::WHITE),
            Tile::Wall => graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), Rect::new(0., 0., 16., 16.), graphics::BLACK),
        }
    }
}

impl Drawable for Tile {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.drawable(ctx)?.draw(ctx, param)
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Option::from(Rect::new(0., 0., 16., 16.))
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}

impl Drawable for Maze {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mut result = Ok(());
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result = result.and(self.get_tile(x, y).draw(ctx, param.clone().dest(na::Point2::new(param.dest.x + x as f32 * 16., param.dest.y + y as f32 * 16.))));
            }
        }
        result
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Option::from(Rect::new(0., 0., self.dim.0 as f32 * 16., self.dim.1 as f32 * 16.))
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}
