use super::Theme;
use glam::Vec4;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Style {
    pub background_color: Option<Vec4>,
    pub border_color: Option<Vec4>,
    pub text_color: Option<Vec4>,
    pub border_width: Option<f32>,
    pub corner_radius: Option<f32>,
    pub font_size: Option<f32>,
    pub opacity: f32,
    pub custom_properties: HashMap<String, StyleValue>,
}

#[derive(Debug, Clone)]
pub enum StyleValue {
    Color(Vec4),
    Float(f32),
    Int(i32),
    String(String),
    Bool(bool),
}

impl Default for Style {
    fn default() -> Self {
        Self {
            background_color: None,
            border_color: None,
            text_color: None,
            border_width: None,
            corner_radius: None,
            font_size: None,
            opacity: 1.0,
            custom_properties: HashMap::new(),
        }
    }
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background_color(mut self, color: Vec4) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_border_color(mut self, color: Vec4) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn with_text_color(mut self, color: Vec4) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn with_border_width(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    pub fn with_corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    pub fn resolve(&self, theme: &Theme) -> ResolvedStyle {
        ResolvedStyle {
            background_color: self.background_color.unwrap_or(theme.background_color),
            border_color: self.border_color.unwrap_or(theme.border_color),
            text_color: self.text_color.unwrap_or(theme.text_color),
            border_width: self.border_width.unwrap_or(theme.border_width),
            corner_radius: self.corner_radius.unwrap_or(theme.corner_radius),
            font_size: self.font_size.unwrap_or(theme.font_size),
            opacity: self.opacity,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedStyle {
    pub background_color: Vec4,
    pub border_color: Vec4,
    pub text_color: Vec4,
    pub border_width: f32,
    pub corner_radius: f32,
    pub font_size: f32,
    pub opacity: f32,
}

pub struct StyleSheet {
    styles: HashMap<String, Style>,
    theme: Theme,
}

impl StyleSheet {
    pub fn new(theme: Theme) -> Self {
        Self {
            styles: HashMap::new(),
            theme,
        }
    }

    pub fn add_style(&mut self, name: &str, style: Style) {
        self.styles.insert(name.to_string(), style);
    }

    pub fn get_style(&self, name: &str) -> Option<&Style> {
        self.styles.get(name)
    }

    pub fn resolve_style(&self, name: &str) -> ResolvedStyle {
        self.styles
            .get(name)
            .map(|style| style.resolve(&self.theme))
            .unwrap_or_else(|| Style::default().resolve(&self.theme))
    }
}
