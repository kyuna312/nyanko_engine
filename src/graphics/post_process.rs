use gl::types::*;
use super::gl_wrapper::{Vao, ShaderProgram};

pub struct PostProcessor {
    fbo: GLuint,
    screen_texture: GLuint,
    quad_vao: Vao,
    effects_shader: ShaderProgram,
}

pub struct PostProcessParams {
    pub enable_bloom: bool,
    pub enable_chromatic: bool,
    pub enable_vignette: bool,
    pub bloom_intensity: f32,
    pub chromatic_strength: f32,
    pub vignette_intensity: f32,
    pub vignette_roundness: f32,
    pub vignette_smoothness: f32,
}

impl Default for PostProcessParams {
    fn default() -> Self {
        PostProcessParams {
            enable_bloom: true,
            enable_chromatic: true,
            enable_vignette: true,
            bloom_intensity: 0.5,
            chromatic_strength: 0.003,
            vignette_intensity: 0.4,
            vignette_roundness: 1.0,
            vignette_smoothness: 0.3,
        }
    }
}

impl PostProcessor {
    pub fn new(width: u32, height: u32) -> Self {
        let mut fbo = 0;
        let mut screen_texture = 0;

        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::GenTextures(1, &mut screen_texture);
            gl::BindTexture(gl::TEXTURE_2D, screen_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                screen_texture,
                0,
            );
        }

        let mut quad_vao = Vao::new();
        let vertices: [f32; 30] = [
            // positions        // texCoords
            -1.0,  1.0, 0.0,   0.0, 1.0,  // Top-left
            -1.0, -1.0, 0.0,   0.0, 0.0,  // Bottom-left
             1.0, -1.0, 0.0,   1.0, 0.0,  // Bottom-right

            -1.0,  1.0, 0.0,   0.0, 1.0,  // Top-left
             1.0, -1.0, 0.0,   1.0, 0.0,  // Bottom-right
             1.0,  1.0, 0.0,   1.0, 1.0   // Top-right
        ];
        quad_vao.add_vertex_buffer(&vertices, &[(0, 3), (1, 2)]);

        let effects_shader = ShaderProgram::new(
            "assets/shaders/post_process.vert",
            "assets/shaders/post_process.frag",
        );

        PostProcessor {
            fbo,
            screen_texture,
            quad_vao,
            effects_shader,
        }
    }

    pub fn begin_render(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn end_render(&mut self, params: &PostProcessParams, bloom_texture: GLuint) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            self.effects_shader.bind();
            
            // Set uniforms
            self.effects_shader.set_bool("enableBloom", params.enable_bloom);
            self.effects_shader.set_bool("enableChromatic", params.enable_chromatic);
            self.effects_shader.set_bool("enableVignette", params.enable_vignette);
            self.effects_shader.set_float("bloomIntensity", params.bloom_intensity);
            self.effects_shader.set_float("chromaticStrength", params.chromatic_strength);
            self.effects_shader.set_float("vignetteIntensity", params.vignette_intensity);
            self.effects_shader.set_float("vignetteRoundness", params.vignette_roundness);
            self.effects_shader.set_float("vignetteSmoothness", params.vignette_smoothness);
            
            // Bind textures
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.screen_texture);
            self.effects_shader.set_int("screenTexture", 0);
            
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, bloom_texture);
            self.effects_shader.set_int("bloomTexture", 1);
            
            self.quad_vao.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }

    pub fn get_screen_texture(&self) -> u32 {
        self.screen_texture
    }
}

impl Drop for PostProcessor {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.fbo);
            gl::DeleteTextures(1, &self.screen_texture);
        }
    }
} 