use super::*;
use crate::graphics::{Mesh, Renderer, Shader, Texture};
use std::sync::Arc;

pub struct UIRenderer {
    renderer: Arc<Renderer>,
    ui_shader: Shader,
    text_shader: Shader,
    white_texture: Texture,
    font_texture: Texture,
    quad_mesh: Mesh,
    vertices: Vec<UIVertex>,
    indices: Vec<u32>,
}

#[derive(Debug, Clone, Copy)]
struct UIVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

impl UIRenderer {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        // Initialize shaders
        let ui_shader = Self::create_ui_shader();
        let text_shader = Self::create_text_shader();

        // Create white texture for colored quads
        let white_texture = Self::create_white_texture();

        // Create basic quad mesh
        let quad_mesh = Self::create_quad_mesh();

        Self {
            renderer,
            ui_shader,
            text_shader,
            white_texture,
            font_texture: white_texture.clone(), // Placeholder until font is loaded
            quad_mesh,
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1536),
        }
    }

    pub fn begin(&mut self, screen_size: Vec2) {
        self.vertices.clear();
        self.indices.clear();

        // Set up orthographic projection
        let projection =
            glam::Mat4::orthographic_rh(0.0, screen_size.x, screen_size.y, 0.0, -1.0, 1.0);

        self.ui_shader.bind();
        self.ui_shader.set_mat4("projection", &projection);

        self.text_shader.bind();
        self.text_shader.set_mat4("projection", &projection);
    }

    pub fn draw_widget(&mut self, widget: &dyn Widget, style_sheet: &StyleSheet) {
        let rect = widget.rect();
        let state = widget.state();
        let style = widget.style().resolve(&style_sheet.theme);

        // Draw background
        if style.background_color.w > 0.0 {
            self.draw_quad(
                rect.position,
                rect.size,
                style.background_color,
                style.corner_radius,
            );
        }

        // Draw border
        if style.border_width > 0.0 && style.border_color.w > 0.0 {
            self.draw_border(
                rect.position,
                rect.size,
                style.border_color,
                style.border_width,
                style.corner_radius,
            );
        }

        // Draw widget-specific content
        match widget.widget_type() {
            WidgetType::Button => {
                // Add hover/pressed effects
                if state.pressed {
                    self.draw_quad(
                        rect.position,
                        rect.size,
                        Vec4::new(0.0, 0.0, 0.0, 0.2),
                        style.corner_radius,
                    );
                } else if state.hovered {
                    self.draw_quad(
                        rect.position,
                        rect.size,
                        Vec4::new(1.0, 1.0, 1.0, 0.1),
                        style.corner_radius,
                    );
                }
            }
            // Add other widget type rendering...
            _ => {}
        }

        // Draw children
        for child in widget.children() {
            self.draw_widget(&*child.read(), style_sheet);
        }
    }

    pub fn end(&mut self) {
        // Upload vertex and index data
        self.quad_mesh.update_vertices(&self.vertices);
        self.quad_mesh.update_indices(&self.indices);

        // Draw all UI elements
        self.ui_shader.bind();
        self.white_texture.bind(0);
        self.quad_mesh.draw();
    }

    fn draw_quad(&mut self, position: Vec2, size: Vec2, color: Vec4, corner_radius: f32) {
        let base_index = self.vertices.len() as u32;

        // Add vertices
        self.vertices.extend_from_slice(&[
            UIVertex {
                position: [position.x, position.y],
                uv: [0.0, 0.0],
                color: color.into(),
            },
            UIVertex {
                position: [position.x + size.x, position.y],
                uv: [1.0, 0.0],
                color: color.into(),
            },
            UIVertex {
                position: [position.x + size.x, position.y + size.y],
                uv: [1.0, 1.0],
                color: color.into(),
            },
            UIVertex {
                position: [position.x, position.y + size.y],
                uv: [0.0, 1.0],
                color: color.into(),
            },
        ]);

        // Add indices
        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    fn draw_border(
        &mut self,
        position: Vec2,
        size: Vec2,
        color: Vec4,
        width: f32,
        corner_radius: f32,
    ) {
        // Top border
        self.draw_quad(position, Vec2::new(size.x, width), color, corner_radius);

        // Bottom border
        self.draw_quad(
            Vec2::new(position.x, position.y + size.y - width),
            Vec2::new(size.x, width),
            color,
            corner_radius,
        );

        // Left border
        self.draw_quad(
            Vec2::new(position.x, position.y + width),
            Vec2::new(width, size.y - 2.0 * width),
            color,
            corner_radius,
        );

        // Right border
        self.draw_quad(
            Vec2::new(position.x + size.x - width, position.y + width),
            Vec2::new(width, size.y - 2.0 * width),
            color,
            corner_radius,
        );
    }

    fn create_ui_shader() -> Shader {
        // Implementation for creating UI shader
        unimplemented!()
    }

    fn create_text_shader() -> Shader {
        // Implementation for creating text shader
        unimplemented!()
    }

    fn create_white_texture() -> Texture {
        // Implementation for creating white texture
        unimplemented!()
    }

    fn create_quad_mesh() -> Mesh {
        // Implementation for creating quad mesh
        unimplemented!()
    }
}
