use glam::{Vec2, Vec3, Vec4};
use rand::prelude::*;
use std::time::Duration;

pub struct Particle {
    position: Vec3,
    velocity: Vec3,
    color: Vec4,
    size: f32,
    life: f32,
    max_life: f32,
}

pub struct ParticleEmitter {
    particles: Vec<Particle>,
    position: Vec3,
    spawn_rate: f32,
    spawn_timer: f32,
    config: ParticleConfig,
}

pub struct ParticleConfig {
    pub max_particles: usize,
    pub particle_life: f32,
    pub particle_size: (f32, f32),
    pub velocity: (Vec3, Vec3),
    pub colors: Vec<Vec4>,
    pub size_over_life: Box<dyn Fn(f32) -> f32>,
    pub color_over_life: Box<dyn Fn(f32, Vec4) -> Vec4>,
}

impl ParticleEmitter {
    pub fn new(position: Vec3, config: ParticleConfig) -> Self {
        Self {
            particles: Vec::with_capacity(config.max_particles),
            position,
            spawn_rate: 10.0,
            spawn_timer: 0.0,
            config,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update existing particles
        self.particles.retain_mut(|particle| {
            particle.life -= delta_time;
            if particle.life <= 0.0 {
                return false;
            }

            particle.position += particle.velocity * delta_time;

            let life_ratio = particle.life / particle.max_life;
            particle.size = (self.config.size_over_life)(life_ratio);
            particle.color = (self.config.color_over_life)(life_ratio, particle.color);

            true
        });

        // Spawn new particles
        self.spawn_timer += delta_time;
        while self.spawn_timer >= 1.0 / self.spawn_rate
            && self.particles.len() < self.config.max_particles
        {
            self.spawn_particle();
            self.spawn_timer -= 1.0 / self.spawn_rate;
        }
    }

    fn spawn_particle(&mut self) {
        let mut rng = thread_rng();

        let velocity = Vec3::lerp(self.config.velocity.0, self.config.velocity.1, rng.gen());

        let size = rng.gen_range(self.config.particle_size.0..self.config.particle_size.1);

        let color = self.config.colors[rng.gen_range(0..self.config.colors.len())];

        self.particles.push(Particle {
            position: self.position,
            velocity,
            color,
            size,
            life: self.config.particle_life,
            max_life: self.config.particle_life,
        });
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }
}
