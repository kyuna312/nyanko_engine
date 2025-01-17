use crate::physics::{PhysicsWorld, RigidBody};
use crate::renderer::{Renderer, Sprite};
use glam::Vec2;

pub struct PhysicsDemo {
    physics_world: PhysicsWorld,
    boxes: Vec<u32>, // Store body IDs
    ground_id: u32,
    ball_id: u32,
    elapsed_time: f32,
}

impl PhysicsDemo {
    pub fn new() -> Self {
        let mut physics_world = PhysicsWorld::new(Vec2::new(0.0, -9.81)); // Normal gravity

        // Create ground
        let ground = RigidBody::new(
            Vec2::new(0.0, -200.0), // position
            f32::INFINITY,          // infinite mass (immovable)
            Vec2::new(800.0, 20.0), // size
        );
        let ground_id = physics_world.add_body(ground);

        // Create bouncing ball
        let ball = RigidBody::new(
            Vec2::new(0.0, 200.0), // start from top
            1.0,                   // mass
            Vec2::new(20.0, 20.0), // size
        );
        let ball_id = physics_world.add_body(ball);

        // Create stack of boxes
        let mut boxes = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                let box_body = RigidBody::new(
                    Vec2::new(-100.0 + (j as f32 * 45.0), -150.0 + (i as f32 * 45.0)),
                    1.0,                   // mass
                    Vec2::new(40.0, 40.0), // size
                );
                boxes.push(physics_world.add_body(box_body));
            }
        }

        Self {
            physics_world,
            boxes,
            ground_id,
            ball_id,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;

        // Add some interactive forces
        if let Some(ball) = self.physics_world.get_body_mut(self.ball_id) {
            // Add sinusoidal horizontal force to the ball
            let force = Vec2::new(500.0 * (self.elapsed_time * 2.0).sin(), 0.0);
            ball.apply_force(force);
        }

        // Update physics
        self.physics_world.step(dt);

        // Keep objects in bounds
        self.keep_in_bounds();
    }

    fn keep_in_bounds(&mut self) {
        let bounds = Vec2::new(400.0, 300.0); // Screen bounds

        for body_id in self.boxes.iter().chain(std::iter::once(&self.ball_id)) {
            if let Some(body) = self.physics_world.get_body_mut(*body_id) {
                // Bounce off screen edges
                if body.position.x < -bounds.x {
                    body.position.x = -bounds.x;
                    body.velocity.x = body.velocity.x.abs();
                } else if body.position.x > bounds.x {
                    body.position.x = bounds.x;
                    body.velocity.x = -body.velocity.x.abs();
                }

                if body.position.y > bounds.y {
                    body.position.y = bounds.y;
                    body.velocity.y = -body.velocity.y.abs();
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        // Render ground
        if let Some(ground) = self.physics_world.get_body(self.ground_id) {
            renderer.draw_rectangle(
                ground.position,
                ground.size,
                [0.2, 0.2, 0.2, 1.0], // Dark gray
            );
        }

        // Render boxes
        for &box_id in &self.boxes {
            if let Some(box_body) = self.physics_world.get_body(box_id) {
                renderer.draw_rectangle(
                    box_body.position,
                    box_body.size,
                    [0.8, 0.6, 0.2, 1.0], // Golden
                );
            }
        }

        // Render ball
        if let Some(ball) = self.physics_world.get_body(self.ball_id) {
            renderer.draw_circle(
                ball.position,
                ball.size.x / 2.0,
                [1.0, 0.2, 0.2, 1.0], // Red
            );
        }
    }

    pub fn add_explosion(&mut self, position: Vec2, force: f32) {
        // Apply explosive force to nearby objects
        for &box_id in &self.boxes {
            if let Some(box_body) = self.physics_world.get_body_mut(box_id) {
                let direction = box_body.position - position;
                let distance = direction.length();
                if distance < 200.0 {
                    let normalized_dir = direction.normalize();
                    let explosion_force = normalized_dir * force * (1.0 - distance / 200.0);
                    box_body.apply_force(explosion_force);
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.physics_world = PhysicsWorld::new(Vec2::new(0.0, -9.81));
        // Recreate the scene...
        // (Similar to new() implementation)
    }
}

// Usage example in main.rs:
fn main() {
    let mut demo = PhysicsDemo::new();
    let mut renderer = Renderer::new();

    // Game loop
    let mut last_time = std::time::Instant::now();
    loop {
        let current_time = std::time::Instant::now();
        let dt = (current_time - last_time).as_secs_f32();
        last_time = current_time;

        // Update physics
        demo.update(dt);

        // Handle input (example)
        if input.is_key_pressed(Key::Space) {
            demo.add_explosion(Vec2::new(0.0, 0.0), 1000.0);
        }
        if input.is_key_pressed(Key::R) {
            demo.reset();
        }

        // Render
        renderer.begin_frame();
        demo.render(&mut renderer);
        renderer.end_frame();
    }
}
