use glam::{Vec3, Vec4};
use rand::Rng;

pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

struct Particle {
    position: Vec3,
    velocity: Vec3,
    color: Vec4,
    life: f32,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            max_particles: 1000,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        for particle in &mut self.particles {
            particle.life -= delta_time;
            particle.position += particle.velocity * delta_time;
        }

        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn emit(&mut self, position: Vec3, color: Vec4) {
        if self.particles.len() >= self.max_particles {
            return;
        }

        let mut rng = rand::thread_rng();
        let velocity = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(0.0..2.0),
            rng.gen_range(-1.0..1.0),
        );

        self.particles.push(Particle {
            position,
            velocity,
            color,
            life: rng.gen_range(0.5..2.0),
        });
    }
}
