use cgmath::{Point3, Vector3, Matrix4, perspective, Deg, EuclideanSpace};
use nyanko_engine::{
    graphics::{window::Window, gl_wrapper::ShaderProgram, model_loader::load_model},
    logger,
};
use log::{info, error};

fn main() {
    logger::init();
    info!("Starting Miku Demo...");

    let mut window = Window::new(1280, 720, "Hatsune Miku Demo")
        .expect("Failed to create window");
    window.init_gl();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
    }

    // Load Miku model
    let mut miku = match load_model("assets/models/miku/miku.obj") {
        Ok(model) => model,
        Err(e) => {
            error!("Failed to load model: {}", e);
            std::process::exit(1);
        }
    };

    let mut shader = ShaderProgram::new(
        "assets/shaders/miku.vert",
        "assets/shaders/miku.frag"
    );

    // Set up camera
    let camera_pos = Point3::new(0.0, 1.0, 3.0);
    let camera_target = Point3::new(0.0, 1.0, 0.0);
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

        // Animate Miku
        miku.set_rotation(0.0, time * 0.5, 0.0); // Gentle rotation
        miku.set_scale(0.1, 0.1, 0.1); // Adjust scale to fit view

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Update uniforms
        shader.bind();
        shader.set_matrix4fv_uniform("view", &view);
        shader.set_matrix4fv_uniform("projection", &projection);
        shader.set_vector3f_uniform("lightPos", &Vector3::new(2.0, 4.0, 2.0));
        shader.set_vector3f_uniform("viewPos", &camera_pos.to_vec());
        shader.set_vector3f_uniform("lightColor", &Vector3::new(1.0, 1.0, 1.0));
        shader.set_vector3f_uniform("objectColor", &Vector3::new(1.0, 1.0, 1.0));

        // Draw Miku
        miku.draw(&mut shader);

        window.update();
    }
} 