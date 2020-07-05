use std::collections::HashMap;
use std::time::Instant;
use std::{env, path};

use rand;

use ggez::conf::{NumSamples, WindowSetup, WindowMode, FullscreenType};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Text, DrawParam};
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;

use ggez::{Context, ContextBuilder, GameResult};

use crate::assets::Assets;
use crate::maze::Maze;
use crate::player::{Animation, Player, PlayerState};

mod assets;
mod maze;
mod player;
mod tile;

struct MainState<'a> {
    maze: Maze,
    player: Player<'a>,
    info: Text,
    start: Instant,
    found: u8,
}

impl<'a> MainState<'a> {
    fn new(assets: &'a Assets) -> GameResult<MainState<'a>> {
        let mut maze = Maze::new((21, 21), assets);
        maze.generate(&mut rand::thread_rng(), assets);

        let mut player_animations = HashMap::new();
        player_animations.insert(
            PlayerState::Idle,
            Animation::new(assets.get_from_pattern("game/idle_*.png"), 50),
        );
        player_animations.insert(
            PlayerState::Run,
            Animation::new(assets.get_from_pattern("game/run_*.png"), 50),
        );
        player_animations.insert(
            PlayerState::Hurt,
            Animation::new(assets.get_from_pattern("game/hurt_*.png"), 50),
        );
        player_animations.insert(
            PlayerState::Dead,
            Animation::new(assets.get_from_pattern("game/death_*.png"), 50),
        );

        Ok(MainState {
            maze,
            info: Text::new("10"),
            start: Instant::now(),
            found: 0,
            player: Player::new(player_animations),
        })
    }
}

impl EventHandler for MainState<'_> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let diff = 10i32 - (Instant::now() - self.start).as_secs() as i32;
        if diff > 0 {
            self.info = Text::new(format!("{:02}", diff));
        } else {
            self.info = Text::new(format!("{}/3", self.found));
        }

        let (fx, fy) = self.player.pos;
        let (x, y) = (fx as usize, fy as usize);
        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            if y != 0 && !self.maze.get([x, y - 1].into()).is_wall() {
                self.player.translate((0.0, -1.0));
            }
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            if y != 20 && !self.maze.get([x, y + 1].into()).is_wall() {
                self.player.translate((0.0, 1.0));
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if x != 0 && !self.maze.get([x - 1, y].into()).is_wall() {
                self.player.translate((-1.0, 0.0));
            }
        }
        else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if x != 20 && !self.maze.get([x + 1, y].into()).is_wall() {
                self.player.translate((1.0, 0.0));
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(
            ctx,
            &self.maze,
            DrawParam::new().dest([0.0, 0.0]).scale([1., 1.]),
        )?;
        graphics::draw(ctx, &self.player, (na::Point2::new(0.0, 0.0),))?;

        graphics::draw(ctx, &self.info, DrawParam::new().dest(na::Point2::new(725.0, 50.0)).scale(na::Vector2::new(2.0, 2.0)))?;

        graphics::present(ctx)?;

        Ok(())
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
            icon: "/ui/arrowBrown_right.png".to_owned(),
            srgb: true,
        })
        .window_mode(WindowMode {
            width: 800.0,
            height: 675.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
        .add_resource_path(resource_dir)
        .build()?;
    let assets = &Assets::load(ctx, &path)?;
    let state = &mut MainState::new(&assets)?;
    event::run(ctx, event_loop, state)
}
