mod layout;
mod renderer;
mod style;
mod widget;

use glam::{Vec2, Vec4};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub use layout::{Constraint, Layout, LayoutType};
pub use renderer::UIRenderer;
pub use style::{Style, StyleSheet};
pub use widget::{Widget, WidgetState, WidgetType};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.position.x
            && point.x <= self.position.x + self.size.x
            && point.y >= self.position.y
            && point.y <= self.position.y + self.size.y
    }
}

#[derive(Debug, Clone)]
pub struct UIContext {
    pub screen_size: Vec2,
    pub scale_factor: f32,
    pub mouse_position: Vec2,
    pub pressed: bool,
    pub hovered_widget: Option<u64>,
    pub active_widget: Option<u64>,
    pub focused_widget: Option<u64>,
}

#[derive(Debug)]
pub struct Theme {
    pub font: String,
    pub font_size: f32,
    pub text_color: Vec4,
    pub primary_color: Vec4,
    pub secondary_color: Vec4,
    pub background_color: Vec4,
    pub border_color: Vec4,
    pub border_width: f32,
    pub corner_radius: f32,
    pub padding: Vec2,
    pub spacing: Vec2,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            font: "default".to_string(),
            font_size: 16.0,
            text_color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            primary_color: Vec4::new(0.2, 0.6, 1.0, 1.0),
            secondary_color: Vec4::new(0.5, 0.5, 0.5, 1.0),
            background_color: Vec4::new(0.1, 0.1, 0.1, 0.9),
            border_color: Vec4::new(0.3, 0.3, 0.3, 1.0),
            border_width: 1.0,
            corner_radius: 4.0,
            padding: Vec2::new(8.0, 8.0),
            spacing: Vec2::new(4.0, 4.0),
        }
    }
}
