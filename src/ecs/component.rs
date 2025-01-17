use glam::{Quat, Vec2, Vec3};
use std::any::Any;

pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub local_matrix: glam::Mat4,
    pub world_matrix: glam::Mat4,
    pub dirty: bool,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        let mut transform = Self {
            position,
            rotation,
            scale,
            local_matrix: glam::Mat4::IDENTITY,
            world_matrix: glam::Mat4::IDENTITY,
            dirty: true,
        };
        transform.update_matrices();
        transform
    }

    pub fn update_matrices(&mut self) {
        if self.dirty {
            self.local_matrix = glam::Mat4::from_scale_rotation_translation(
                self.scale,
                self.rotation,
                self.position,
            );
            self.world_matrix = self.local_matrix;
            self.dirty = false;
        }
    }
}

impl Component for Transform {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub texture_id: Option<u32>,
    pub color: [f32; 4],
    pub size: Vec2,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Component for Sprite {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct RigidBody2D {
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub use_gravity: bool,
    pub is_kinematic: bool,
    pub restitution: f32,
    pub friction: f32,
}

impl Component for RigidBody2D {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
