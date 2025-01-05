use log::info;
use nyanko_engine::graphics::window::Window;
use nyanko_engine::graphics::gl_wrapper::{Vao, ShaderProgram};
use nyanko_engine::logger;
use cgmath::{Matrix4, Vector3};
use std::f32::consts::PI;

fn main() {
    logger::init();
    info!("Starting Nyanko Engine...");

    let mut window = Window::new(800, 600, "Nyanko Engine Demo")
        .expect("Failed to create window");
    
    window.init_gl();

    // Create vertex data for a triangle
    let vertices: Vec<f32> = vec![
        // positions      // colors
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,  // bottom left - red
         0.5, -0.5, 0.0, 0.0, 1.0, 0.0,  // bottom right - green
         0.0,  0.5, 0.0, 0.0, 0.0, 1.0,  // top - blue
    ];

    let mut vao = Vao::new();
    vao.add_vertex_buffer(&vertices, &[
        (0, 3), // position attribute
        (1, 3), // color attribute
    ]);

    let mut shader = ShaderProgram::new(
        "assets/shaders/basic.vert",
        "assets/shaders/basic.frag"
    );

    // Variables for animation (explicitly typed as f32)
    let mut time: f32 = 0.0;
    let movement_speed: f32 = 1.0;
    let movement_radius: f32 = 0.5;

    while !window.should_close() {
        // Update time
        time += 0.016_f32;

        // Calculate position with explicit f32 methods
        let x = movement_radius * f32::cos(time * movement_speed);
        let y = movement_radius * f32::sin(time * movement_speed);

        // Create transformation matrix
        let transform = Matrix4::from_translation(Vector3::new(x, y, 0.0));

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw the triangle with transformation
        shader.bind();
        shader.set_matrix4fv_uniform("transform", &transform);
        vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        Vao::unbind();
        ShaderProgram::unbind();

        window.update();
    }

    info!("Shutting down Nyanko Engine");
} 