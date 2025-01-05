use crate::graphics::gl_wrapper::{ShaderProgram, Vao};
use cgmath::{Vector2, Vector4};
use gl::types::*;

pub struct UiManager {
    shader: ShaderProgram,
    quad_vao: Vao,
    elements: Vec<UiElement>,
}

pub struct UiElement {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub color: Vector4<f32>,
    pub texture_id: Option<GLuint>,
    pub is_visible: bool,
}

impl UiManager {
    pub fn new() -> Self {
        let shader = ShaderProgram::new(
            "assets/shaders/ui.vert",
            "assets/shaders/ui.frag",
        );

        let mut quad_vao = Vao::new();
        let vertices: [f32; 24] = [
            // pos      // tex
            0.0, 1.0,   0.0, 1.0,
            0.0, 0.0,   0.0, 0.0,
            1.0, 0.0,   1.0, 0.0,
            0.0, 1.0,   0.0, 1.0,
            1.0, 0.0,   1.0, 0.0,
            1.0, 1.0,   1.0, 1.0,
        ];
        quad_vao.add_vertex_buffer(&vertices, &[(0, 2), (1, 2)]);

        UiManager {
            shader,
            quad_vao,
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: UiElement) -> usize {
        self.elements.push(element);
        self.elements.len() - 1
    }

    pub fn render(&mut self, window_width: u32, window_height: u32) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        self.shader.bind();
        self.shader.set_vector2f(
            "screenSize",
            &Vector2::new(window_width as f32, window_height as f32)
        );

        for element in &self.elements {
            if !element.is_visible {
                continue;
            }

            self.shader.set_vector2f("position", &element.position);
            self.shader.set_vector2f("size", &element.size);
            self.shader.set_vector4f("color", &element.color);

            if let Some(texture_id) = element.texture_id {
                self.shader.set_bool("hasTexture", true);
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0);
                    gl::BindTexture(gl::TEXTURE_2D, texture_id);
                }
            } else {
                self.shader.set_bool("hasTexture", false);
            }

            self.quad_vao.bind();
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
    }

    pub fn get_element_mut(&mut self, index: usize) -> Option<&mut UiElement> {
        self.elements.get_mut(index)
    }
}

impl UiElement {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, color: Vector4<f32>) -> Self {
        UiElement {
            position,
            size,
            color,
            texture_id: None,
            is_visible: true,
        }
    }

    pub fn with_texture(mut self, texture_id: GLuint) -> Self {
        self.texture_id = Some(texture_id);
        self
    }
} 