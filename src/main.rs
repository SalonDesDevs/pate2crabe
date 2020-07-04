mod maze;

use crate::maze::{Maze, Tile};
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use rand;

struct MainState {
    maze: Maze,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let mut maze = Maze::new((11, 11));
        maze.generate(&mut rand::thread_rng());

        let s = MainState { maze };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.maze.set([5, 7].into(), Tile::Wall);
        graphics::draw(ctx, &self.maze, (na::Point2::new(100.0, 100.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
