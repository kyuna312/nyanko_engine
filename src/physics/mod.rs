use cgmath::Vector2;
use std::vec::Vec;

pub struct PhysicsObject {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub mass: f32,
    pub velocity: Vector2<f32>,
    pub bounce_count: i32,
    pub should_expand: bool,
}

impl PhysicsObject {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, mass: f32) -> Self {
        Self {
            position,
            size,
            mass,
            velocity: Vector2::new(0.0, 0.0),
            bounce_count: 0,
            should_expand: false,
        }
    }
}

pub struct PhysicsWorld {
    objects: Vec<PhysicsObject>,
    gravity: f32,
    damping: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            gravity: 3.0,
            damping: 0.7,
        }
    }

    pub fn add_object(&mut self, object: PhysicsObject) -> usize {
        let id = self.objects.len();
        self.objects.push(object);
        id
    }

    pub fn get_object(&self, id: usize) -> Option<&PhysicsObject> {
        self.objects.get(id)
    }

    pub fn get_object_mut(&mut self, id: usize) -> Option<&mut PhysicsObject> {
        self.objects.get_mut(id)
    }

    pub fn update(&mut self, dt: f32) {
        for object in self.objects.iter_mut() {
            // Apply gravity
            object.velocity.y -= self.gravity * dt;
            
            // Update position
            object.position += object.velocity * dt;

            // Ground collision with bounce counting
            if object.position.y < -1.0 {
                object.position.y = -1.0;
                if object.velocity.y < 0.0 {
                    // Only count bounce when velocity is negative (moving down)
                    object.bounce_count += 1;
                    
                    // Apply damping to slow down bounces
                    object.velocity.y = -object.velocity.y * self.damping;

                    // Check if we should trigger expansion
                    if object.bounce_count >= 3 {
                        object.should_expand = true;
                        object.velocity = Vector2::new(0.0, 0.0); // Stop movement
                    }
                }
            }
        }
    }
} 