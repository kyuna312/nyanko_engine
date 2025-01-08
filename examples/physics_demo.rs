use nyanko_engine::{
    Engine, EngineConfig,
    components::{Transform, SpriteRenderer, RigidBody2D, BoxCollider2D},
    graphics::{Renderer, Texture, RendererConfig},
};
use glam::{Vec2, Vec3, Quat};
use glutin::{
    self,
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Start,
    Playing,
    Paused,
    GameOver,
}

struct PhysicsDemo {
    engine: Engine,
    renderer: Renderer,
    character_id: u64,
    jump_force: f32,
    move_speed: f32,
    game_state: GameState,
    score: u32,
    lives: u32,
}

impl PhysicsDemo {
    fn new() -> Self {
        let config = EngineConfig::default();
        let mut engine = Engine::new(config.clone());
        
        let renderer_config = RendererConfig {
            window_width: config.window_width,
            window_height: config.window_height,
        };
        let renderer = Renderer::new(&renderer_config);

        // Create character
        let character_id = engine.scene_mut().create_entity();
        let character = engine.scene_mut().get_entity_mut(character_id).unwrap();

        // Add components
        character.add_component(Transform {
            position: Vec3::new(400.0, 300.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(100.0, 200.0, 1.0),
        });

        character.add_component(SpriteRenderer {
            texture: Some(Texture::from_path("assets/character.png").unwrap()),
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
        });

        character.add_component(RigidBody2D {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            use_gravity: true,
        });

        character.add_component(BoxCollider2D {
            size: Vec2::new(100.0, 200.0),
            offset: Vec2::ZERO,
            is_trigger: false,
        });

        // Create ground
        let ground_id = engine.scene_mut().create_entity();
        let ground = engine.scene_mut().get_entity_mut(ground_id).unwrap();

        ground.add_component(Transform {
            position: Vec3::new(400.0, 50.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(800.0, 20.0, 1.0),
        });

        ground.add_component(SpriteRenderer {
            texture: None,
            color: [0.2, 0.8, 0.2, 1.0],
            flip_x: false,
            flip_y: false,
        });

        ground.add_component(BoxCollider2D {
            size: Vec2::new(800.0, 20.0),
            offset: Vec2::ZERO,
            is_trigger: false,
        });

        Self {
            engine,
            renderer,
            character_id,
            jump_force: 400.0,
            move_speed: 200.0,
            game_state: GameState::Start,
            score: 0,
            lives: 3,
        }
    }

    fn handle_input(&mut self, input: &KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            match (self.game_state, keycode, input.state) {
                // Start screen controls
                (GameState::Start, VirtualKeyCode::Return, ElementState::Pressed) => {
                    self.game_state = GameState::Playing;
                },

                // Playing state controls
                (GameState::Playing, VirtualKeyCode::P, ElementState::Pressed) => {
                    self.game_state = GameState::Paused;
                },
                (GameState::Playing, VirtualKeyCode::Space, ElementState::Pressed) => {
                    if let Some(character) = self.engine.scene_mut().get_entity_mut(self.character_id) {
                        if let Some(rb) = character.get_component_mut::<RigidBody2D>() {
                            rb.velocity.y = self.jump_force;
                        }
                    }
                },
                (GameState::Playing, VirtualKeyCode::Left, state) => {
                    if let Some(character) = self.engine.scene_mut().get_entity_mut(self.character_id) {
                        if let Some(rb) = character.get_component_mut::<RigidBody2D>() {
                            rb.velocity.x = if state == ElementState::Pressed {
                                -self.move_speed
                            } else {
                                0.0
                            };
                        }
                    }
                },
                (GameState::Playing, VirtualKeyCode::Right, state) => {
                    if let Some(character) = self.engine.scene_mut().get_entity_mut(self.character_id) {
                        if let Some(rb) = character.get_component_mut::<RigidBody2D>() {
                            rb.velocity.x = if state == ElementState::Pressed {
                                self.move_speed
                            } else {
                                0.0
                            };
                        }
                    }
                },

                // Paused state controls
                (GameState::Paused, VirtualKeyCode::P, ElementState::Pressed) => {
                    self.game_state = GameState::Playing;
                },
                (GameState::Paused, VirtualKeyCode::Q, ElementState::Pressed) => {
                    self.game_state = GameState::GameOver;
                },

                // Game Over state controls
                (GameState::GameOver, VirtualKeyCode::R, ElementState::Pressed) => {
                    self.reset_game();
                },
                _ => {}
            }
        }
    }

    fn reset_game(&mut self) {
        self.score = 0;
        self.lives = 3;
        self.game_state = GameState::Start;

        // Reset character position
        if let Some(character) = self.engine.scene_mut().get_entity_mut(self.character_id) {
            if let Some(transform) = character.get_component_mut::<Transform>() {
                transform.position = Vec3::new(400.0, 300.0, 0.0);
            }
            if let Some(rb) = character.get_component_mut::<RigidBody2D>() {
                rb.velocity = Vec2::ZERO;
                rb.acceleration = Vec2::ZERO;
            }
        }
    }

    fn update(&mut self) {
        match self.game_state {
            GameState::Playing => {
                self.engine.update();
                
                // Check if character fell off screen
                if let Some(character) = self.engine.scene().get_entity(self.character_id) {
                    if let Some(transform) = character.get_component::<Transform>() {
                        if transform.position.y < -100.0 {
                            self.lives -= 1;
                            if self.lives == 0 {
                                self.game_state = GameState::GameOver;
                            } else {
                                // Reset character position
                                if let Some(character) = self.engine.scene_mut().get_entity_mut(self.character_id) {
                                    if let Some(transform) = character.get_component_mut::<Transform>() {
                                        transform.position = Vec3::new(400.0, 300.0, 0.0);
                                    }
                                    if let Some(rb) = character.get_component_mut::<RigidBody2D>() {
                                        rb.velocity = Vec2::ZERO;
                                        rb.acceleration = Vec2::ZERO;
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn render(&mut self) {
        self.renderer.begin_frame();

        match self.game_state {
            GameState::Start => {
                // Render start screen
                // TODO: Add text rendering for "Press Enter to Start"
                self.engine.scene().render(&mut self.renderer);
            },
            GameState::Playing => {
                // Render game
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for score and lives
            },
            GameState::Paused => {
                // Render paused game state
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for "PAUSED - P to resume, Q to quit"
            },
            GameState::GameOver => {
                // Render game over screen
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for "GAME OVER - R to restart"
            },
        }

        self.renderer.end_frame();
    }
}

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Nyanko Engine - Physics Demo")
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));
    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    let mut demo = PhysicsDemo::new();

    el.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    demo.handle_input(&input);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                demo.update();
                demo.render();
                windowed_context.swap_buffers().unwrap();
            }
            _ => {}
        }
    });
} 