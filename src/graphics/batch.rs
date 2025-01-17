use crate::graphics::texture::Texture;
use glam::{Mat4, Vec2, Vec4};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct BatchRenderer {
    vao: u32,
    vbo: u32,
    ibo: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
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

        Self {
            vao,
            vbo,
            ibo,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn begin(&mut self, transform: Mat4) {
        self.vertices.clear();
        self.indices.clear();

        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn flush(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
                self.vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );

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
        let base_index = (self.vertices.len() / 5) as u32;

        // Add vertices
        let x = position.x;
        let y = position.y;
        let w = size.x;
        let h = size.y;

        // Vertex positions and texture coordinates
        let vertices = [
            x,
            y,
            0.0,
            0.0, // Top-left
            x + w,
            y,
            1.0,
            0.0, // Top-right
            x + w,
            y + h,
            1.0,
            1.0, // Bottom-right
            x,
            y + h,
            0.0,
            1.0, // Bottom-left
        ];

        self.vertices.extend_from_slice(&vertices);

        // Add indices
        let indices = [
            base_index,
            base_index + 1,
            base_index + 2,
            base_index + 2,
            base_index + 3,
            base_index,
        ];

        self.indices.extend_from_slice(&indices);
    }
}

impl Drop for BatchRenderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ibo);
        }
    }
}
