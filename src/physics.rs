use glam::Vec2;
use std::collections::HashMap;

pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub size: Vec2,
    id: u32,
}

impl RigidBody {
    pub fn new(position: Vec2, mass: f32, size: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass,
            size,
            id: 0,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn update(&mut self, dt: f32) {
        self.velocity += self.acceleration * dt;
        self.position += self.velocity * dt;
        self.acceleration = Vec2::ZERO;
    }
}

pub struct PhysicsWorld {
    bodies: HashMap<u32, RigidBody>,
    gravity: Vec2,
    time_step: f32,
}

impl PhysicsWorld {
    pub fn new(gravity: Vec2) -> Self {
        Self {
            bodies: HashMap::new(),
            gravity,
            time_step: 1.0 / 60.0,
        }
    }

    pub fn add_body(&mut self, mut body: RigidBody) -> u32 {
        let id = (self.bodies.len() as u32) + 1;
        body.id = id;
        self.bodies.insert(id, body);
        id
    }

    pub fn get_body(&self, id: u32) -> Option<&RigidBody> {
        self.bodies.get(&id)
    }

    pub fn get_body_mut(&mut self, id: u32) -> Option<&mut RigidBody> {
        self.bodies.get_mut(&id)
    }

    pub fn step(&mut self, dt: f32) {
        let step = self.time_step.min(dt); // Use time_step as max step size
        
        // Apply gravity to all bodies
        for body in self.bodies.values_mut() {
            body.apply_force(self.gravity * body.mass);
        }

        // Check collisions
        self.check_collisions();

        // Update all bodies
        for body in self.bodies.values_mut() {
            body.update(step);
        }
    }

    fn check_collisions(&mut self) {
        let bodies: Vec<(u32, Vec2, Vec2)> = self.bodies
            .iter()
            .map(|(&id, body)| (id, body.position, body.size))
            .collect();

        for i in 0..bodies.len() {
            for j in (i + 1)..bodies.len() {
                let (id1, pos1, size1) = bodies[i];
                let (id2, pos2, size2) = bodies[j];

                // AABB collision check
                if pos1.x < pos2.x + size2.x &&
                   pos1.x + size1.x > pos2.x &&
                   pos1.y < pos2.y + size2.y &&
                   pos1.y + size1.y > pos2.y {
                    self.resolve_collision(id1, id2);
                }
            }
        }
    }

    fn resolve_collision(&mut self, id1: u32, id2: u32) {
        // Get the bodies' data first
        let (pos1, pos2, mass1, mass2, vel1, vel2) = {
            let body1 = self.bodies.get(&id1).unwrap();
            let body2 = self.bodies.get(&id2).unwrap();
            (
                body1.position,
                body2.position,
                body1.mass,
                body2.mass,
                body1.velocity,
                body2.velocity
            )
        };

        // Calculate collision response
        let normal = (pos2 - pos1).normalize();
        let relative_velocity = vel2 - vel1;
        let impulse = -2.0 * relative_velocity.dot(normal) / (1.0/mass1 + 1.0/mass2);
        
        // Apply the impulse to both bodies
        if let Some(body1) = self.bodies.get_mut(&id1) {
            body1.velocity -= impulse / mass1 * normal;
        }
        if let Some(body2) = self.bodies.get_mut(&id2) {
            body2.velocity += impulse / mass2 * normal;
        }
    }
} 