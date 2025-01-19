use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::DisplayApiPreference;
use glutin::prelude::*;
use glutin::surface::SurfaceAttributesBuilder;
use nyanko_engine::graphics::Shader;
use raw_window_handle::RawDisplayHandle;

#[test]
fn test_shader_creation() {
    // Create a headless context
    let display_builder = glutin::display::DisplayApiPreference::Cgl;
    let display = unsafe {
        glutin::display::Display::new(RawDisplayHandle::Cgl(None), display_builder).unwrap()
    };

    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true)
        .build();

    let config = unsafe { display.find_configs(template) }
        .unwrap()
        .next()
        .unwrap();

    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(None))
        .build(None);

    let context = unsafe {
        display
            .create_context(&config, &context_attributes)
            .unwrap()
    };

    let surface_attributes = SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .with_visibility(false)
        .build();

    let _surface = unsafe {
        display
            .create_window_surface(&config, &surface_attributes)
            .unwrap()
    };

    let context = context.make_current_surfaceless().unwrap();

    // Now we can load GL functions and create shaders
    gl::load_with(|s| display.get_proc_address(s) as *const _);

    let vertex_shader = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
            gl_Position = vec4(aPos, 1.0);
        }
    "#;

    let fragment_shader = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    let shader = Shader::new(vertex_shader, fragment_shader);
    assert!(shader.is_ok(), "Shader creation should succeed");
}
