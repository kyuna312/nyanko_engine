use crate::ecs::Component;
use glam::Vec2;
use std::time::Duration;

#[derive(Clone)]
pub struct Weapon {
    pub damage: f32,
    pub fire_rate: f32,
    pub bullet_speed: f32,
    pub bullet_size: Vec2,
    pub magazine_size: u32,
    pub current_ammo: u32,
    pub reload_time: Duration,
    pub last_shot_time: f32,
    pub is_reloading: bool,
}

impl Component for Weapon {}

impl Weapon {
    pub fn new(damage: f32, fire_rate: f32, bullet_speed: f32) -> Self {
        Self {
            damage,
            fire_rate,
            bullet_speed,
            bullet_size: Vec2::new(10.0, 5.0),
            magazine_size: 30,
            current_ammo: 30,
            reload_time: Duration::from_secs(2),
            last_shot_time: 0.0,
            is_reloading: false,
        }
    }

    pub fn can_shoot(&self, current_time: f32) -> bool {
        !self.is_reloading 
            && self.current_ammo > 0 
            && current_time - self.last_shot_time >= 1.0 / self.fire_rate
    }

    pub fn shoot(&mut self, current_time: f32) -> bool {
        if self.can_shoot(current_time) {
            self.current_ammo -= 1;
            self.last_shot_time = current_time;
            true
        } else {
            false
        }
    }

    pub fn reload(&mut self) {
        if !self.is_reloading && self.current_ammo < self.magazine_size {
            self.is_reloading = true;
        }
    }

    pub fn update_reload(&mut self, dt: Duration) {
        if self.is_reloading {
            self.reload_time -= dt;
            if self.reload_time <= Duration::ZERO {
                self.current_ammo = self.magazine_size;
                self.is_reloading = false;
                self.reload_time = Duration::from_secs(2);
            }
        }
    }
} 