use glam::{Mat4, Quat, Vec2, Vec3};

pub use glam;

/// Utility functions for common math operations
pub mod utils {
    use super::*;

    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t.clamp(0.0, 1.0)
    }

    pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        Vec2::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t))
    }

    pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
        Vec3::new(lerp(a.x, b.x, t), lerp(a.y, b.y, t), lerp(a.z, b.z, t))
    }
}

/// Transform related math operations
pub mod transform {
    use super::*;

    pub fn create_transform_matrix(position: Vec3, rotation: Quat, scale: Vec3) -> Mat4 {
        Mat4::from_scale_rotation_translation(scale, rotation, position)
    }

    pub fn look_at_2d(eye: Vec2, target: Vec2) -> Quat {
        let direction = (target - eye).normalize();
        let angle = direction.y.atan2(direction.x);
        Quat::from_rotation_z(angle)
    }
}
