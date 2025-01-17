use glutin::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder};
use nyanko_engine::graphics::Shader;

fn create_context() -> (
    EventLoop<()>,
    glutin::WindowedContext<glutin::PossiblyCurrent>,
) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_visible(false)
        .with_inner_size(PhysicalSize::new(1, 1));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe {
        windowed_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    (el, windowed_context)
}

#[test]
fn test_shader_creation() {
    let (_el, _context) = create_context();

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
