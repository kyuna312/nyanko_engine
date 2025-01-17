use super::{Camera, ParticleSystem, Shader, Texture, Vertex};
use crate::platform::PlatformWindow;
use glam::{Mat4, Vec3};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub msaa_samples: u32,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            msaa_samples: 4,
        }
    }
}

pub struct Renderer {
    config: RendererConfig,
    window: Arc<PlatformWindow>,
    shaders: HashMap<String, Arc<Shader>>,
    textures: HashMap<String, Arc<RwLock<Texture>>>,
    particle_system: ParticleSystem,
    camera: Camera,
}

impl Renderer {
    pub fn new(window: PlatformWindow, config: RendererConfig) -> Self {
        let window = Arc::new(window);
        let particle_system = ParticleSystem::new();
        let camera = Camera::new(Vec3::new(0.0, 0.0, -10.0), Vec3::ZERO, Vec3::Y);

        Self {
            config,
            window,
            shaders: HashMap::new(),
            textures: HashMap::new(),
            particle_system,
            camera,
        }
    }

    pub fn draw(&mut self) {
        // Implementation
    }
}
