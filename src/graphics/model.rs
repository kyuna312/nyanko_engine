use super::texture::Texture;
use cgmath::{EuclideanSpace, Matrix4, Point3, Vector3};

pub struct Material {
    pub diffuse_texture: Option<Texture>,
    pub specular_texture: Option<Texture>,
    pub shininess: f32,
}

pub struct Model {
    vao: super::gl_wrapper::Vao,
    indices: Vec<u32>,
    position: Point3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    material: Material,
}

impl Model {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, material: Material) -> Self {
        let mut vao = super::gl_wrapper::Vao::new();
        vao.add_vertex_buffer(
            &vertices,
            &[
                (0, 3), // position
                (1, 3), // normal
                (2, 2), // texture coordinates
            ],
        );

        unsafe {
            let mut ebo = 0;
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );
        }

        Model {
            vao,
            indices,
            position: Point3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            material,
        }
    }

    pub fn get_model_matrix(&self) -> Matrix4<f32> {
        let translation = Matrix4::from_translation(self.position.to_vec());
        let rotation_x = Matrix4::from_angle_x(cgmath::Rad(self.rotation.x));
        let rotation_y = Matrix4::from_angle_y(cgmath::Rad(self.rotation.y));
        let rotation_z = Matrix4::from_angle_z(cgmath::Rad(self.rotation.z));
        let scale = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        translation * rotation_z * rotation_y * rotation_x * scale
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = Point3::new(x, y, z);
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        self.rotation = Vector3::new(x, y, z);
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scale = Vector3::new(x, y, z);
    }

    pub fn draw(&self, shader: &mut super::gl_wrapper::ShaderProgram) {
        shader.bind();

        // Set material properties
        if let Some(ref texture) = self.material.diffuse_texture {
            texture.bind();
            shader.set_int("material.diffuse", 0);
        }
        if let Some(ref texture) = self.material.specular_texture {
            texture.bind();
            shader.set_int("material.specular", 1);
        }
        shader.set_float("material.shininess", self.material.shininess);

        shader.set_matrix4fv_uniform("model", &self.get_model_matrix());
        self.vao.bind();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        super::gl_wrapper::Vao::unbind();
        super::gl_wrapper::ShaderProgram::unbind();
    }
}
