use std::path::Path;

pub struct Shader {
    program_id: gl::types::GLuint,
}

impl Shader {
    pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Self, String> {
        unsafe {
            let vertex_shader = compile_shader(vertex_source, gl::VERTEX_SHADER)?;
            let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER)?;

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program_id: program })
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }
}

fn compile_shader(source: &str, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let c_str = std::ffi::CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as gl::types::GLint {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = Vec::with_capacity(len as usize);
            buffer.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), buffer.as_mut_ptr() as *mut gl::types::GLchar);
            return Err(String::from_utf8_lossy(&buffer).to_string());
        }

        Ok(shader)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

    pub fn from_source(source: &str, shader_type: GLenum) -> Result<Self, String> {
        let source = CString::new(source).unwrap();
        let id = unsafe {
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            // Check for compilation errors
            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                return Err(String::from_utf8_unchecked(buffer));
            }
            shader
        };

        Ok(Self { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

pub struct ShaderProgram {
    id: GLuint,
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let vertex_shader = Shader::from_source(vertex_src, gl::VERTEX_SHADER)
            .expect("Failed to compile vertex shader");
        let fragment_shader = Shader::from_source(fragment_src, gl::FRAGMENT_SHADER)
            .expect("Failed to compile fragment shader");

        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader.id);
            gl::AttachShader(program, fragment_shader.id);
            gl::LinkProgram(program);

            // Check for linking errors
            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    program,
                    len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "Shader linking error: {}",
                    String::from_utf8_unchecked(buffer)
                );
            }

            Self { id: program }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1i(location, value as i32);
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_vec2(&self, name: &str, value: &Vec2) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform2f(location, value.x, value.y);
        }
    }

    pub fn set_vec3(&self, name: &str, value: &Vec3) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_vec4(&self, name: &str, value: &Vec4) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn set_mat4(&self, name: &str, value: &Mat4) {
        unsafe {
            let location = self.get_uniform_location(name);
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.to_cols_array().as_ptr());
        }
    }

    fn get_uniform_location(&self, name: &str) -> GLint {
        let name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.id, name.as_ptr()) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
