mod platform;
mod system;

use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub use platform::PlatformAudio;
pub use system::AudioSystem;

#[derive(Debug, Clone)]
pub struct AudioSource {
    pub buffer: Arc<Vec<f32>>,
    pub sample_rate: u32,
    pub channels: u8,
    pub loop_audio: bool,
    pub volume: f32,
    pub pitch: f32,
    pub position: f32, // Playback position in seconds
}

impl AudioSource {
    pub fn new(buffer: Vec<f32>, sample_rate: u32, channels: u8) -> Self {
        Self {
            buffer: Arc::new(buffer),
            sample_rate,
            channels,
            loop_audio: false,
            volume: 1.0,
            pitch: 1.0,
            position: 0.0,
        }
    }

    pub fn duration(&self) -> f32 {
        self.buffer.len() as f32 / (self.sample_rate as f32 * self.channels as f32)
    }
}

#[derive(Debug)]
pub struct AudioClip {
    pub name: String,
    pub source: AudioSource,
    pub category: String,
}

#[derive(Debug)]
pub struct AudioListener {
    pub position: glam::Vec3,
    pub forward: glam::Vec3,
    pub up: glam::Vec3,
    pub velocity: glam::Vec3,
}

impl Default for AudioListener {
    fn default() -> Self {
        Self {
            position: glam::Vec3::ZERO,
            forward: glam::Vec3::new(0.0, 0.0, -1.0),
            up: glam::Vec3::Y,
            velocity: glam::Vec3::ZERO,
        }
    }
}

#[derive(Debug)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub category_volumes: HashMap<String, f32>,
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub channels: u8,
}

impl Default for AudioConfig {
    fn default() -> Self {
        let mut category_volumes = HashMap::new();
        category_volumes.insert("master".to_string(), 1.0);
        category_volumes.insert("music".to_string(), 0.8);
        category_volumes.insert("sfx".to_string(), 1.0);
        category_volumes.insert("voice".to_string(), 1.0);

        Self {
            master_volume: 1.0,
            category_volumes,
            sample_rate: 44100,
            buffer_size: 1024,
            channels: 2,
        }
    }
}
