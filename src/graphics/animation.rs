use std::time::Duration;
use super::texture::Texture;
use glam::Vec2;

#[derive(Clone)]
pub struct AnimationFrame {
    pub texture: Texture,
    pub duration: Duration,
    pub offset: Vec2,
    pub size: Vec2,
}

#[derive(Clone)]
pub struct Animation {
    pub name: String,
    pub frames: Vec<AnimationFrame>,
    pub looping: bool,
    current_frame: usize,
    time_accumulated: Duration,
}

impl Animation {
    pub fn new(name: String, frames: Vec<AnimationFrame>, looping: bool) -> Self {
        Self {
            name,
            frames,
            looping,
            current_frame: 0,
            time_accumulated: Duration::ZERO,
        }
    }

    pub fn update(&mut self, dt: Duration) -> bool {
        if self.frames.is_empty() {
            return false;
        }

        self.time_accumulated += dt;
        let frame_duration = self.frames[self.current_frame].duration;

        if self.time_accumulated >= frame_duration {
            self.time_accumulated -= frame_duration;
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.looping {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn current_frame(&self) -> Option<&AnimationFrame> {
        self.frames.get(self.current_frame)
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.time_accumulated = Duration::ZERO;
    }
} 