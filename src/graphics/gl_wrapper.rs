use cgmath::{Matrix, Matrix4, Vector2, Vector3, Vector4};
use gl::types::*;
use std::ffi::CString;
use std::fs;

pub struct Vao {
    id: GLuint,
    vbos: Vec<BufferObject>,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao {
            id,
            vbos: Vec::new(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn add_vertex_buffer(
        &mut self,
        data: &[f32],
        attributes: &[(GLuint, GLint)],
    ) -> &BufferObject {
        self.bind();
        let vbo = BufferObject::new();
        vbo.bind();

        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let mut offset: usize = 0;
            let stride = attributes
                .iter()
                .map(|(_, size)| *size as usize)
                .sum::<usize>();

            for &(location, size) in attributes {
                gl::EnableVertexAttribArray(location);
                gl::VertexAttribPointer(
                    location,
                    size,
                    gl::FLOAT,
                    gl::FALSE,
                    (stride * std::mem::size_of::<f32>()) as GLsizei,
                    (offset * std::mem::size_of::<f32>()) as *const _,
                );
                offset += size as usize;
            }
        }

        self.vbos.push(vbo);
        self.vbos.last().unwrap()
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_source = fs::read_to_string(vertex_path).expect(&format!(
            "Failed to read vertex shader from {}",
            vertex_path
        ));
        let fragment_source = fs::read_to_string(fragment_path).expect(&format!(
            "Failed to read fragment shader from {}",
            fragment_path
        ));

        unsafe {
            let vertex_shader = Self::compile_shader(&vertex_source, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(&fragment_source, gl::FRAGMENT_SHADER);

            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader);
            gl::AttachShader(program_id, fragment_shader);
            gl::LinkProgram(program_id);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::with_capacity(len as usize);
                info_log.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                // Print shader sources for debugging
                println!("Vertex Shader Source:\n{}", vertex_source);
                println!("Fragment Shader Source:\n{}", fragment_source);

                panic!(
                    "Shader program linking failed: {}\nVertex path: {}\nFragment path: {}",
                    String::from_utf8_lossy(&info_log),
                    vertex_path,
                    fragment_path
                );
            }

            // Clean up shaders after linking
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            ShaderProgram { id: program_id }
        }
    }

    unsafe fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // Check for compilation errors
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
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let shader_type_str = match shader_type {
                gl::VERTEX_SHADER => "vertex",
                gl::FRAGMENT_SHADER => "fragment",
                _ => "unknown",
            };
            panic!(
                "{} shader compilation failed: {}\nSource:\n{}",
                shader_type_str,
                String::from_utf8_lossy(&info_log),
                source
            );
        }

        shader
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_vector2f(&self, name: &str, value: &Vector2<f32>) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform2f(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value.x,
                value.y,
            );
        }
    }

    pub fn set_vector3f(&self, name: &str, value: &Vector3<f32>) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value.x,
                value.y,
                value.z,
            );
        }
    }

    pub fn set_vector4f(&self, name: &str, value: &Vector4<f32>) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value.x,
                value.y,
                value.z,
                value.w,
            );
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value as i32,
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, c_name.as_ptr()), value);
        }
    }

    pub fn set_matrix4fv(&self, name: &str, value: &Matrix4<f32>) {
        let c_name = CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

pub struct BufferObject {
    id: GLuint,
}

impl BufferObject {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
