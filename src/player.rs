use std::collections::HashMap;
use std::time::Duration;

use ggez::{self, Context, GameResult};
use ggez::graphics::{BlendMode, DrawParam, Drawable, Image, Rect};
use ggez::nalgebra as na;
use ggez::timer;

pub struct Animation<'a> {
    frames: Vec<&'a Image>,
    index: usize,
    interval: Duration,
}

impl<'a> Animation<'a> {
    pub fn new(frames: Vec<&'a Image>, interval: Duration) -> Animation<'a> {
        Animation {
            frames,
            index: 0,
            interval
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.frames.len();
    }
}

impl Drawable for Animation<'_> {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.frames[self.index].draw(ctx, param)
    }

    fn dimensions(&self, _ctx: &mut Context) -> Option<Rect> {
        Some(self.frames[self.index].dimensions())
    }

    fn set_blend_mode(&mut self, _mode: Option<BlendMode>) {
        unimplemented!()
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

pub struct Player<'a> {
    pub pos: (f32, f32),
    animations: HashMap<PlayerState, Animation<'a>>,
    state: PlayerState,
    last_update_time: Duration,
}

impl<'a> Player<'a> {
    pub fn new(animations: HashMap<PlayerState, Animation>) -> Player {
        Player {
            pos: (1.0, 1.0), // start
            animations,
            state: PlayerState::Idle,
            last_update_time: Duration::from_secs(0),
        }
    }

    pub fn translate(&mut self, vec: (f32, f32)) {
        if self.state != PlayerState::Dead {
            self.pos = (self.pos.0 + vec.0, self.pos.1 + vec.1);
        }
    }
    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    fn current_animation(&self) -> &Animation {
        &self.animations[&self.state]
    }

    pub fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    pub fn next_frame(&mut self, ctx: &Context) {
        let current_time = timer::time_since_start(ctx);

        if current_time > self.last_update_time + self.current_animation().interval {
            self.animations.get_mut(&self.state).unwrap().next();
            self.last_update_time = current_time;
        }
    }
}

impl Drawable for Player<'_> {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        self.current_animation().draw(
            ctx,
            param.clone().dest(na::Point2::new(
                param.dest.x + self.pos.0 * 32.0,
                param.dest.y + self.pos.1 * 32.0,
            )),
        )
    }

    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.current_animation().dimensions(ctx)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.animations
            .get_mut(&self.state)
            .unwrap()
            .set_blend_mode(mode);
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.current_animation().blend_mode()
    }
}
