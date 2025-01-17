use super::*;
use crate::platform::PlatformWindow;
use std::collections::HashMap;

pub struct Renderer {
    config: RendererConfig,
    window: Arc<PlatformWindow>,
    shaders: HashMap<String, Arc<ShaderProgram>>,
    textures: HashMap<String, Arc<RwLock<Texture>>>,
    batch_renderer: BatchRenderer,
    particle_system: ParticleSystem,
    camera: Camera,
    projection: Mat4,
    view: Mat4,
}

impl Renderer {
    pub fn new(window: &PlatformWindow, config: RendererConfig) -> Self {
        let batch_renderer = BatchRenderer::new();
        let particle_system = ParticleSystem::new();
        let camera = Camera::new(Vec3::new(0.0, 0.0, -10.0), Vec3::ZERO, Vec3::Y);

        let aspect = config.width as f32 / config.height as f32;
        let projection = Mat4::perspective_rh(45.0f32.to_radians(), aspect, 0.1, 1000.0);

        Self {
            config,
            window: Arc::new(window.clone()),
            shaders: HashMap::new(),
            textures: HashMap::new(),
            batch_renderer,
            particle_system,
            camera,
            projection,
            view: camera.view_matrix(),
        }
    }

    pub fn begin_frame(&mut self) {
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.view = self.camera.view_matrix();
        self.batch_renderer.begin(self.projection * self.view);
    }

    pub fn end_frame(&mut self) {
        self.batch_renderer.flush();
        self.particle_system.render(&self.projection, &self.view);
        self.window.swap_buffers();
    }

    pub fn draw_sprite(
        &mut self,
        position: Vec3,
        size: Vec2,
        texture: Option<Arc<RwLock<Texture>>>,
        color: Vec4,
    ) {
        self.batch_renderer
            .draw_sprite(position, size, texture, color);
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, transform: Mat4) {
        let shader = self.get_default_shader();
        shader.bind();
        shader.set_mat4("uProjection", &self.projection);
        shader.set_mat4("uView", &self.view);
        shader.set_mat4("uModel", &transform);

        if let Some(texture) = &mesh.texture {
            texture.read().bind(0);
            shader.set_int("uTexture", 0);
        }

        unsafe {
            // Create and bind VAO/VBO/EBO
            let mut vao = 0;
            let mut vbo = 0;
            let mut ebo = 0;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            // Upload vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
                mesh.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Upload index data
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
                mesh.indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Set up vertex attributes
            let stride = std::mem::size_of::<Vertex>() as i32;

            // Position
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, 0 as *const _);
            gl::EnableVertexAttribArray(0);

            // Texture coordinates
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            // Color
            gl::VertexAttribPointer(
                2,
                4,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (5 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(2);

            // Draw
            gl::DrawElements(
                gl::TRIANGLES,
                mesh.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _,
            );

            // Cleanup
            gl::DeleteVertexArrays(1, &vao);
            gl::DeleteBuffers(1, &vbo);
            gl::DeleteBuffers(1, &ebo);
        }
    }

    pub fn load_shader(
        &mut self,
        name: &str,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Arc<ShaderProgram> {
        let shader = Arc::new(ShaderProgram::new(vertex_src, fragment_src));
        self.shaders.insert(name.to_string(), shader.clone());
        shader
    }

    pub fn load_texture(&mut self, name: &str, data: &[u8]) -> Arc<RwLock<Texture>> {
        let texture = Arc::new(RwLock::new(Texture::from_memory(data)));
        self.textures.insert(name.to_string(), texture.clone());
        texture
    }

    fn get_default_shader(&self) -> Arc<ShaderProgram> {
        self.shaders
            .get("default")
            .expect("Default shader not loaded")
            .clone()
    }
}
