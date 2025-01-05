use cgmath::{Point3, Vector3, Matrix4, perspective, Deg, EuclideanSpace};
use nyanko_engine::{
    graphics::{window::Window, gl_wrapper::ShaderProgram, model::Model},
    logger,
};

fn main() {
    logger::init();

    let mut window = Window::new(1280, 720, "3D Model Demo")
        .expect("Failed to create window");
    window.init_gl();

    // Enable depth testing
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // Create a simple cube mesh
    let (vertices, indices) = create_cube();
    let mut model = Model::new(vertices, indices);

    let mut shader = ShaderProgram::new(
        "assets/shaders/basic.vert",
        "assets/shaders/basic.frag"
    );

    // Set up camera
    let camera_pos = Point3::new(0.0, 2.0, 5.0);
    let camera_target = Point3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    
    let view = Matrix4::look_at_rh(camera_pos, camera_target, up);
    let projection = perspective(
        Deg(45.0),
        1280.0 / 720.0,
        0.1,
        100.0
    );

    // Animation variables
    let mut time: f32 = 0.0;

    while !window.should_close() {
        time += 0.016_f32;

        // Animate the model
        model.set_rotation(0.0, time, 0.0);
        model.set_position(
            f32::sin(time) * 2.0,
            f32::cos(time) * 0.5,
            0.0
        );

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Update uniforms
        shader.bind();
        shader.set_matrix4fv_uniform("view", &view);
        shader.set_matrix4fv_uniform("projection", &projection);
        shader.set_vector3f_uniform("lightPos", &Vector3::new(2.0, 2.0, 2.0));
        shader.set_vector3f_uniform("viewPos", &camera_pos.to_vec());
        shader.set_vector3f_uniform("lightColor", &Vector3::new(1.0, 1.0, 1.0));
        shader.set_vector3f_uniform("objectColor", &Vector3::new(1.0, 0.5, 0.7));

        // Draw the model
        model.draw(&mut shader);

        window.update();
    }
}

fn create_cube() -> (Vec<f32>, Vec<u32>) {
    // Cube vertices with positions, normals, and texture coordinates
    let vertices = vec![
        // Front face
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 0.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 0.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,  0.0, 1.0,
        // Back face
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
    ];

    let indices = vec![
        0, 1, 2,  2, 3, 0,  // Front
        1, 5, 6,  6, 2, 1,  // Right
        5, 4, 7,  7, 6, 5,  // Back
        4, 0, 3,  3, 7, 4,  // Left
        3, 2, 6,  6, 7, 3,  // Top
        4, 5, 1,  1, 0, 4,  // Bottom
    ];

    (vertices, indices)
} 