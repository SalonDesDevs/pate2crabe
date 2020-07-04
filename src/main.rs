use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, path};

use ggez::conf::{NumSamples, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Image, Text};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};
use rand;
use std::time::Instant;

use crate::assets::{load_assets, Assets};
use crate::maze::Maze;
use crate::player::{Animation, Player, PlayerState};

mod assets;
mod maze;
mod player;
mod tile;

struct MainState {
    maze: Maze,
    player: Player,
    info: Text,
    start: Instant,
    found: u8,
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context, path: PathBuf) -> GameResult<MainState> {
        let assets = load_assets(ctx, &path)?;
        let mut maze = Maze::new((21, 21), &assets);
        maze.generate(&mut rand::thread_rng(), &assets);

        let mut player_animations = HashMap::new();
        player_animations.insert(
            PlayerState::Idle,
            Animation::new(
                ctx,
                &assets,
                &[
                    "/game/idle_1.png",
                    "/game/idle_2.png",
                    "/game/idle_3.png",
                    "/game/idle_4.png",
                ],
                50,
            ),
        );
        player_animations.insert(
            PlayerState::Run,
            Animation::new(ctx, &assets, &["/game/idle_1.png"], 50),
        );
        player_animations.insert(
            PlayerState::Hurt,
            Animation::new(ctx, &assets, &["/game/idle_1.png"], 50),
        );
        player_animations.insert(
            PlayerState::Dead,
            Animation::new(ctx, &assets, &["/game/idle_1.png"], 50),
        );

        Ok(MainState {
            maze,
            info: Text::new("10"),
            start: Instant::now(),
            found: 0,
            player: Player::new(player_animations),
            assets,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let diff = (Instant::now() - self.start).as_secs();
        if diff > 0 {
            self.info = Text::new(format!("{:02}", diff));
        } else {
            self.info = Text::new(format!("{}/3", self.found));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(ctx, &self.maze, (na::Point2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &self.player, (na::Point2::new(0.0, 0.0),))?;

        graphics::draw(ctx, &self.info, (na::Point2::new(500.0, 50.0),))?;

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
            KeyCode::Up => {
                if y != 0 && !self.maze.get([x, y - 1].into()).is_wall() {
                    self.player.translate((0.0, -1.0));
                }
            }
            KeyCode::Down => {
                if y != 20 && !self.maze.get([x, y + 1].into()).is_wall() {
                    self.player.translate((0.0, 1.0));
                }
            }
            KeyCode::Left => {
                if x != 0 && !self.maze.get([x - 1, y].into()).is_wall() {
                    self.player.translate((-1.0, 0.0));
                }
            }
            KeyCode::Right => {
                if x != 20 && !self.maze.get([x + 1, y].into()).is_wall() {
                    self.player.translate((1.0, 0.0));
                }
            }
            _ => (),
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
    let path = resource_dir.clone();

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
    let state = &mut MainState::new(ctx, path)?;
    event::run(ctx, event_loop, state)
}
