use nyanko_engine::graphics::{
    window::Window,
    texture::Texture,
    bloom::{BloomProcessor, BloomSettings},
    post_process::{PostProcessor, PostProcessParams},
    gl_wrapper::{ShaderProgram, Vao},
};
use cgmath::{Deg, perspective, Matrix4, vec3, Rad};

fn main() {
    // Create window
    let mut window = Window::new(1280, 720, "Character Demo")
        .expect("Failed to create window");
    window.init_gl();

    // Enable OpenGL features
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }

    // Create shader
    let mut shader = ShaderProgram::new(
        "assets/shaders/sprite.vert",
        "assets/shaders/sprite.frag",
    );

    // Load texture
    let image_path = "assets/models/character/textures/character.jpg";
    println!("Loading texture: {}", image_path);
    let texture = Texture::new(image_path)
        .expect("Failed to load texture");

    // Create VAO with a quad
    let mut vao = Vao::new();
    let vertices: [f32; 20] = [
        // positions     // texture coords
         0.5,  1.0, 0.0,  1.0, 1.0,  // Top right
        -0.5,  1.0, 0.0,  0.0, 1.0,  // Top left
        -0.5, -1.0, 0.0,  0.0, 0.0,  // Bottom left
         0.5, -1.0, 0.0,  1.0, 0.0,  // Bottom right
    ];
    vao.add_vertex_buffer(&vertices, &[(0, 3), (1, 2)]);

    // Create post-processing
    let mut post_processor = PostProcessor::new(1280, 720);
    let post_process_params = PostProcessParams {
        enable_bloom: true,
        enable_chromatic: true,
        enable_vignette: true,
        bloom_intensity: 0.5,
        chromatic_strength: 0.003,
        vignette_intensity: 0.4,
        vignette_roundness: 1.0,
        vignette_smoothness: 0.3,
    };

    // Create bloom
    let mut bloom_processor = BloomProcessor::new(1280, 720);
    let mut bloom_settings = BloomSettings::default();
    bloom_settings.threshold = 0.7;
    bloom_settings.softness = 0.5;
    bloom_settings.intensity = 0.6;

    // Set up projection and model matrices
    let projection = perspective(
        Deg(45.0),
        1280.0 / 720.0,
        0.1,
        100.0
    );
    
    let model = Matrix4::from_translation(vec3(0.0, 0.0, -2.5)) * 
                Matrix4::from_scale(0.8) * 
                Matrix4::from_angle_z(Rad(0.0));

    println!("Starting render loop...");

    // Main loop
    let mut time = 0.0;
    while !window.should_close() {
        time += 0.016; // Approximate delta time

        // Begin post-processing
        post_processor.begin_render();

        // Clear screen
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Render character
        shader.bind();
        shader.set_matrix4fv("projection", &projection);
        shader.set_matrix4fv("model", &model);
        texture.bind();
        vao.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
        }

        // Apply post-processing effects
        let screen_texture = post_processor.get_screen_texture();
        let bloomed_texture = bloom_processor.process(screen_texture, &bloom_settings);
        post_processor.end_render(&post_process_params, bloomed_texture);

        // Update window
        window.update();
    }
}