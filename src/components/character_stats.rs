use crate::ecs::Component;

#[derive(Clone)]
pub struct CharacterStats {
    pub level: u32,
    pub exp: u32,
    pub hp: f32,
    pub max_hp: f32,
    pub mp: f32,
    pub max_mp: f32,
    pub attack: f32,
    pub defense: f32,
    pub speed: f32,
}

impl Component for CharacterStats {}

impl CharacterStats {
    pub fn new(level: u32) -> Self {
        Self {
            level,
            exp: 0,
            hp: 100.0,
            max_hp: 100.0,
            mp: 50.0,
            max_mp: 50.0,
            attack: 10.0,
            defense: 5.0,
            speed: 100.0,
        }
    }

    pub fn add_exp(&mut self, exp: u32) -> bool {
        self.exp += exp;
        let exp_needed = self.level * 100;

        if self.exp >= exp_needed {
            self.level_up();
            true
        } else {
            false
        }
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.max_hp *= 1.1;
        self.max_mp *= 1.1;
        self.hp = self.max_hp;
        self.mp = self.max_mp;
        self.attack *= 1.1;
        self.defense *= 1.1;
    }

    pub fn take_damage(&mut self, amount: f32) -> bool {
        let actual_damage = (amount - self.defense).max(1.0);
        self.hp -= actual_damage;
        self.hp <= 0.0
    }

    pub fn heal(&mut self, amount: f32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn use_mp(&mut self, amount: f32) -> bool {
        if self.mp >= amount {
            self.mp -= amount;
            true
        } else {
            false
        }
    }
}
