use cgmath::{Point3, Vector3, Matrix4, perspective, Deg, EuclideanSpace};
use nyanko_engine::{
    graphics::{
        window::Window,
        gl_wrapper::ShaderProgram,
        model::{Material},
        model_loader::load_model,
        texture::Texture,
    },
    logger,
};
use log::info;

fn main() {
    logger::init();
    info!("Starting Character Demo...");

    let mut window = Window::new(1280, 720, "Character Demo")
        .expect("Failed to create window");
    window.init_gl();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // Load textures first
    let diffuse_texture = Texture::new("assets/models/character/textures/diffuse.png")
        .expect("Failed to load diffuse texture");
    let specular_texture = Texture::new("assets/models/character/textures/specular.png")
        .expect("Failed to load specular texture");

    // Create material with loaded textures
    let material = Material {
        diffuse_texture: Some(diffuse_texture),
        specular_texture: Some(specular_texture),
        shininess: 32.0,
    };

    // Load character model with material
    let mut character = load_model("assets/models/character/model.obj")
        .expect("Failed to load character model");

    let mut shader = ShaderProgram::new(
        "assets/shaders/character.vert",
        "assets/shaders/character.frag"
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

    let mut time: f32 = 0.0;

    while !window.should_close() {
        time += 0.016_f32;

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader.bind();
        shader.set_matrix4fv_uniform("view", &view);
        shader.set_matrix4fv_uniform("projection", &projection);
        shader.set_vector3f_uniform("lightPos", &Vector3::new(2.0, 4.0, 2.0));
        shader.set_vector3f_uniform("viewPos", &camera_pos.to_vec());
        shader.set_vector3f_uniform("lightColor", &Vector3::new(1.0, 1.0, 1.0));

        character.set_rotation(0.0, time * 0.5, 0.0);
        character.draw(&mut shader);

        window.update();
    }
} 