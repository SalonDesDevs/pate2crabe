use ggez::graphics::{BlendMode, Color, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra as na;
use ggez::{graphics, Context, GameResult};
use na::{Point2, Vector2};
use rand::prelude::*;

use crate::assets::Assets;

pub type CellIndex = Point2<usize>;
pub type Point = Point2<f64>;

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

#[derive(Debug, Clone)]
pub struct Maze {
    /// (width, height)
    dim: (usize, usize),
    tiles: Vec<Tile>,
    grass_asset: Image,
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
    pub fn new((w, h): (usize, usize), assets: &Assets) -> Self {
        Maze {
            dim: (w, h),
            tiles: vec![Tile::Wall(None); w * h],
            grass_asset: assets.grass.clone(),
        }
    }

    pub fn generate<R: Rng>(&mut self, rng: &mut R, assets: &Assets) {
        // start at (0, 0)
        self.backtrack_gen([0, 0].into(), rng);
        self.set_textures(assets);
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
            if self.is_in_range(index) && self.get(index).is_wall() {
                self.set(between, Tile::Ground);
                self.backtrack_gen(index, rng);
            }
        }
    }

    pub fn get(&self, pos: CellIndex) -> &Tile {
        assert!(self.is_in_range(pos));
        &self.tiles[pos.y * self.dim.0 + pos.x]
    }

    pub fn set(&mut self, pos: CellIndex, tile: Tile) {
        assert!(self.is_in_range(pos));
        self.tiles[pos.y * self.dim.0 + pos.x] = tile;
    }

    pub fn is_in_range(&self, pos: CellIndex) -> bool {
        (0..self.dim.0).contains(&pos.x) && (0..self.dim.1).contains(&pos.y)
    }

    fn set_textures(&mut self, assets: &Assets) {
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                let index = CellIndex::from([x, y]);

                let texture = match (
                    matches!(
                        self.get_tile_rel(index, Direction::North),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::East),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::South),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::West),
                        Some(Tile::Wall(_))
                    ),
                ) {
                    (true, true, false, false) => Some(assets.wall_corn_bot_lft.clone()),
                    (true, false, false, true) => Some(assets.wall_corn_bot_rgt.clone()),
                    (false, true, true, false) => Some(assets.wall_corn_top_lft.clone()),
                    (false, false, true, true) => Some(assets.wall_corn_top_rgt.clone()),

                    (true, true, true, true) => Some(assets.wall_crss_all.clone()),
                    (true, true, false, true) => Some(assets.wall_crss_hori_bot.clone()),
                    (false, true, true, true) => Some(assets.wall_crss_hori_top.clone()),
                    (true, true, true, false) => Some(assets.wall_crss_vert_lft.clone()),
                    (true, false, true, true) => Some(assets.wall_crss_vert_rgt.clone()),

                    (false, false, false, true) => Some(assets.wall_hori_lft.clone()),
                    (false, true, false, true) => Some(assets.wall_hori_mid.clone()),
                    (false, true, false, false) => Some(assets.wall_hori_rgt.clone()),

                    (true, false, false, false) => Some(assets.wall_vert_bot.clone()),
                    (true, false, true, false) => Some(assets.wall_vert_mid.clone()),
                    (false, false, true, false) => Some(assets.wall_vert_top.clone()),

                    (false, false, false, false) => None,
                };

                self.set(index, Tile::Wall(texture));
            }
        }
    }

    fn get_tile_rel(&self, pos: CellIndex, dir: Direction) -> Option<&Tile> {
        let rel = dir.as_relative();
        let pos2 = CellIndex::from([
            (pos.x as isize + rel.x) as usize,
            (pos.y as isize + rel.y) as usize,
        ]);

        match self.is_in_range(pos2) {
            false => None,
            true => Some(self.get(pos2)),
        }
    }
}

impl Drawable for Tile {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        if let Tile::Wall(Some(img)) = self {
            img.draw(ctx, param)?;
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

impl Drawable for Maze {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                let param2 = param.clone().dest(Point2::new(
                    param.dest.x + x as f32 * 16.,
                    param.dest.y + y as f32 * 16.,
                ));
                self.grass_asset.draw(ctx, param2)?;
                self.get([x, y].into()).draw(ctx, param2)?;
            }
        }
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
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
