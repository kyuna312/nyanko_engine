use super::*;
use crate::ecs::{Component, Entity};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct PhysicsWorld {
    config: PhysicsConfig,
    bodies: HashMap<Entity, Arc<RwLock<RigidBody>>>,
    colliders: HashMap<Entity, Arc<RwLock<Collider>>>,
    spatial_hash: SpatialHash,
    accumulated_time: f32,
}

impl PhysicsWorld {
    pub fn new(config: PhysicsConfig) -> Self {
        Self {
            config,
            bodies: HashMap::new(),
            colliders: HashMap::new(),
            spatial_hash: SpatialHash::new(64.0), // Cell size of 64 units
            accumulated_time: 0.0,
        }
    }

    pub fn add_body(&mut self, entity: Entity, body: RigidBody) -> Arc<RwLock<RigidBody>> {
        let body = Arc::new(RwLock::new(body));
        self.bodies.insert(entity, body.clone());
        body
    }

    pub fn add_collider(&mut self, entity: Entity, collider: Collider) -> Arc<RwLock<Collider>> {
        let collider = Arc::new(RwLock::new(collider));
        self.colliders.insert(entity, collider.clone());
        collider
    }

    pub fn update(&mut self, delta_time: f32) {
        self.accumulated_time += delta_time;

        while self.accumulated_time >= self.config.time_step {
            self.step();
            self.accumulated_time -= self.config.time_step;
        }
    }

    fn step(&mut self) {
        // Update spatial hash
        self.spatial_hash.clear();
        for (entity, collider) in &self.colliders {
            if let Some(body) = self.bodies.get(entity) {
                let position = body.read().position;
                let collider = collider.read();
                self.spatial_hash
                    .insert(*entity, position, collider.bounds());
            }
        }

        // Detect and resolve collisions
        let mut collisions = Vec::new();
        for (entity_a, collider_a) in &self.colliders {
            let potential_collisions = self.spatial_hash.query(
                self.bodies[entity_a].read().position,
                collider_a.read().bounds(),
            );

            for entity_b in potential_collisions {
                if entity_a == &entity_b {
                    continue;
                }

                if let Some(collider_b) = self.colliders.get(&entity_b) {
                    if let Some(collision) = self.check_collision(
                        *entity_a,
                        entity_b,
                        &collider_a.read(),
                        &collider_b.read(),
                    ) {
                        collisions.push(collision);
                    }
                }
            }
        }

        // Resolve collisions
        for _ in 0..self.config.position_iterations {
            for collision in &collisions {
                self.resolve_collision(collision);
            }
        }

        // Update velocities
        for body in self.bodies.values() {
            let mut body = body.write();
            if body.body_type == BodyType::Dynamic {
                body.velocity += self.config.gravity * self.config.time_step;
            }
        }

        // Update positions
        for body in self.bodies.values() {
            let mut body = body.write();
            if body.body_type != BodyType::Static {
                body.position += body.velocity * self.config.time_step;
            }
        }
    }

    fn check_collision(
        &self,
        entity_a: Entity,
        entity_b: Entity,
        collider_a: &Collider,
        collider_b: &Collider,
    ) -> Option<Collision> {
        let body_a = &self.bodies[&entity_a].read();
        let body_b = &self.bodies[&entity_b].read();

        collision::detect_collision(body_a.position, body_b.position, collider_a, collider_b)
    }

    fn resolve_collision(&mut self, collision: &Collision) {
        let (body_a, body_b) = {
            let body_a = self.bodies.get(&collision.entity_a).unwrap();
            let body_b = self.bodies.get(&collision.entity_b).unwrap();
            (body_a.clone(), body_b.clone())
        };

        let mut body_a = body_a.write();
        let mut body_b = body_b.write();

        // Position correction
        let percent = 0.2; // Penetration percentage to correct
        let slop = 0.01; // Penetration allowance
        let correction = collision.normal
            * (f32::max(collision.penetration - slop, 0.0) / (body_a.inv_mass + body_b.inv_mass)
                * percent);

        if body_a.body_type == BodyType::Dynamic {
            body_a.position -= correction * body_a.inv_mass;
        }
        if body_b.body_type == BodyType::Dynamic {
            body_b.position += correction * body_b.inv_mass;
        }

        // Velocity resolution
        let relative_velocity = body_b.velocity - body_a.velocity;
        let velocity_along_normal = relative_velocity.dot(collision.normal);

        // Only resolve if objects are moving towards each other
        if velocity_along_normal > 0.0 {
            return;
        }

        let restitution = f32::min(body_a.material.restitution, body_b.material.restitution);

        let j = -(1.0 + restitution) * velocity_along_normal / (body_a.inv_mass + body_b.inv_mass);

        let impulse = collision.normal * j;

        if body_a.body_type == BodyType::Dynamic {
            body_a.velocity -= impulse * body_a.inv_mass;
        }
        if body_b.body_type == BodyType::Dynamic {
            body_b.velocity += impulse * body_b.inv_mass;
        }
    }
}
