mod assets;
mod maze;
mod player;

use crate::assets::load_assets;
use crate::maze::{Maze, Tile};
use crate::player::{Player, PlayerState, Animation};
use ggez::{GameResult, Context, ContextBuilder};
use ggez::conf::{WindowSetup, NumSamples};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Image};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use std::{path, env};
use std::collections::HashMap;
use rand;
use std::{
    env,
    path::{self, PathBuf},
};

struct MainState {
    maze: Maze,
    player: Player,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = load_assets(ctx)?;
        let mut maze = Maze::new((21, 21), &assets);
        maze.generate(&mut rand::thread_rng(), &assets);

        Ok(MainState {
            maze,
            player: Player::new(HashMap::new()),
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::draw(ctx, &self.maze, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.player, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let (fx, fy) = self.player.pos;
        let (x, y) = (fx as usize, fy as usize);

        match keycode {
            KeyCode::Up => if y != 0 && !self.maze.get([x, y - 1].into()).is_wall() {
                self.player.translate((0.0, -1.0));
            },
            KeyCode::Down => if y != 20 && !self.maze.get([x, y + 1].into()).is_wall() {
                self.player.translate((0.0, 1.0));
            },
            KeyCode::Left => if x != 0 && !self.maze.get([x - 1, y].into()).is_wall() {
                self.player.translate((-1.0, 0.0));
            },
            KeyCode::Right => if x != 20 && !self.maze.get([x + 1, y].into()).is_wall() {
                self.player.translate((1.0, 0.0));
            },
            _ => ()
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    } else {
        path::PathBuf::from("./assets")
    };

    let (ctx, event_loop) = &mut ContextBuilder::new("pate2crabe", "team_pate2crabe")
        .window_setup(WindowSetup {
            title: "pate2crabe".to_owned(),
            samples: NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .add_resource_path(resource_dir)
        .build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
