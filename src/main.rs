use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::{env, path};

use ggez::audio::{SoundData, SoundSource, Source};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{
    self, Color, DrawMode, DrawParam, FillOptions, Image, MeshBuilder, Rect, Text,
};
use ggez::input::keyboard::{self, KeyCode};
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};
use rand;

use crate::assets::Assets;
use crate::maze::Maze;
use crate::player::{Animation, Player, PlayerState};

mod assets;
mod maze;
mod player;
mod tile;
mod rewards;

struct MainState<'a> {
    maze: Maze,
    player: Player<'a>,
    info: Text,
    start: Instant,
    found: u8,
    images: &'a Assets<Image>,
    sounds: &'a Assets<SoundData>,
    hidden: bool,
}

impl<'a> MainState<'a> {
    fn new(ctx: &mut Context, images: &'a Assets<Image>, sounds: &'a Assets<SoundData>) -> GameResult<MainState<'a>> {
        let mut maze = Maze::new((21, 21), images);
        maze.generate(&mut rand::thread_rng(), &images);

        let mut player_animations = HashMap::new();
        player_animations.insert(
            PlayerState::Idle,
            Animation::new(images.get_from_pattern("game/idle_*.png"), Duration::from_millis(50)),
        );
        player_animations.insert(
            PlayerState::Run,
            Animation::new(images.get_from_pattern("game/run_*.png"), Duration::from_millis(50)),
        );
        player_animations.insert(
            PlayerState::Hurt,
            Animation::new(images.get_from_pattern("game/hurt_*.png"), Duration::from_millis(50)),
        );
        player_animations.insert(
            PlayerState::Dead,
            Animation::new(images.get_from_pattern("game/death_*.png"), Duration::from_millis(50)),
        );

        let mut source =
            Source::from_data(ctx, sounds["/audio/game/audio_loop.ogg"].clone()).unwrap();
        source.set_repeat(true);
        source.play_detached()?;

        Ok(MainState {
            maze,
            info: Text::new("10"),
            start: Instant::now(),
            found: 0,
            player: Player::new(player_animations),
            images,
            sounds,
            hidden: false,
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
            self.hidden = true;
        }

        if !self.hidden {
            return Ok(());
        }

        let (fx, fy) = self.player.pos;
        let (x, y) = (fx as usize, fy as usize);

        for cx in 0..20 {
            for cy in 0..20 {
                if let Some(reward) = self.maze.get_mut_reward([cx, cy].into()) {
                    if self.hidden {
                        reward.texture = self.images["/game/pan_empty.png"].clone();
                    }

                    if cx != x || cy != y || reward.found {
                        continue;
                    }

                    reward.found = true;

                    if reward.malus {
                        self.player.set_state(PlayerState::Dead);
                        // println!("Perdu UwU");
                        // std::process::exit(0);
                    } else {
                        self.found += 1;
                    }
                }
            }
        }

        if self.found == 3 && x == 10 && y == 19 {
            println!("Gagné OwO");
            std::process::exit(0);
        }

        if !self.player.is_dead() {
            if keyboard::is_key_pressed(ctx, KeyCode::Up) {
                if y != 0 && !self.maze.get([x, y - 1].into()).is_wall() {
                    self.player.translate((0.0, -1.0));
                }
            } else if keyboard::is_key_pressed(ctx, KeyCode::Down) {
                if y != 20 && !self.maze.get([x, y + 1].into()).is_wall() {
                    self.player.translate((0.0, 1.0));
                }
            }
            if keyboard::is_key_pressed(ctx, KeyCode::Left) {
                if x != 0 && !self.maze.get([x - 1, y].into()).is_wall() {
                    self.player.translate((-1.0, 0.0));
                }
            } else if keyboard::is_key_pressed(ctx, KeyCode::Right) {
                if x != 20 && !self.maze.get([x + 1, y].into()).is_wall() {
                    self.player.translate((1.0, 0.0));
                }
            }
        }
        self.player.next_frame(ctx);

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

        graphics::draw(
            ctx,
            &self.images["/ui/panel_brown.png"],
            DrawParam::new()
                .dest(na::Point2::new(690.0, 40.0))
                .scale(na::Vector2::new(0.9, 0.5)),
        )?;
        graphics::draw(
            ctx,
            &self.images["/ui/panelInset_beige.png"],
            DrawParam::new()
                .dest(na::Point2::new(697.5, 47.5))
                .scale(na::Vector2::new(0.8, 0.38)),
        )?;
        graphics::draw(
            ctx,
            &self.info,
            DrawParam::new()
                .dest(na::Point2::new(715.0, 50.0))
                .scale(na::Vector2::new(2.0, 2.0)),
        )?;

        if self.hidden {
            let (x, y) = self.player.pos;

            let black = Color::from_rgb(0, 0, 0);
            let mut mesh = MeshBuilder::new();

            mesh.rectangle(
                DrawMode::Fill(FillOptions::DEFAULT),
                Rect {
                    x: 0.0,
                    y: 0.0,
                    w: x * 32.0 - 15.0,
                    h: 675.0,
                },
                black.clone(),
            );
            mesh.rectangle(
                DrawMode::Fill(FillOptions::DEFAULT),
                Rect {
                    x: x * 32.0 - 15.0,
                    y: 0.0,
                    w: 62.5,
                    h: y * 32.0 - 15.0,
                },
                black.clone(),
            );
            mesh.rectangle(
                DrawMode::Fill(FillOptions::DEFAULT),
                Rect {
                    x: x * 32.0 - 15.0,
                    y: y * 32.0 - 10.0 + 50.0,
                    w: 62.5,
                    h: 675.0 - y * 32.0 - 15.0 + 55.0,
                },
                black.clone(),
            );
            mesh.rectangle(
                DrawMode::Fill(FillOptions::DEFAULT),
                Rect {
                    x: x * 32.0 - 15.0 + 62.5,
                    y: 0.0,
                    w: 580.0 - x * 32.0 - 15.0 + 62.5,
                    h: 675.0,
                },
                black.clone(),
            );

            let mesh = &mesh.build(ctx)?;
            graphics::draw(ctx, mesh, (na::Point2::new(0.0, 0.0),))?;
        }

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
    let images = &Assets::load(&path, &["png"], |path| Image::new(ctx, path))?;
    let sounds = &Assets::load(&path, &["ogg", "wav"], |path| SoundData::new(ctx, path))?;
    let state = &mut MainState::new(ctx, images, sounds)?;
    event::run(ctx, event_loop, state)
}
