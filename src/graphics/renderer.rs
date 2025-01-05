use super::gl_wrapper::{ShaderProgram, Vao};
use super::texture::Texture;
use cgmath::{Matrix4, Vector3};

pub struct Renderer {
    shader: ShaderProgram,
    vao: Vao,
    texture: Texture,
    projection: Matrix4<f32>,
}

impl Renderer {
    pub fn new(
        shader_path: (&str, &str),
        texture_path: &str,
        width: u32,
        height: u32
    ) -> Self {
        let shader = ShaderProgram::new(shader_path.0, shader_path.1);
        
        // Create VAO with proper aspect ratio for the anime character
        let mut vao = Vao::new();
        let aspect = width as f32 / height as f32;
        let vertices: [f32; 20] = [
            aspect,  1.0, 0.0,   1.0, 1.0,
            -aspect,  1.0, 0.0,   0.0, 1.0,
            -aspect, -1.0, 0.0,   0.0, 0.0,
            aspect, -1.0, 0.0,   1.0, 0.0,
        ];
        vao.add_vertex_buffer(&vertices, &[(0, 3), (1, 2)]);

        // Load texture with proper filtering for anime art
        let texture = Texture::new(texture_path)
            .expect("Failed to load texture");

        // Adjust projection for better character view
        let projection = cgmath::perspective(
            cgmath::Deg(40.0), // Narrower FOV for less distortion
            width as f32 / height as f32,
            0.1,
            100.0
        );

        Renderer {
            shader,
            vao,
            texture,
            projection,
        }
    }

    pub fn render(&self, position: Vector3<f32>, scale: f32) {
        self.shader.bind();
        
        let model = Matrix4::from_translation(position) * Matrix4::from_scale(scale);
        
        self.shader.set_matrix4fv("projection", &self.projection);
        self.shader.set_matrix4fv("model", &model);
        self.shader.set_int("textureSampler", 0);
        
        // Enable alpha blending for transparency
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        
        self.texture.bind();
        self.vao.bind();
        
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }
    }
} 