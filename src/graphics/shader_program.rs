use std::ffi::CString;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let vertex_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
        let fragment_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            program
        };

        Self { id: program }
    }
}

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();

    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    shader
}
