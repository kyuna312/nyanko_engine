use crate::graphics::Shader;

impl UIRenderer {
    fn create_ui_shader() -> Shader {
        let vertex_src = r#"
            #version 450

            layout(location = 0) in vec2 a_position;
            layout(location = 1) in vec2 a_uv;
            layout(location = 2) in vec4 a_color;

            layout(location = 0) out vec2 v_uv;
            layout(location = 1) out vec4 v_color;

            layout(set = 0, binding = 0) uniform Uniforms {
                mat4 projection;
            };

            void main() {
                v_uv = a_uv;
                v_color = a_color;
                gl_Position = projection * vec4(a_position, 0.0, 1.0);
            }
        "#;

        let fragment_src = r#"
            #version 450

            layout(location = 0) in vec2 v_uv;
            layout(location = 1) in vec4 v_color;

            layout(location = 0) out vec4 o_color;

            layout(set = 1, binding = 0) uniform texture2D u_texture;
            layout(set = 1, binding = 1) uniform sampler u_sampler;

            void main() {
                vec4 tex_color = texture(sampler2D(u_texture, u_sampler), v_uv);
                o_color = tex_color * v_color;
            }
        "#;

        Shader::from_source(vertex_src, fragment_src)
    }

    fn create_text_shader() -> Shader {
        let vertex_src = r#"
            #version 450

            layout(location = 0) in vec2 a_position;
            layout(location = 1) in vec2 a_uv;
            layout(location = 2) in vec4 a_color;

            layout(location = 0) out vec2 v_uv;
            layout(location = 1) out vec4 v_color;

            layout(set = 0, binding = 0) uniform Uniforms {
                mat4 projection;
            };

            void main() {
                v_uv = a_uv;
                v_color = a_color;
                gl_Position = projection * vec4(a_position, 0.0, 1.0);
            }
        "#;

        let fragment_src = r#"
            #version 450

            layout(location = 0) in vec2 v_uv;
            layout(location = 1) in vec4 v_color;

            layout(location = 0) out vec4 o_color;

            layout(set = 1, binding = 0) uniform texture2D u_font_texture;
            layout(set = 1, binding = 1) uniform sampler u_sampler;

            void main() {
                float alpha = texture(sampler2D(u_font_texture, u_sampler), v_uv).r;
                o_color = v_color * vec4(1.0, 1.0, 1.0, alpha);
            }
        "#;

        Shader::from_source(vertex_src, fragment_src)
    }

    fn create_white_texture() -> Texture {
        let white_pixels = vec![255u8; 4]; // Single white pixel (RGBA)
        Texture::from_rgba8(&white_pixels, 1, 1)
    }

    fn create_quad_mesh() -> Mesh {
        let vertices = vec![
            UIVertex {
                position: [-0.5, -0.5],
                uv: [0.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            UIVertex {
                position: [0.5, -0.5],
                uv: [1.0, 0.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            UIVertex {
                position: [0.5, 0.5],
                uv: [1.0, 1.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
            UIVertex {
                position: [-0.5, 0.5],
                uv: [0.0, 1.0],
                color: [1.0, 1.0, 1.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2, 0, 2, 3];

        Mesh::new(
            &vertices,
            &indices,
            &[
                // position
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                // uv
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 8,
                    shader_location: 1,
                },
                // color
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 16,
                    shader_location: 2,
                },
            ],
        )
    }
}
