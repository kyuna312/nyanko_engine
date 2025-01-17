use crate::ecs::Entity;
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec2,
    pub penetration: f32,
    pub contact_point: Vec2,
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: CollisionShape,
    pub offset: Vec2,
    pub trigger: bool,
}

#[derive(Debug, Clone)]
pub enum CollisionShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Polygon { vertices: Vec<Vec2> },
}

impl Collider {
    pub fn new(shape: CollisionShape) -> Self {
        Self {
            shape,
            offset: Vec2::ZERO,
            trigger: false,
        }
    }

    pub fn bounds(&self) -> (Vec2, Vec2) {
        match &self.shape {
            CollisionShape::Circle { radius } => {
                let min = Vec2::new(-radius, -radius) + self.offset;
                let max = Vec2::new(*radius, *radius) + self.offset;
                (min, max)
            }
            CollisionShape::Rectangle { width, height } => {
                let min = Vec2::new(-width / 2.0, -height / 2.0) + self.offset;
                let max = Vec2::new(*width / 2.0, *height / 2.0) + self.offset;
                (min, max)
            }
            CollisionShape::Polygon { vertices } => {
                let mut min = Vec2::new(f32::MAX, f32::MAX);
                let mut max = Vec2::new(f32::MIN, f32::MIN);
                for vertex in vertices {
                    min = min.min(*vertex + self.offset);
                    max = max.max(*vertex + self.offset);
                }
                (min, max)
            }
        }
    }
}

pub fn detect_collision(
    pos_a: Vec2,
    pos_b: Vec2,
    collider_a: &Collider,
    collider_b: &Collider,
) -> Option<Collision> {
    match (&collider_a.shape, &collider_b.shape) {
        (CollisionShape::Circle { radius: r1 }, CollisionShape::Circle { radius: r2 }) => {
            check_circle_circle(pos_a, pos_b, *r1, *r2)
        }
        (
            CollisionShape::Rectangle {
                width: w1,
                height: h1,
            },
            CollisionShape::Rectangle {
                width: w2,
                height: h2,
            },
        ) => check_aabb_aabb(
            pos_a + collider_a.offset,
            pos_b + collider_b.offset,
            Vec2::new(*w1, *h1),
            Vec2::new(*w2, *h2),
        ),
        (CollisionShape::Circle { radius }, CollisionShape::Rectangle { width, height }) => {
            check_circle_aabb(
                pos_a + collider_a.offset,
                pos_b + collider_b.offset,
                *radius,
                Vec2::new(*width, *height),
            )
        }
        (CollisionShape::Rectangle { .. }, CollisionShape::Circle { .. }) => {
            // Swap the order and negate the normal
            detect_collision(pos_b, pos_a, collider_b, collider_a).map(|mut c| {
                c.normal = -c.normal;
                c
            })
        }
        _ => None, // Other shape combinations not yet implemented
    }
}

fn check_circle_circle(
    pos_a: Vec2,
    pos_b: Vec2,
    radius_a: f32,
    radius_b: f32,
) -> Option<Collision> {
    let distance = pos_b - pos_a;
    let distance_sqr = distance.length_squared();
    let combined_radius = radius_a + radius_b;

    if distance_sqr < combined_radius * combined_radius {
        let distance_len = distance_sqr.sqrt();
        let normal = if distance_len != 0.0 {
            distance / distance_len
        } else {
            Vec2::new(1.0, 0.0)
        };

        Some(Collision {
            entity_a: Entity::new(0), // These will be filled in by the physics system
            entity_b: Entity::new(0),
            normal,
            penetration: combined_radius - distance_len,
            contact_point: pos_a + normal * radius_a,
        })
    } else {
        None
    }
}

fn check_aabb_aabb(pos_a: Vec2, pos_b: Vec2, size_a: Vec2, size_b: Vec2) -> Option<Collision> {
    let half_a = size_a * 0.5;
    let half_b = size_b * 0.5;

    let min_a = pos_a - half_a;
    let max_a = pos_a + half_a;
    let min_b = pos_b - half_b;
    let max_b = pos_b + half_b;

    if max_a.x < min_b.x || min_a.x > max_b.x || max_a.y < min_b.y || min_a.y > max_b.y {
        return None;
    }

    let overlap_x = (max_a.x.min(max_b.x) - min_a.x.max(min_b.x)).abs();
    let overlap_y = (max_a.y.min(max_b.y) - min_a.y.max(min_b.y)).abs();

    let normal = if overlap_x < overlap_y {
        Vec2::new(if pos_a.x < pos_b.x { -1.0 } else { 1.0 }, 0.0)
    } else {
        Vec2::new(0.0, if pos_a.y < pos_b.y { -1.0 } else { 1.0 })
    };

    Some(Collision {
        entity_a: Entity::new(0),
        entity_b: Entity::new(0),
        normal,
        penetration: overlap_x.min(overlap_y),
        contact_point: pos_a + (pos_b - pos_a) * 0.5,
    })
}
