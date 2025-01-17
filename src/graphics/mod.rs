mod batch;
mod particles;
mod renderer;
mod renderer_config;
mod shader;
mod texture;

pub use batch::{BatchRenderer, SpriteBatch};
pub use particles::{ParticleEmitter, ParticleSystem};
pub use renderer::Renderer;
pub use renderer_config::{RenderAPI, RendererConfig};
pub use shader::{Shader, ShaderProgram};
pub use texture::Texture;

use glam::{Mat4, Vec2, Vec3, Vec4};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coords: Vec2,
    pub color: Vec4,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture: Option<Arc<RwLock<Texture>>>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            texture: None,
        }
    }

    pub fn with_texture(mut self, texture: Arc<RwLock<Texture>>) -> Self {
        self.texture = Some(texture);
        self
    }
}
