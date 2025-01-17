use super::{BodyType, Material};
use glam::Vec2;

#[derive(Debug)]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub body_type: BodyType,
    pub material: Material,
    pub mass: f32,
    pub inv_mass: f32,
    pub inertia: f32,
    pub inv_inertia: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub gravity_scale: f32,
    pub fixed_rotation: bool,
}

impl RigidBody {
    pub fn new(position: Vec2, body_type: BodyType) -> Self {
        let (mass, inv_mass) = match body_type {
            BodyType::Static => (0.0, 0.0),
            _ => (1.0, 1.0),
        };

        Self {
            position,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            rotation: 0.0,
            angular_velocity: 0.0,
            body_type,
            material: Material::default(),
            mass,
            inv_mass,
            inertia: 0.0,
            inv_inertia: 0.0,
            linear_damping: 0.01,
            angular_damping: 0.01,
            gravity_scale: 1.0,
            fixed_rotation: false,
        }
    }

    pub fn set_mass(&mut self, mass: f32) {
        if self.body_type != BodyType::Static {
            self.mass = mass;
            self.inv_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
            self.update_inertia();
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        if self.body_type == BodyType::Dynamic {
            self.acceleration += force * self.inv_mass;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if self.body_type == BodyType::Dynamic {
            self.velocity += impulse * self.inv_mass;
        }
    }

    pub fn apply_torque(&mut self, torque: f32) {
        if self.body_type == BodyType::Dynamic && !self.fixed_rotation {
            self.angular_velocity += torque * self.inv_inertia;
        }
    }

    fn update_inertia(&mut self) {
        // For now, assume circular inertia. This should be updated based on shape
        self.inertia = self.mass * (self.mass * 0.5).powi(2);
        self.inv_inertia = if self.inertia > 0.0 {
            1.0 / self.inertia
        } else {
            0.0
        };
    }
}
