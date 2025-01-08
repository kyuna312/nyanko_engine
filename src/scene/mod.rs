use crate::ecs::{Entity, EntityId};
use crate::components::{Transform, SpriteRenderer, RigidBody2D};
use glam::{Vec2, Vec3};
use std::collections::HashMap;

pub struct Scene {
    entities: HashMap<EntityId, Entity>,
    next_entity_id: EntityId,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_entity_id: 0,
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.insert(id, Entity::new(id));
        id
    }

    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn update(&mut self, dt: f32) {
        // Update physics
        for entity in self.entities.values_mut() {
            // Get RigidBody2D first
            if let Some(rb) = entity.get_component_mut::<RigidBody2D>() {
                // Apply gravity
                if rb.use_gravity {
                    rb.acceleration.y -= 9.81;
                }
                rb.velocity += rb.acceleration * dt;
                
                // Store velocity for transform update
                let velocity = rb.velocity;
                rb.acceleration = Vec2::ZERO;

                // Now get Transform and update it
                if let Some(transform) = entity.get_component_mut::<Transform>() {
                    transform.position += Vec3::new(velocity.x, velocity.y, 0.0) * dt;
                }
            }
        }

        // Check collisions
        // ... implement collision detection ...
    }

    pub fn render(&self, renderer: &mut crate::graphics::Renderer) {
        for entity in self.entities.values() {
            if let (Some(transform), Some(sprite)) = (
                entity.get_component::<Transform>(),
                entity.get_component::<SpriteRenderer>(),
            ) {
                if let Some(texture) = &sprite.texture {
                    renderer.draw_sprite(
                        texture,
                        Vec2::new(transform.position.x, transform.position.y),
                        transform.scale.truncate() * Vec2::new(
                            if sprite.flip_x { -1.0 } else { 1.0 },
                            if sprite.flip_y { -1.0 } else { 1.0 },
                        ),
                    );
                }
            }
        }
    }
} 