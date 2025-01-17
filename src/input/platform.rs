use std::collections::HashMap;
use std::time::Instant;

#[cfg(target_os = "macos")]
use core_graphics as cg;

pub struct PlatformInput {
    keyboard_state: HashMap<u32, bool>,
    mouse_position: (f64, f64),
    mouse_buttons: HashMap<u32, bool>,
    last_update: Instant,
    delta_time: f32,
}

impl PlatformInput {
    pub fn new() -> Self {
        Self {
            keyboard_state: HashMap::new(),
            mouse_position: (0.0, 0.0),
            mouse_buttons: HashMap::new(),
            last_update: Instant::now(),
            delta_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn set_key_state(&mut self, key: u32, state: bool) {
        self.keyboard_state.insert(key, state);
    }

    pub fn is_key_down(&self, key: u32) -> bool {
        *self.keyboard_state.get(&key).unwrap_or(&false)
    }

    pub fn set_mouse_position(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }

    pub fn mouse_position(&self) -> (f64, f64) {
        self.mouse_position
    }

    pub fn set_mouse_button(&mut self, button: u32, state: bool) {
        self.mouse_buttons.insert(button, state);
    }

    pub fn is_mouse_button_down(&self, button: u32) -> bool {
        *self.mouse_buttons.get(&button).unwrap_or(&false)
    }
}
