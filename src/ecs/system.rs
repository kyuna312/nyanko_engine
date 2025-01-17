use super::World;

pub trait System: Send + Sync {
    fn update(&mut self, world: &mut World, delta_time: f32);
}

pub struct PhysicsSystem {
    gravity: f32,
    substeps: u32,
}

impl PhysicsSystem {
    pub fn new(gravity: f32, substeps: u32) -> Self {
        Self { gravity, substeps }
    }
}

impl System for PhysicsSystem {
    fn update(&mut self, world: &mut World, delta_time: f32) {
        let dt = delta_time / self.substeps as f32;

        for _ in 0..self.substeps {
            // Update velocities
            for entity in &world.entities {
                if let Some(rb) = world.get_component_mut::<RigidBody2D>(*entity) {
                    if !rb.is_kinematic && rb.use_gravity {
                        rb.velocity.y -= self.gravity * dt;
                    }
                    rb.velocity += rb.acceleration * dt;
                    rb.acceleration = Vec2::ZERO;
                }
            }

            // Update positions
            for entity in &world.entities {
                if let (Some(transform), Some(rb)) = (
                    world.get_component_mut::<Transform>(*entity),
                    world.get_component::<RigidBody2D>(*entity),
                ) {
                    if !rb.is_kinematic {
                        transform.position.x += rb.velocity.x * dt;
                        transform.position.y += rb.velocity.y * dt;
                        transform.dirty = true;
                    }
                }
            }
        }
    }
}
