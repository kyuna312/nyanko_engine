use cgmath::Vector2;

pub struct PhysicsWorld {
    gravity: Vector2<f32>,
    objects: Vec<PhysicsObject>,
}

pub struct PhysicsObject {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub mass: f32,
    pub size: Vector2<f32>,
    pub is_static: bool,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        PhysicsWorld {
            gravity: Vector2::new(0.0, -9.81),
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: PhysicsObject) -> usize {
        self.objects.push(object);
        self.objects.len() - 1
    }

    pub fn update(&mut self, dt: f32) {
        for i in 0..self.objects.len() {
            if self.objects[i].is_static {
                continue;
            }

            // Cache current values
            let acceleration = self.objects[i].acceleration + self.gravity;
            let velocity = self.objects[i].velocity + acceleration * dt;
            let position = self.objects[i].position + velocity * dt;

            // Update object state
            self.objects[i].acceleration = Vector2::new(0.0, 0.0);
            self.objects[i].velocity = velocity;
            self.objects[i].position = position;

            // Store current object indices for collision checks
            let current_indices: Vec<usize> = (0..self.objects.len())
                .filter(|&j| j != i)
                .collect();

            // Check collisions
            for &j in &current_indices {
                self.check_collision(i, j);
            }
        }
    }

    fn check_collision(&mut self, i: usize, j: usize) {
        let (pos1, size1) = (self.objects[i].position, self.objects[i].size);
        let (pos2, size2) = (self.objects[j].position, self.objects[j].size);

        // AABB collision detection
        let collision = pos1.x < pos2.x + size2.x &&
                       pos1.x + size1.x > pos2.x &&
                       pos1.y < pos2.y + size2.y &&
                       pos1.y + size1.y > pos2.y;

        if collision {
            // Handle collision response
            if !self.objects[i].is_static {
                let vel = self.objects[i].velocity;
                self.objects[i].velocity = -vel * 0.8; // 0.8 is the restitution coefficient
            }
        }
    }

    pub fn get_object(&self, index: usize) -> Option<&PhysicsObject> {
        self.objects.get(index)
    }

    pub fn get_object_mut(&mut self, index: usize) -> Option<&mut PhysicsObject> {
        self.objects.get_mut(index)
    }
}

impl PhysicsObject {
    pub fn new(position: Vector2<f32>, size: Vector2<f32>, mass: f32) -> Self {
        PhysicsObject {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            mass,
            size,
            is_static: false,
        }
    }

    pub fn new_static(position: Vector2<f32>, size: Vector2<f32>) -> Self {
        PhysicsObject {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            mass: f32::INFINITY,
            size,
            is_static: true,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        if !self.is_static {
            self.acceleration += force / self.mass;
        }
    }

    pub fn apply_impulse(&mut self, impulse: Vector2<f32>) {
        if !self.is_static {
            self.velocity += impulse / self.mass;
        }
    }
} 