use gl;
use image;
use cgmath::{Matrix4, Point3, Vector3, Vector2, perspective, Deg};
use std::path::Path;
use std::ffi::CString;
use log::error;

pub struct Renderer {
    shader_program: u32,
    vao: u32,
    vbo: u32,
    ebo: u32,
    texture: u32,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32, shader_paths: (&str, &str), texture_path: &str) -> Self {
        println!("Loading texture from: {}", texture_path);
        
        let vertices: [f32; 20] = [
            // positions      // texture coords
             1.0,  1.0, 0.0,  1.0, 1.0,  // top right
             1.0, -1.0, 0.0,  1.0, 0.0,  // bottom right
            -1.0, -1.0, 0.0,  0.0, 0.0,  // bottom left
            -1.0,  1.0, 0.0,  0.0, 1.0   // top left
        ];
        
        let indices: [u32; 6] = [
            0, 1, 3,  // first triangle
            1, 2, 3   // second triangle
        ];

        unsafe {
            // Clear any existing GL errors
            while gl::GetError() != gl::NO_ERROR {}

            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;
            let mut texture = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::GenTextures(1, &mut texture);

            // Set up VAO and buffers
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW
            );

            // Set up vertex attributes
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * std::mem::size_of::<f32>() as i32, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const _
            );
            gl::EnableVertexAttribArray(1);

            // Load and set up texture
            gl::BindTexture(gl::TEXTURE_2D, texture);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Load image without flipping
            let img = match image::open(texture_path) {
                Ok(img) => img.into_rgba8(),
                Err(e) => {
                    error!("Failed to load texture: {}", e);
                    panic!("Failed to load texture: {}", e);
                }
            };

            println!("Image loaded: {}x{}", img.width(), img.height());

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_raw().as_ptr() as *const _
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Create and compile shaders
            let shader_program = create_shader_program(shader_paths.0, shader_paths.1);
            gl::UseProgram(shader_program);

            // Set texture uniform
            let texture_loc = gl::GetUniformLocation(shader_program, b"texture1\0".as_ptr() as *const _);
            gl::Uniform1i(texture_loc, 0); // Set texture unit 0

            Self {
                shader_program,
                vao,
                vbo,
                ebo,
                texture,
                width,
                height,
            }
        }
    }

    pub fn render(&self, position: Vector3<f32>, scale: f32) {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.shader_program);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao);

            let aspect_ratio = self.width as f32 / self.height as f32;
            
            let model = Matrix4::from_translation(position) * 
                       Matrix4::from_scale(scale) *
                       Matrix4::from_angle_z(Deg(180.0));
            
            let view = Matrix4::look_at_rh(
                Point3::new(0.0, 0.0, 3.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0)
            );
            
            let projection = perspective(
                Deg(45.0),
                aspect_ratio,
                0.1,
                100.0
            );

            let model_loc = gl::GetUniformLocation(self.shader_program, b"model\0".as_ptr() as *const _);
            let view_loc = gl::GetUniformLocation(self.shader_program, b"view\0".as_ptr() as *const _);
            let projection_loc = gl::GetUniformLocation(self.shader_program, b"projection\0".as_ptr() as *const _);

            let model_array: [[f32; 4]; 4] = model.into();
            let view_array: [[f32; 4]; 4] = view.into();
            let projection_array: [[f32; 4]; 4] = projection.into();

            gl::UniformMatrix4fv(model_loc, 1, gl::FALSE, model_array.as_ptr() as *const f32);
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, view_array.as_ptr() as *const f32);
            gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection_array.as_ptr() as *const f32);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            
            gl::Disable(gl::BLEND);

            let error = gl::GetError();
            if error != gl::NO_ERROR {
                println!("OpenGL error: {}", error);
            }
        }
    }

    pub fn render_scaled(&self, position: Vector3<f32>, scale: Vector2<f32>) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::BindVertexArray(self.vao);

            // Calculate aspect ratios
            let window_aspect = self.width as f32 / self.height as f32;
            let scale_factor = if window_aspect > 1.0 {
                // Wider than tall - fit to height
                Vector2::new(scale.x / window_aspect, scale.y)
            } else {
                // Taller than wide - fit to width
                Vector2::new(scale.x, scale.y * window_aspect)
            };

            let model = Matrix4::from_translation(position) * 
                       Matrix4::from_nonuniform_scale(scale_factor.x, scale_factor.y, 1.0);
            
            let view = Matrix4::look_at_rh(
                Point3::new(0.0, 0.0, 3.0),
                Point3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0)
            );
            
            let projection = perspective(
                Deg(45.0),
                window_aspect,
                0.1,
                100.0
            );

            let model_array: [[f32; 4]; 4] = model.into();
            let view_array: [[f32; 4]; 4] = view.into();
            let projection_array: [[f32; 4]; 4] = projection.into();

            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program, b"model\0".as_ptr() as *const _),
                1,
                gl::FALSE,
                model_array.as_ptr() as *const f32
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program, b"view\0".as_ptr() as *const _),
                1,
                gl::FALSE,
                view_array.as_ptr() as *const f32
            );
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.shader_program, b"projection\0".as_ptr() as *const _),
                1,
                gl::FALSE,
                projection_array.as_ptr() as *const f32
            );

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            
            gl::Disable(gl::BLEND);
        }
    }
}

fn create_shader_program(vert_path: &str, frag_path: &str) -> u32 {
    unsafe {
        let vertex_shader = compile_shader(vert_path, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(frag_path, gl::FRAGMENT_SHADER);

        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Check for linking errors
        let mut success = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );
            panic!("Shader program linking failed: {}", String::from_utf8_lossy(&info_log));
        }

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        program
    }
}

fn compile_shader(path: &str, shader_type: u32) -> u32 {
    unsafe {
        let source = std::fs::read_to_string(path)
            .expect(&format!("Failed to read shader file: {}", path));
        let source = CString::new(source.as_bytes()).unwrap();
        
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut i8,
            );
            panic!(
                "Shader compilation failed for {}: {}",
                path,
                String::from_utf8_lossy(&info_log)
            );
        }

        shader
    }
} 