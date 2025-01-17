use glam::{Mat4, Vec2, Vec3, Vec4};
use std::collections::HashMap;

const MAX_BATCH_SIZE: usize = 1000;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coords: Vec2,
    pub color: Vec4,
}

pub struct SpriteBatch {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    texture_id: Option<u32>,
    transform: Mat4,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            vertices: Vec::with_capacity(MAX_BATCH_SIZE * 4),
            indices: Vec::with_capacity(MAX_BATCH_SIZE * 6),
            texture_id: None,
            transform: Mat4::IDENTITY,
        }
    }

    pub fn can_add(&self, texture_id: Option<u32>) -> bool {
        self.texture_id == texture_id && self.vertices.len() < MAX_BATCH_SIZE * 4
    }

    pub fn add_sprite(
        &mut self,
        position: Vec3,
        size: Vec2,
        color: Vec4,
        tex_coords: [Vec2; 4],
        texture_id: Option<u32>,
    ) {
        if self.texture_id.is_none() {
            self.texture_id = texture_id;
        }

        let base_index = self.vertices.len() as u32;

        // Add vertices
        self.vertices.extend_from_slice(&[
            Vertex {
                position: position + Vec3::new(-size.x / 2.0, -size.y / 2.0, 0.0),
                tex_coords: tex_coords[0],
                color,
            },
            Vertex {
                position: position + Vec3::new(size.x / 2.0, -size.y / 2.0, 0.0),
                tex_coords: tex_coords[1],
                color,
            },
            Vertex {
                position: position + Vec3::new(size.x / 2.0, size.y / 2.0, 0.0),
                tex_coords: tex_coords[2],
                color,
            },
            Vertex {
                position: position + Vec3::new(-size.x / 2.0, size.y / 2.0, 0.0),
                tex_coords: tex_coords[3],
                color,
            },
        ]);

        // Add indices
        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.texture_id = None;
    }
}

pub struct BatchRenderer {
    batches: Vec<SpriteBatch>,
    current_batch: usize,
}

impl BatchRenderer {
    pub fn new() -> Self {
        Self {
            batches: vec![SpriteBatch::new()],
            current_batch: 0,
        }
    }

    pub fn submit_sprite(
        &mut self,
        position: Vec3,
        size: Vec2,
        color: Vec4,
        tex_coords: [Vec2; 4],
        texture_id: Option<u32>,
    ) {
        if !self.batches[self.current_batch].can_add(texture_id) {
            self.flush_current_batch();
        }
        self.batches[self.current_batch].add_sprite(position, size, color, tex_coords, texture_id);
    }

    pub fn flush_current_batch(&mut self) {
        self.current_batch += 1;
        if self.current_batch >= self.batches.len() {
            self.batches.push(SpriteBatch::new());
        }
    }

    pub fn render(&mut self) {
        // Implement actual rendering here
        for batch in &self.batches[..=self.current_batch] {
            // Render batch
        }

        // Reset state
        for batch in &mut self.batches {
            batch.clear();
        }
        self.current_batch = 0;
    }
}
