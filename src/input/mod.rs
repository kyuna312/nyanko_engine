mod platform;

use glam::Vec2;
use std::collections::HashMap;

pub use platform::PlatformInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    Left,
    Right,
    Up,
    Down,
    LShift,
    RShift,
    LControl,
    RControl,
    LAlt,
    RAlt,
    // Add more keys as needed
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputState {
    Pressed,
    Released,
    Held,
}

#[derive(Debug, Default)]
pub struct InputAxis {
    pub name: String,
    pub positive_keys: Vec<KeyCode>,
    pub negative_keys: Vec<KeyCode>,
    pub dead_zone: f32,
    pub sensitivity: f32,
    pub gravity: f32,
    pub value: f32,
    pub raw_value: f32,
}

impl InputAxis {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            positive_keys: Vec::new(),
            negative_keys: Vec::new(),
            dead_zone: 0.001,
            sensitivity: 3.0,
            gravity: 3.0,
            value: 0.0,
            raw_value: 0.0,
        }
    }
}
