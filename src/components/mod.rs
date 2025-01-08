mod character_stats;
mod weapon;

pub use character_stats::CharacterStats;
pub use weapon::Weapon;
use crate::ecs::Component;
use crate::graphics::Texture;
use glam::{Vec2, Vec3, Quat};

#[derive(Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Component for Transform {}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

#[derive(Clone)]
pub struct SpriteRenderer {
    pub texture: Option<Texture>,
    pub color: [f32; 4],
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Component for SpriteRenderer {}

impl Default for SpriteRenderer {
    fn default() -> Self {
        Self {
            texture: None,
            color: [1.0, 1.0, 1.0, 1.0],
            flip_x: false,
            flip_y: false,
        }
    }
}

#[derive(Clone)]
pub struct RigidBody2D {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub use_gravity: bool,
}

impl Component for RigidBody2D {}

impl Default for RigidBody2D {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            use_gravity: true,
        }
    }
}

#[derive(Clone)]
pub struct BoxCollider2D {
    pub size: Vec2,
    pub offset: Vec2,
    pub is_trigger: bool,
}

impl Component for BoxCollider2D {}

impl Default for BoxCollider2D {
    fn default() -> Self {
        Self {
            size: Vec2::ONE,
            offset: Vec2::ZERO,
            is_trigger: false,
        }
    }
} 