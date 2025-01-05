use super::gl_wrapper::{ShaderProgram, Vao};
use super::texture::Texture;

pub struct BloomProcessor {
    shader: ShaderProgram,
    vao: Vao,
    bright_texture: Texture,
    blur_textures: Vec<Texture>,
}

impl BloomProcessor {
    pub fn new(width: u32, height: u32) -> Self {
        let shader = ShaderProgram::new(
            "assets/shaders/bloom.vert",
            "assets/shaders/bloom.frag"
        );

        let mut vao = Vao::new();
        let vertices: [f32; 20] = [
            1.0,  1.0, 0.0,   1.0, 1.0,
            -1.0,  1.0, 0.0,   0.0, 1.0,
            -1.0, -1.0, 0.0,   0.0, 0.0,
            1.0, -1.0, 0.0,   1.0, 0.0,
        ];
        vao.add_vertex_buffer(&vertices, &[(0, 3), (1, 2)]);

        let bright_texture = Texture::new_empty(width, height);
        let mut blur_textures = Vec::new();
        
        // Create mip chain for bloom
        let mip_levels = 5;
        for i in 0..mip_levels {
            let mip_width = width >> i;
            let mip_height = height >> i;
            blur_textures.push(Texture::new_empty(mip_width, mip_height));
        }

        BloomProcessor {
            shader,
            vao,
            bright_texture,
            blur_textures,
        }
    }

    pub fn process(&self, input_texture: u32, settings: &BloomSettings) -> u32 {
        self.shader.bind();
        self.shader.set_int("inputTexture", 0);
        self.shader.set_float("threshold", settings.threshold);
        
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, input_texture);
        }
        
        self.bright_texture.id()
    }
}

pub struct BloomSettings {
    pub threshold: f32,
    pub softness: f32,
    pub intensity: f32,
}

impl Default for BloomSettings {
    fn default() -> Self {
        BloomSettings {
            threshold: 0.7,
            softness: 0.5,
            intensity: 1.2,
        }
    }
} 