use nyanko_engine::{
    graphics::{
        window::Window,
        renderer::Renderer,
        bloom::{BloomProcessor, BloomSettings},
        post_process::{PostProcessor, PostProcessParams},
    },
    physics::{PhysicsWorld, PhysicsObject},
};
use cgmath::{Vector2, Vector3};
use std::path::Path;

fn main() {
    // Create window
    let mut window = Window::new(1920, 1080, "Anime Character Demo")
        .expect("Failed to create window");
    window.init_gl();

    // Verify texture path and load
    let texture_path = "character.png";
    if !Path::new(texture_path).exists() {
        panic!("Character image not found at: {}", texture_path);
    }
    println!("Loading character from: {}", texture_path);
                            
    // Initialize renderer with proper scale and position
    let renderer = Renderer::new(
        ("assets/shaders/sprite.vert", "assets/shaders/sprite.frag"),
        texture_path,
        1920,
        1080
    );

    let mut bloom_processor = BloomProcessor::new(1920, 1080);
    let mut post_processor = PostProcessor::new(1920, 1080);
    
    let bloom_settings = BloomSettings {
        threshold: 0.5,
        softness: 0.8,
        intensity: 1.8,
        ..BloomSettings::default()
    };

    let post_process_params = PostProcessParams {
        enable_bloom: true,
        enable_chromatic: true,
        enable_vignette: true,
        bloom_intensity: 1.2,
        chromatic_strength: 0.001,
        vignette_intensity: 0.15,
        vignette_roundness: 1.0,
        vignette_smoothness: 0.7,
    };

    let mut physics_world = PhysicsWorld::new();

    // Adjust character size and position
    let character = PhysicsObject::new(
        Vector2::new(0.0, -0.2),  // Slightly lower position
        Vector2::new(0.8, 1.4),   // Adjusted size for better proportions
        1.0,
    );
    let character_id = physics_world.add_object(character);

    // Main loop
    while !window.should_close() {
        physics_world.update(1.0/60.0);
        post_processor.begin_render();

        // Clear with dark background
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Render character with adjusted scale and position
        if let Some(character) = physics_world.get_object(character_id) {
            renderer.render(
                Vector3::new(character.position.x, character.position.y, -2.0), // Moved back
                2.2  // Increased scale
            );
        }

        // Apply post-processing
        let screen_texture = post_processor.get_screen_texture();
        let bloomed_texture = bloom_processor.process(screen_texture, &bloom_settings);
        post_processor.end_render(&post_process_params, bloomed_texture);

        window.update();
    }
} 