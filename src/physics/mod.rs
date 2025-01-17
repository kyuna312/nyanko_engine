mod collision;
mod rigidbody;
mod spatial;
mod world;

pub use collision::{Collider, Collision, CollisionShape};
pub use rigidbody::RigidBody;
pub use spatial::SpatialHash;
pub use world::PhysicsWorld;

use glam::Vec2;
use std::collections::HashMap;

pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub velocity_iterations: u32,
    pub position_iterations: u32,
    pub time_step: f32,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity: Vec2::new(0.0, -9.81),
            velocity_iterations: 8,
            position_iterations: 3,
            time_step: 1.0 / 60.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub density: f32,
    pub restitution: f32,
    pub friction: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            density: 1.0,
            restitution: 0.5,
            friction: 0.3,
        }
    }
}
