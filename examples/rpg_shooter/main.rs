use nyanko_engine::{
    Engine, EngineConfig,
    components::{Transform, SpriteRenderer, RigidBody2D, BoxCollider2D, CharacterStats, Weapon},
    graphics::{Renderer, Texture, RendererConfig},
    core::GameState,
};
use glam::{Vec2, Vec3, Quat};
use glutin::{
    self,
    event::{Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState, MouseButton},
};
use std::time::{Instant, Duration};

struct Player {
    entity_id: u64,
    move_speed: f32,
    stats: CharacterStats,
    weapon: Weapon,
}

impl Player {
    fn new(engine: &mut Engine) -> Self {
        let entity_id = engine.scene_mut().create_entity();
        let player = engine.scene_mut().get_entity_mut(entity_id).unwrap();

        // Add components
        player.add_component(Transform {
            position: Vec3::new(400.0, 300.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(50.0, 50.0, 1.0),
        });

        player.add_component(SpriteRenderer {
            texture: None,
            color: [0.0, 1.0, 0.0, 1.0],
            flip_x: false,
            flip_y: false,
        });

        player.add_component(RigidBody2D {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            use_gravity: false,
        });

        player.add_component(BoxCollider2D {
            size: Vec2::new(50.0, 50.0),
            offset: Vec2::ZERO,
            is_trigger: false,
        });

        Self {
            entity_id,
            move_speed: 200.0,
            stats: CharacterStats::new(1),
            weapon: Weapon::new(20.0, 5.0, 500.0),
        }
    }

    fn update(&mut self, engine: &mut Engine, dt: f32) {
        // Get input state first
        let w_pressed = engine.input().is_key_down(VirtualKeyCode::W as usize);
        let s_pressed = engine.input().is_key_down(VirtualKeyCode::S as usize);
        let a_pressed = engine.input().is_key_down(VirtualKeyCode::A as usize);
        let d_pressed = engine.input().is_key_down(VirtualKeyCode::D as usize);

        // Update weapon reload
        self.weapon.update_reload(Duration::from_secs_f32(dt));

        // Then update entity
        if let Some(player) = engine.scene_mut().get_entity_mut(self.entity_id) {
            if let Some(rb) = player.get_component_mut::<RigidBody2D>() {
                // Reset velocity
                rb.velocity = Vec2::ZERO;

                // Update movement based on cached input
                if w_pressed {
                    rb.velocity.y = self.move_speed;
                }
                if s_pressed {
                    rb.velocity.y = -self.move_speed;
                }
                if a_pressed {
                    rb.velocity.x = -self.move_speed;
                }
                if d_pressed {
                    rb.velocity.x = self.move_speed;
                }
            }
        }
    }

    fn shoot(&mut self, engine: &mut Engine, direction: Vec2) {
        let current_time = engine.time().total_time;
        if self.weapon.can_shoot(current_time) {
            self.weapon.shoot(current_time);

            let player_pos = if let Some(player) = engine.scene().get_entity(self.entity_id) {
                if let Some(transform) = player.get_component::<Transform>() {
                    transform.position
                } else {
                    Vec3::ZERO
                }
            } else {
                Vec3::ZERO
            };

            let bullet_id = engine.scene_mut().create_entity();
            let bullet = engine.scene_mut().get_entity_mut(bullet_id).unwrap();

            bullet.add_component(Transform {
                position: player_pos,
                rotation: Quat::IDENTITY,
                scale: Vec3::new(10.0, 5.0, 1.0),
            });

            bullet.add_component(SpriteRenderer {
                texture: None,
                color: [1.0, 1.0, 0.0, 1.0],
                flip_x: false,
                flip_y: false,
            });

            bullet.add_component(RigidBody2D {
                velocity: direction.normalize() * self.weapon.bullet_speed,
                acceleration: Vec2::ZERO,
                mass: 1.0,
                use_gravity: false,
            });

            bullet.add_component(BoxCollider2D {
                size: self.weapon.bullet_size,
                offset: Vec2::ZERO,
                is_trigger: true,
            });
        }
    }
}

struct RpgShooterDemo {
    engine: Engine,
    renderer: Renderer,
    player: Player,
    game_state: GameState,
    start_time: Instant,
}

impl RpgShooterDemo {
    fn new() -> Self {
        let config = EngineConfig::default();
        let mut engine = Engine::new(config.clone());
        
        let renderer_config = RendererConfig {
            window_width: config.window_width,
            window_height: config.window_height,
        };
        let renderer = Renderer::new(&renderer_config);

        let player = Player::new(&mut engine);

        Self {
            engine,
            renderer,
            player,
            game_state: GameState::Start,
            start_time: Instant::now(),
        }
    }

    fn handle_input(&mut self, input: &KeyboardInput) {
        match self.game_state {
            GameState::Start => {
                if let Some(VirtualKeyCode::Return) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        self.game_state = GameState::Playing;
                    }
                }
            },
            GameState::Playing => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        self.game_state = GameState::Paused;
                    }
                }
            },
            GameState::Paused => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    if input.state == ElementState::Pressed {
                        self.game_state = GameState::Playing;
                    }
                }
            },
            _ => {}
        }
    }

    fn handle_mouse(&mut self, button: MouseButton, state: ElementState) {
        if self.game_state == GameState::Playing && button == MouseButton::Left && state == ElementState::Pressed {
            let mouse_pos = self.engine.input().mouse_position;
            let player_pos = if let Some(player) = self.engine.scene().get_entity(self.player.entity_id) {
                if let Some(transform) = player.get_component::<Transform>() {
                    Vec2::new(transform.position.x, transform.position.y)
                } else {
                    Vec2::ZERO
                }
            } else {
                Vec2::ZERO
            };

            let direction = mouse_pos - player_pos;
            self.player.shoot(&mut self.engine, direction);
        }
    }

    fn update(&mut self) {
        if self.game_state == GameState::Playing {
            let dt = self.engine.time().delta_time;
            self.player.update(&mut self.engine, dt);
            self.engine.update();
        }
    }

    fn render(&mut self) {
        self.renderer.begin_frame();

        match self.game_state {
            GameState::Start => {
                // Render start screen
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for "Press Enter to Start"
            },
            GameState::Playing => {
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add UI rendering for player stats and ammo
            },
            GameState::Paused => {
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for "PAUSED"
            },
            GameState::GameOver => {
                self.engine.scene().render(&mut self.renderer);
                // TODO: Add text rendering for "GAME OVER"
            },
        }

        self.renderer.end_frame();
    }
}

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Nyanko Engine - RPG Shooter Demo")
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 600));
    let windowed_context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

    let mut demo = RpgShooterDemo::new();

    el.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                WindowEvent::KeyboardInput { input, .. } => {
                    demo.handle_input(&input);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    demo.handle_mouse(button, state);
                },
                _ => {}
            },
            Event::MainEventsCleared => {
                demo.update();
                demo.render();
                windowed_context.swap_buffers().unwrap();
            },
            _ => {}
        }
    });
} 