mod maze;
mod player;

use crate::maze::{Maze, Tile};
use crate::player::Player;
use ggez::{GameResult, Context, ContextBuilder};
use ggez::conf::{WindowSetup, NumSamples};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Image};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use rand;

struct MainState {
    maze: Maze,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut maze = Maze::new((21, 21));
        maze.generate(&mut rand::thread_rng());

        Ok(MainState {
            maze,
            player: Player::new(Image::from_rgba8(
                ctx,
                16, 16,
                &[125; 16 * 16 * 4]
            )?),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.maze.set([5, 7].into(), Tile::Wall);
        graphics::draw(ctx, &self.maze, (na::Point2::new(100.0, 100.0),))?;
        graphics::draw(ctx, &self.player, (na::Point2::new(100.0, 100.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool
    ) {
        let (fx, fy) = self.player.pos;
        let (x, y) = (fx as usize, fy as usize);

        match keycode {
            KeyCode::Up => if y != 0 && self.maze.get([x, y - 1].into()) == Tile::Ground {
                self.player.translate((0.0, -1.0));
            },
            KeyCode::Down => if y != 20 && self.maze.get([x, y + 1].into()) == Tile::Ground {
                self.player.translate((0.0, 1.0));
            },
            KeyCode::Left => if x != 0 && self.maze.get([x - 1, y].into()) == Tile::Ground {
                self.player.translate((-1.0, 0.0));
            },
            KeyCode::Right => if x != 20 && self.maze.get([x + 1, y].into()) == Tile::Ground {
                self.player.translate((1.0, 0.0));
            },
            _ => ()
        }
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("pate2crabe", "team_pate2crabe")
        .window_setup(WindowSetup {
            title: "pate2crabe".to_owned(),
            samples: NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
