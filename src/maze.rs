use ggez::graphics::{BlendMode, Color, DrawParam, Drawable, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use na::{Point2, Vector2};
use rand::prelude::*;

pub type CellIndex = Point2<usize>;
pub type Point = Point2<f64>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Tile {
    Wall,
    Ground,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Maze {
    /// (width, height)
    dim: (usize, usize),
    tiles: Vec<Tile>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn as_relative(self) -> Vector2<isize> {
        match self {
            Self::North => [0, 1],
            Self::East => [1, 0],
            Self::South => [0, -1],
            Self::West => [-1, 0],
        }
        .into()
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl Maze {
    pub fn new((w, h): (usize, usize)) -> Self {
        Maze {
            dim: (w, h),
            tiles: vec![Tile::Wall; w * h],
        }
    }

    pub fn generate<R: Rng>(&mut self, rng: &mut R) {
        // start at (0, 0)
        self.backtrack_gen([0, 0].into(), rng)
    }

    fn backtrack_gen<R: Rng>(&mut self, curr: CellIndex, rng: &mut R) {
        let curr_as_vec: Vector2<isize> = [curr.x as isize, curr.y as isize].into();

        // mark as visited
        self.set(curr, Tile::Ground);

        if curr == [self.dim.0 - 1, self.dim.1 - 1].into() {
            // end at (self.dim.0 - 1, self.dim.1 - 1)
            return;
        }

        let mut directions = DIRECTIONS;
        directions.shuffle(rng);

        // find unvisited neighbours
        for dir in &directions {
            let index_ = curr_as_vec + dir.as_relative() * 2;
            let index: CellIndex = [index_.x as usize, index_.y as usize].into();

            // between is the cell between the current one and the neighbour
            let between_ = curr_as_vec + dir.as_relative();
            let between: CellIndex = [between_.x as usize, between_.y as usize].into();

            // if neighbour is not visited
            if self.is_in_range(index) && self.get(index) == Tile::Wall {
                self.set(between, Tile::Ground);
                self.backtrack_gen(index, rng);
            }
        }
    }

    pub fn get(&self, pos: CellIndex) -> Tile {
        assert!(self.is_in_range(pos));
        self.tiles[pos.y * self.dim.0 + pos.x]
    }

    pub fn set(&mut self, pos: CellIndex, tile: Tile) {
        assert!(self.is_in_range(pos));
        self.tiles[pos.y * self.dim.0 + pos.x] = tile;
    }

    pub fn is_in_range(&self, pos: CellIndex) -> bool {
        return (0..self.dim.0).contains(&pos.x) && (0..self.dim.1).contains(&pos.y);
    }
}

impl Tile {
    pub fn drawable(&self, ctx: &mut Context) -> GameResult<impl Drawable> {
        match self {
            Tile::Ground => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 16., 16.),
                graphics::WHITE,
            ),
            Tile::Wall => graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(0., 0., 16., 16.),
                graphics::BLACK,
            ),
        }
    }
}

impl Drawable for Tile {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.drawable(ctx)?.draw(ctx, param)
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

impl Drawable for Maze {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let mut result = Ok(());
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                result = result.and(self.get([x, y].into()).draw(
                    ctx,
                    param.clone().dest(na::Point2::new(
                        param.dest.x + x as f32 * 16.,
                        param.dest.y + y as f32 * 16.,
                    )),
                ));
            }
        }
        result
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        Option::from(Rect::new(
            0.,
            0.,
            self.dim.0 as f32 * 16.,
            self.dim.1 as f32 * 16.,
        ))
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}
