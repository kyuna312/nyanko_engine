use glam::{Mat4, Vec3};

pub struct Camera {
    position: Vec3,
    target: Vec3,
    up: Vec3,
    view: Mat4,
    projection: Mat4,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, up: Vec3) -> Self {
        let view = Mat4::look_at_rh(position, target, up);
        let projection = Mat4::perspective_rh(45.0f32.to_radians(), 16.0 / 9.0, 0.1, 1000.0);

        Self {
            position,
            target,
            up,
            view,
            projection,
        }
    }

    pub fn get_view(&self) -> Mat4 {
        self.view
    }

    pub fn get_projection(&self) -> Mat4 {
        self.projection
    }
}
