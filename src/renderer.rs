// Add vertex buffer object (VBO) caching
pub struct RenderCache {
    vertex_buffers: HashMap<String, gl::types::GLuint>,
    shader_cache: HashMap<String, Shader>,
}

impl Renderer {
    // Batch similar draw calls
    pub fn batch_draw(&mut self, sprites: &[Sprite]) {
        let mut batches: HashMap<TextureId, Vec<Sprite>> = HashMap::new();

        // Group sprites by texture
        for sprite in sprites {
            batches
                .entry(sprite.texture_id)
                .or_default()
                .push(sprite.clone());
        }

        // Draw each batch
        for (texture_id, batch) in batches {
            self.bind_texture(texture_id);
            self.draw_batch(&batch);
        }
    }
}
