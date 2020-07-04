use std::collections::HashMap;

use ggez::{self, Context, GameResult};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};
use ggez::nalgebra as na;

use crate::assets::Assets;

pub struct Animation {
    frames: Vec<Image>,
    index: usize,
    delay: usize,
}

impl Animation {
    pub fn new(ctx: &mut Context, assets: &Assets, sprite_paths: &[&str], delay: usize) -> Animation {
        Animation {
            frames: sprite_paths.iter().map(|p| assets[*p].clone()).collect(),
            index: 0,
            delay,
        }
    }
}

impl Drawable for Animation {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.frames[self.index].draw(ctx, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.frames[self.index].dimensions())
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.frames[self.index].set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.frames[self.index].blend_mode()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerState {
    Idle,
    Run,
    Hurt,
    Dead,
}

pub struct Player {
    pub pos: (f32, f32),
    animations: HashMap<PlayerState, Animation>,
    state: PlayerState,
}

impl Player {
    pub fn new(animations: HashMap<PlayerState, Animation>) -> Player {
        Player {
            pos: (1.0, 1.0), // start
            animations,
            state: PlayerState::Idle,
        }
    }

    pub fn translate(&mut self, vec: (f32, f32)) {
        if self.state != PlayerState::Dead {
            self.pos = (
                self.pos.0 + vec.0,
                self.pos.1 + vec.1
            );
        }
    }
}

impl Drawable for Player {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.animations[&self.state].draw(ctx, param.clone().dest(
            na::Point2::new(
                param.dest.x + self.pos.0 * 32.0,
                param.dest.y + self.pos.1 * 32.0,
            )
        ))
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.animations[&self.state].dimensions(ctx)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.animations.get_mut(&self.state).unwrap().set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.animations[&self.state].blend_mode()
    }
}
