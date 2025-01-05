use nyanko_engine::{
    graphics::{window::Window, renderer::Renderer},
    physics::{PhysicsWorld, PhysicsObject},
};
use cgmath::{Vector2, Vector3};
use std::{time::{Instant, Duration}, path::Path, env};

struct GameState {
    physics_world: PhysicsWorld,
    character_id: usize,
    last_update: Instant,
    fixed_timestep: Duration,
    expansion_start: Option<Instant>,
}

impl GameState {
    fn new() -> Self {
        let mut physics_world = PhysicsWorld::new();
        
        // Create character with centered position and no initial movement
        let mut character = PhysicsObject::new(
            Vector2::new(0.0, 0.0),  // Center position
            Vector2::new(1.0, 1.0),  // Unit size
            1.0,
        );
        character.velocity.y = 3.0;
        let character_id = physics_world.add_object(character);

        Self {
            physics_world,
            character_id,
            last_update: Instant::now(),
            fixed_timestep: Duration::from_secs_f32(1.0 / 60.0),
            expansion_start: None,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now - self.last_update;
        self.last_update = now;

        self.physics_world.update(dt.as_secs_f32());
    }
}

struct GameRenderer {
    renderer: Renderer,
}

impl GameRenderer {
    fn new(width: u32, height: u32, asset_manager: &AssetManager) -> Self {
        let vertex_shader = asset_manager.get_asset_path("shaders/sprite.vert");
        let fragment_shader = asset_manager.get_asset_path("shaders/sprite.frag");
        let character_texture = asset_manager.get_asset_path("textures/character.png");

        Self {
            renderer: Renderer::new(
                width,
                height,
                (&vertex_shader, &fragment_shader),
                &character_texture
            ),
        }
    }

    fn render(&mut self, game_state: &GameState) {
        if let Some(character) = game_state.physics_world.get_object(game_state.character_id) {
            let mut base_scale = 1.8; // Adjusted base scale
            
            if character.should_expand {
                if let Some(start_time) = game_state.expansion_start {
                    let elapsed = start_time.elapsed().as_secs_f32();
                    // Smooth transition to full screen
                    base_scale = 1.8 + (elapsed * 1.2).min(1.2); // Max scale of 3.0
                }
            }

            // Use the same scale for both dimensions
            let scale = Vector2::new(base_scale, base_scale);
            
            // Center position
            let position = Vector3::new(
                0.0,
                character.position.y,
                0.0
            );

            self.renderer.render_scaled(position, scale);
        }
    }
}

struct AssetManager {
    base_path: String,
}

impl AssetManager {
    fn new() -> Self {
        // Use CARGO_MANIFEST_DIR to get project root directory
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .expect("Failed to get CARGO_MANIFEST_DIR");
        
        Self {
            base_path: Path::new(&manifest_dir)
                .join("assets")
                .to_string_lossy()
                .to_string(),
        }
    }

    fn get_asset_path(&self, asset_name: &str) -> String {
        let path = Path::new(&self.base_path)
            .join(asset_name)
            .to_string_lossy()
            .to_string();
        
        println!("Resolving asset path: {}", path);
        
        if !Path::new(&path).exists() {
            panic!("Asset not found: {}", path);
        }
        
        path
    }
}

fn main() {
    // Create assets directory if it doesn't exist
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to get CARGO_MANIFEST_DIR");
    let assets_dir = Path::new(&manifest_dir).join("assets");
    let textures_dir = assets_dir.join("textures");
    let shaders_dir = assets_dir.join("shaders");

    std::fs::create_dir_all(&textures_dir)
        .expect("Failed to create textures directory");
    std::fs::create_dir_all(&shaders_dir)
        .expect("Failed to create shaders directory");

    // Print the expected path for the character texture
    println!("Expected character texture path: {}", 
        textures_dir.join("character.png").display());

    // Update window size to maintain aspect ratio
    let window_width = 1920;
    let window_height = 1080;
    
    let mut window = Window::new(window_width, window_height, "Anime Character Demo")
        .expect("Failed to create window");
    window.init_gl();

    let asset_manager = AssetManager::new();
    let mut game_state = GameState::new();
    let mut game_renderer = GameRenderer::new(window_width, window_height, &asset_manager);

    // Main game loop
    while !window.should_close() {
        // Update game state
        game_state.update();

        // Clear screen
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 1.0); // White background
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Render game
        game_renderer.render(&game_state);

        // Update window
        window.update();
    }
} 