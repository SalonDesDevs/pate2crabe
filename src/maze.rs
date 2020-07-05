use crate::rewards::Reward;
use ggez::graphics::{BlendMode, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use na::{Point2, Vector2};
use rand::prelude::*;

use crate::assets::Assets;
use crate::tile::Tile;

pub type CellIndex = Point2<usize>;

#[derive(Debug, Clone)]
pub struct Maze {
    /// (width, height)
    dim: (usize, usize),
    tiles: Vec<Tile>,
    grass_asset: Image,
    rewards: Vec<Reward>,
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
            Self::North => [0, -1],
            Self::East => [1, 0],
            Self::South => [0, 1],
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
    pub fn new((w, h): (usize, usize), images: &Assets<Image>) -> Self {
        Maze {
            dim: (w, h),
            tiles: vec![Tile::Wall(None); w * h],
            grass_asset: images["/game/grass.png"].clone(),
            rewards: vec![],
        }
    }

    pub fn generate<R: Rng>(&mut self, rng: &mut R, images: &Assets<Image>) {
        // generate 3 rewards and 3 maluses
        for i in 0..6 {
            loop {
                let pos: CellIndex = [
                    rng.gen_range(0, self.dim.0 / 2) * 2 + 1,
                    rng.gen_range(0, self.dim.1 / 2) * 2 + 1,
                ]
                .into();

                if pos != CellIndex::from([1, 1]) && self.get_reward(pos).is_none() {
                    self.rewards.push(Reward::new(images, pos, i > 2));
                    break;
                }
            }
        }

        self.set([self.dim.0 - 1, self.dim.1 - 2].into(), Tile::Ground);

        // start at (1, 1)
        self.backtrack_gen([1, 1].into(), rng);
        self.set_textures(images);
    }

    pub fn get_reward(&self, pos: CellIndex) -> Option<&Reward> {
        self.rewards.iter().find(|r| r.pos() == &pos)
    }

    pub fn get_mut_reward(&mut self, pos: CellIndex) -> Option<&mut Reward> {
        self.rewards.iter_mut().find(|r| r.pos() == &pos)
    }

    fn backtrack_gen<R: Rng>(&mut self, curr: CellIndex, rng: &mut R) {
        let curr_as_vec: Vector2<isize> = [curr.x as isize, curr.y as isize].into();

        // mark as visited
        self.set(curr, Tile::Ground);

        if curr == [self.dim.0 - 2, self.dim.1 - 2].into() {
            // end at (self.dim.0 - 2, self.dim.1 - 2)
            return;
        }

        if self.get_reward(curr).is_some() {
            // end at rewards
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
            if (1..self.dim.0 - 1).contains(&index.x)
                && (1..self.dim.1 - 1).contains(&index.y)
                && self.get(index).is_wall()
                && (self.get_reward(index).is_none() || dir != &Direction::South)
            // do not go down if reward there
            {
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

    fn set_textures(&mut self, images: &Assets<Image>) {
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                let index = CellIndex::from([x, y]);

                if !self.get(index).is_wall() {
                    continue;
                }

                let texture = match (
                    matches!(
                        self.get_tile_rel(index, Direction::South),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::East),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::North),
                        Some(Tile::Wall(_))
                    ),
                    matches!(
                        self.get_tile_rel(index, Direction::West),
                        Some(Tile::Wall(_))
                    ),
                ) {
                    (true, true, false, false) => {
                        Some(images["/game/wall_corn_top_lft.png"].clone())
                    }
                    (true, false, false, true) => {
                        Some(images["/game/wall_corn_top_rgt.png"].clone())
                    }
                    (false, true, true, false) => {
                        Some(images["/game/wall_corn_bot_lft.png"].clone())
                    }
                    (false, false, true, true) => {
                        Some(images["/game/wall_corn_bot_rgt.png"].clone())
                    }

                    (true, true, true, true) => Some(images["/game/wall_crss_all.png"].clone()),
                    (true, true, false, true) => {
                        Some(images["/game/wall_crss_hori_top.png"].clone())
                    }
                    (false, true, true, true) => {
                        Some(images["/game/wall_crss_hori_bot.png"].clone())
                    }
                    (true, true, true, false) => {
                        Some(images["/game/wall_crss_vert_lft.png"].clone())
                    }
                    (true, false, true, true) => {
                        Some(images["/game/wall_crss_vert_rgt.png"].clone())
                    }

                    (false, false, false, true) => Some(images["/game/wall_hori_rgt.png"].clone()),
                    (false, true, false, true) => Some(images["/game/wall_hori_mid.png"].clone()),
                    (false, true, false, false) => Some(images["/game/wall_hori_lft.png"].clone()),

                    (true, false, false, false) => Some(images["/game/wall_vert_top.png"].clone()),
                    (true, false, true, false) => Some(images["/game/wall_vert_mid.png"].clone()),
                    (false, false, true, false) => Some(images["/game/wall_vert_bot.png"].clone()),

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

impl Drawable for Maze {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        for x in 0..self.dim.0 {
            for y in 0..self.dim.1 {
                let param2 = param.clone().dest(Point2::new(
                    param.dest.x + x as f32 * 32. * param.scale.x,
                    param.dest.y + y as f32 * 32. * param.scale.y,
                ));

                // debug
                // graphics::Mesh::new_rectangle(
                //     ctx,
                //     graphics::DrawMode::stroke(1.0),
                //     Rect::new(0., 0., 32., 32.),
                //     graphics::WHITE,
                // )?
                // .draw(ctx, param2)?;

                self.grass_asset.draw(ctx, param2)?;
                self.get([x, y].into()).draw(ctx, param2)?;
            }
        }
        for r in &self.rewards {
            if r.found {
                continue;
            }

            r.draw(
                ctx,
                param.clone().dest([
                    param.dest.x + r.pos().x as f32 * 32. * param.scale.x,
                    param.dest.y + (r.pos().y as f32 * 32. - 24.) * param.scale.y,
                ]),
            )?;
        }
        Ok(())
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Option::from(Rect::new(
            0.,
            0.,
            self.dim.0 as f32 * 32.,
            self.dim.1 as f32 * 32.,
        ))
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {
        unimplemented!()
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        unimplemented!()
    }
}
