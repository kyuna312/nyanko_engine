use std::time::{Instant, Duration};
use glam::Vec2;

pub struct Time {
    pub delta_time: f32,
    pub fixed_delta_time: f32,
    pub total_time: f32,
    last_update: Instant,
    fixed_time_step: Duration,
    accumulated_time: Duration,
}

impl Time {
    pub fn new(fixed_time_step: f32) -> Self {
        Self {
            delta_time: 0.0,
            fixed_delta_time: fixed_time_step,
            total_time: 0.0,
            last_update: Instant::now(),
            fixed_time_step: Duration::from_secs_f32(fixed_time_step),
            accumulated_time: Duration::ZERO,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = now - self.last_update;
        self.last_update = now;
        self.delta_time = frame_time.as_secs_f32();
        self.total_time += self.delta_time;
        self.accumulated_time += frame_time;
    }

    pub fn should_run_fixed_update(&mut self) -> bool {
        if self.accumulated_time >= self.fixed_time_step {
            self.accumulated_time -= self.fixed_time_step;
            true
        } else {
            false
        }
    }
}

pub struct Input {
    pub mouse_position: Vec2,
    pub mouse_delta: Vec2,
    keys_down: Vec<bool>,
    keys_pressed: Vec<bool>,
    keys_released: Vec<bool>,
    mouse_buttons_down: Vec<bool>,
    mouse_buttons_pressed: Vec<bool>,
    mouse_buttons_released: Vec<bool>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            keys_down: vec![false; 512],
            keys_pressed: vec![false; 512],
            keys_released: vec![false; 512],
            mouse_buttons_down: vec![false; 16],
            mouse_buttons_pressed: vec![false; 16],
            mouse_buttons_released: vec![false; 16],
        }
    }

    pub fn update(&mut self) {
        self.keys_pressed.fill(false);
        self.keys_released.fill(false);
        self.mouse_buttons_pressed.fill(false);
        self.mouse_buttons_released.fill(false);
        self.mouse_delta = Vec2::ZERO;
    }

    // Input methods
    pub fn is_key_down(&self, key: usize) -> bool {
        self.keys_down[key]
    }

    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keys_pressed[key]
    }

    pub fn is_key_released(&self, key: usize) -> bool {
        self.keys_released[key]
    }

    pub fn set_key_down(&mut self, key: usize, down: bool) {
        if down && !self.keys_down[key] {
            self.keys_pressed[key] = true;
        } else if !down && self.keys_down[key] {
            self.keys_released[key] = true;
        }
        self.keys_down[key] = down;
    }
} 