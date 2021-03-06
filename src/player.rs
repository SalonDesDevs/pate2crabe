use std::collections::HashMap;
use std::time::Duration;

use ggez::{self, Context, GameResult};
use ggez::audio::{SoundSource, Source};
use ggez::graphics::{BlendMode, Drawable, DrawParam, Image, Rect};
use ggez::nalgebra as na;
use ggez::nalgebra::{Vector2, Point2};
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
            interval,
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
    last_animation_update_time: Duration,
    last_movement_update_time: Duration,
    current_translation: Option<(f32, f32, f32, f32)>,
    step_count: usize,
    running_sound: Source,
    flipped: bool,
}

impl<'a> Player<'a> {
    pub fn new(animations: HashMap<PlayerState, Animation>, mut running_sound: Source) -> Player {
        running_sound.set_volume(0.3);
        Player {
            pos: (1.0, 1.0), // start
            animations,
            state: PlayerState::Idle,
            last_animation_update_time: Duration::from_secs(0),
            last_movement_update_time: Duration::from_secs(0),
            current_translation: None,
            step_count: 0,
            running_sound,
            flipped: false,
        }
    }

    pub fn translate(&mut self, vec: (f32, f32)) {
        if self.state != PlayerState::Dead && self.current_translation.is_none() {
            self.current_translation = Some((vec.0, vec.1, self.pos.0 + vec.0, self.pos.1 + vec.1));
            self.state = PlayerState::Run;
            if self.step_count % 2 == 0 {
                self.running_sound.play();
            }
            self.step_count += 1;
        }
    }

    pub fn set_flipped(&mut self, flipped: bool) {
        self.flipped = flipped
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

    pub fn update(&mut self, ctx: &Context) {
        let current_time = timer::time_since_start(ctx);

        // Animations
        if current_time > self.last_animation_update_time + self.current_animation().interval {
            self.animations.get_mut(&self.state).unwrap().next();
            self.last_animation_update_time = current_time;
        }

        // Movement
        if current_time > self.last_movement_update_time + Duration::from_millis(20)
            && self.state != PlayerState::Dead
            && self.state != PlayerState::Hurt
        {
            if let Some(translation) = self.current_translation {
                if translation.2 <= self.pos.0 + 0.1
                    && translation.2 >= self.pos.0 - 0.1
                    && translation.3 <= self.pos.1 + 0.1
                    && translation.3 >= self.pos.1 - 0.1
                {
                    self.pos = (translation.2, translation.3);
                    self.state = PlayerState::Idle;
                    self.current_translation = None;
                } else {
                    self.pos = (
                        self.pos.0 + translation.0 * 0.1,
                        self.pos.1 + translation.1 * 0.1,
                    );
                }
                self.last_movement_update_time = current_time;
            }
        }
    }
}

impl Drawable for Player<'_> {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult {
        let current_animation = self.current_animation();
        current_animation.draw(
            ctx,
            param.clone().offset(Point2::new(0.5, 0.)).scale(Vector2::new(if self.flipped { -1. } else { 1. }, 1.)).dest(na::Point2::new(
                param.dest.x + self.pos.0 * 32.0 + 16.,
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
