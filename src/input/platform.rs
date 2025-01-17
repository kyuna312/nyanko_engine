use super::*;
use crate::math::Vec2;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(target_os = "macos")]
use core_graphics::event as cg_event;
#[cfg(target_os = "windows")]
use winapi::um::winuser as win_input;
#[cfg(target_os = "linux")]
use x11::xlib as x11_input;

pub struct PlatformInput {
    keyboard_state: HashMap<u32, bool>,
    mouse_position: Vec2,
    mouse_buttons: HashMap<u32, bool>,
    #[cfg(target_os = "windows")]
    raw_input: bool,
    #[cfg(target_os = "macos")]
    event_tap: Option<cg_event::CGEventTap>,
}

impl PlatformInput {
    pub fn new() -> Self {
        Self {
            keyboard_state: HashMap::new(),
            mouse_position: Vec2::ZERO,
            mouse_buttons: HashMap::new(),
            #[cfg(target_os = "windows")]
            raw_input: false,
            #[cfg(target_os = "macos")]
            event_tap: None,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn enable_raw_input(&mut self) {
        unsafe {
            let mut rid = win_input::RAWINPUTDEVICE {
                usUsagePage: 0x01,
                usUsage: 0x02,
                dwFlags: win_input::RIDEV_INPUTSINK,
                hwndTarget: std::ptr::null_mut(),
            };
            win_input::RegisterRawInputDevices(
                &mut rid,
                1,
                std::mem::size_of::<win_input::RAWINPUTDEVICE>() as u32,
            );
            self.raw_input = true;
        }
    }

    #[cfg(target_os = "linux")]
    pub fn grab_input(&mut self, display: *mut x11_input::Display, window: x11_input::Window) {
        unsafe {
            x11_input::XGrabPointer(
                display,
                window,
                x11_input::True,
                (x11_input::ButtonPressMask
                    | x11_input::ButtonReleaseMask
                    | x11_input::PointerMotionMask) as i32,
                x11_input::GrabModeAsync,
                x11_input::GrabModeAsync,
                window,
                0,
                x11_input::CurrentTime,
            );
        }
    }

    #[cfg(target_os = "macos")]
    pub fn setup_event_tap(&mut self) {
        // Set up event tap for global input monitoring
        // Note: Requires accessibility permissions
    }
}

pub struct InputManager {
    platform_input: PlatformInput,
    key_states: HashMap<KeyCode, InputState>,
    previous_key_states: HashMap<KeyCode, InputState>,
    mouse_button_states: HashMap<MouseButton, InputState>,
    previous_mouse_button_states: HashMap<MouseButton, InputState>,
    mouse_position: Vec2,
    previous_mouse_position: Vec2,
    mouse_delta: Vec2,
    scroll_delta: Vec2,
    axes: HashMap<String, InputAxis>,
    text_input: String,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            platform_input: PlatformInput::new(),
            key_states: HashMap::new(),
            previous_key_states: HashMap::new(),
            mouse_button_states: HashMap::new(),
            previous_mouse_button_states: HashMap::new(),
            mouse_position: Vec2::ZERO,
            previous_mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: Vec2::ZERO,
            axes: HashMap::new(),
            text_input: String::new(),
        }
    }

    pub fn update(&mut self) {
        self.previous_key_states = self.key_states.clone();
        self.previous_mouse_button_states = self.mouse_button_states.clone();
        self.previous_mouse_position = self.mouse_position;

        self.platform_input.update();
        self.update_axes();

        // Clear per-frame data
        self.mouse_delta = Vec2::ZERO;
        self.scroll_delta = Vec2::ZERO;
        self.text_input.clear();
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::Pressed))
    }

    pub fn is_key_held(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::Held))
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        matches!(self.key_states.get(&key), Some(InputState::Released))
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_button_states.get(&button),
            Some(InputState::Pressed)
        )
    }

    pub fn is_mouse_button_held(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_button_states.get(&button),
            Some(InputState::Held)
        )
    }

    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        matches!(
            self.mouse_button_states.get(&button),
            Some(InputState::Released)
        )
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    pub fn get_axis(&self, name: &str) -> f32 {
        self.axes.get(name).map_or(0.0, |axis| axis.value)
    }

    pub fn get_raw_axis(&self, name: &str) -> f32 {
        self.axes.get(name).map_or(0.0, |axis| axis.raw_value)
    }

    pub fn register_axis(&mut self, axis: InputAxis) {
        self.axes.insert(axis.name.clone(), axis);
    }

    pub fn text_input(&self) -> &str {
        &self.text_input
    }

    fn update_axes(&mut self) {
        for axis in self.axes.values_mut() {
            let mut raw_value = 0.0;

            // Calculate raw value from keys
            for key in &axis.positive_keys {
                if self.is_key_held(*key) {
                    raw_value += 1.0;
                }
            }
            for key in &axis.negative_keys {
                if self.is_key_held(*key) {
                    raw_value -= 1.0;
                }
            }

            axis.raw_value = raw_value;

            // Apply sensitivity and gravity
            if raw_value != 0.0 {
                axis.value += raw_value * axis.sensitivity * self.platform_input.delta_time();
            } else {
                let gravity = axis.gravity * self.platform_input.delta_time();
                if axis.value > 0.0 {
                    axis.value = (axis.value - gravity).max(0.0);
                } else if axis.value < 0.0 {
                    axis.value = (axis.value + gravity).min(0.0);
                }
            }

            // Apply dead zone
            if axis.value.abs() < axis.dead_zone {
                axis.value = 0.0;
            }

            // Clamp value
            axis.value = axis.value.clamp(-1.0, 1.0);
        }
    }
}
