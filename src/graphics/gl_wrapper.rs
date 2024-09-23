use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::os::raw::*;
use std::ptr;

use gl::types::*;
use cgmath::*;

/// # Vertex Array Object (VAO)
pub struct Vao {
    id: GLuint,
}

impl Vao {
    /// Creates a new Vertex Array Object.
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    /// Binds the VAO.
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    /// Unbinds the VAO.
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

/// # Buffer Object (VBO)
pub struct BufferObject {
    id: GLuint,
    target: GLenum,
    usage: GLenum,
}

impl BufferObject {
    /// Creates a new Buffer Object (VBO).
    pub fn new(target: GLenum, usage: GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        Self { id, target, usage }
    }

    /// Binds the buffer object.
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.id);
        }
    }

    /// Unbinds the buffer object.
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    /// Stores float data in the buffer.
    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                self.usage,
            );
        }
    }

    /// Stores integer data in the buffer.
    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * mem::size_of::<GLint>()) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                self.usage,
            );
        }
    }
}

/// # Vertex Attribute
pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    /// Creates a new vertex attribute.
    pub fn new(
        index: GLuint,
        size: GLint,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> Self {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }
        Self { index }
    }

    /// Enables the vertex attribute.
    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    /// Disables the vertex attribute.
    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}

/// # Shader Program
pub struct ShaderProgram {
    id: GLuint,
    uniforms: HashMap<String, GLint>,
}

impl ShaderProgram {
    /// Creates a new shader program from vertex and fragment shader files.
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Self {
        let vertex_shader_source = Self::load_shader_source(vertex_shader_path);
        let fragment_shader_source = Self::load_shader_source(fragment_shader_path);

        unsafe {
            let vertex_shader = Self::compile_shader(&vertex_shader_source, gl::VERTEX_SHADER);
            let fragment_shader = Self::compile_shader(&fragment_shader_source, gl::FRAGMENT_SHADER);

            let id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Self {
                id,
                uniforms: HashMap::new(),
            }
        }
    }

    /// Loads shader source code from a file.
    fn load_shader_source(path: &str) -> String {
        let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", path));
        let mut source = String::new();
        file.read_to_string(&mut source).expect("Failed to read shader");
        source
    }

    /// Compiles a shader from source code.
    unsafe fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader = gl::CreateShader(shader_type);
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        shader
    }

    /// Binds the shader program.
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// Unbinds the current shader program.
    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    /// Creates a uniform location in the shader program.
    pub fn create_uniform(&mut self, name: &str) {
        let location = unsafe {
            gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr())
        };
        if location < 0 {
            panic!("Uniform '{}' not found in shader program", name);
        } else {
            self.uniforms.insert(name.to_string(), location);
        }
    }

    /// Sets a matrix uniform (4x4 float) in the shader program.
    pub fn set_matrix4fv_uniform(&self, name: &str, matrix: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(
                *self.uniforms.get(name).expect("Uniform not found"),
                1,
                gl::FALSE,
                matrix.as_ptr(),
            );
        }
    }
}
