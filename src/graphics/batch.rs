use crate::math::*;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct BatchRenderer {
    vao: u32,
    vbo: u32,
    ibo: u32,
}

impl BatchRenderer {
    pub fn new() -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ibo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ibo);
        }

        Self { vao, vbo, ibo }
    }

    pub fn begin(&mut self, transform: Mat4) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_sprite(
        &mut self,
        position: Vec2,
        size: Vec2,
        texture: Option<Arc<RwLock<Texture>>>,
        color: Vec4,
    ) {
        // Implement sprite drawing
    }
}
