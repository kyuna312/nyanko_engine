use gl;
use glam::{Vec2, Vec3, Mat4};
use image::GenericImageView;
use super::texture::Texture;

#[derive(Clone)]
pub struct RendererConfig {
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
        }
    }
}

pub struct Renderer {
    width: u32,
    height: u32,
    shader_program: u32,
    sprite_vao: u32,
    sprite_vbo: u32,
    projection: Mat4,
}

impl Renderer {
    pub fn new(config: &RendererConfig) -> Self {
        let shader_program = create_shader_program();
        let (sprite_vao, sprite_vbo) = create_sprite_buffers();
        
        let projection = Mat4::orthographic_rh(
            0.0,
            config.window_width as f32,
            config.window_height as f32,
            0.0,
            -1.0,
            1.0
        );

        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Self {
            width: config.window_width,
            height: config.window_height,
            shader_program,
            sprite_vao,
            sprite_vbo,
            projection,
        }
    }

    pub fn begin_frame(&self) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn end_frame(&self) {
        // Frame end - buffer swap is handled by the window system
    }

    pub fn draw_sprite(&self, texture: &Texture, position: Vec2, size: Vec2) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.sprite_vao);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);

            let model = Mat4::from_translation(Vec3::new(position.x, position.y, 0.0))
                * Mat4::from_scale(Vec3::new(size.x, size.y, 1.0));

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program, b"projection\0".as_ptr() as *const _),
                1,
                gl::FALSE,
                self.projection.as_ref().as_ptr(),
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program, b"model\0".as_ptr() as *const _),
                1,
                gl::FALSE,
                model.as_ref().as_ptr(),
            );

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

fn create_sprite_buffers() -> (u32, u32) {
    let vertices: [f32; 24] = [
        // position    // texcoords
        0.0, 0.0,     0.0, 0.0,  // bottom left
        1.0, 0.0,     1.0, 0.0,  // bottom right
        1.0, 1.0,     1.0, 1.0,  // top right
        
        0.0, 0.0,     0.0, 0.0,  // bottom left
        1.0, 1.0,     1.0, 1.0,  // top right
        0.0, 1.0,     0.0, 1.0   // top left
    ];

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Position attribute
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        
        // Texture coords attribute
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as i32,
            (2 * std::mem::size_of::<f32>()) as *const _,
        );
    }
    
    (vao, vbo)
}

fn create_shader_program() -> u32 {
    let vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec2 aPos;
        layout (location = 1) in vec2 aTexCoord;
        
        uniform mat4 projection;
        uniform mat4 model;
        
        out vec2 TexCoord;
        
        void main() {
            gl_Position = projection * model * vec4(aPos, 0.0, 1.0);
            TexCoord = aTexCoord;
        }
    "#;

    let fragment_shader = r#"
        #version 330 core
        in vec2 TexCoord;
        
        uniform sampler2D texture1;
        
        out vec4 FragColor;
        
        void main() {
            FragColor = texture(texture1, TexCoord);
        }
    "#;

    unsafe {
        // Create vertex shader
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str = std::ffi::CString::new(vertex_shader.as_bytes()).unwrap();
        gl::ShaderSource(vertex, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(vertex);
        check_shader_errors(vertex, "vertex");

        // Create fragment shader
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str = std::ffi::CString::new(fragment_shader.as_bytes()).unwrap();
        gl::ShaderSource(fragment, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(fragment);
        check_shader_errors(fragment, "fragment");

        // Create shader program
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex);
        gl::AttachShader(program, fragment);
        gl::LinkProgram(program);
        check_program_errors(program);

        // Delete shaders
        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        program
    }
}

fn check_shader_errors(shader: u32, shader_type: &str) {
    unsafe {
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            info_log.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            panic!(
                "Failed to compile {} shader: {}",
                shader_type,
                std::str::from_utf8(&info_log).unwrap()
            );
        }
    }
}

fn check_program_errors(program: u32) {
    unsafe {
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            info_log.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut gl::types::GLchar,
            );
            panic!(
                "Failed to link shader program: {}",
                std::str::from_utf8(&info_log).unwrap()
            );
        }
    }
} 