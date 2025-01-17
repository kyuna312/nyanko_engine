use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    None,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    None,
    Min(f32),
    Max(f32),
    Fixed(f32),
    Percentage(f32),
    Aspect(f32),
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub layout_type: LayoutType,
    pub constraints: (Constraint, Constraint),
    pub margin: Vec2,
    pub padding: Vec2,
    pub spacing: Vec2,
    pub align: Vec2,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            layout_type: LayoutType::None,
            constraints: (Constraint::None, Constraint::None),
            margin: Vec2::ZERO,
            padding: Vec2::new(4.0, 4.0),
            spacing: Vec2::new(4.0, 4.0),
            align: Vec2::new(0.5, 0.5),
        }
    }
}

impl Layout {
    pub fn new(layout_type: LayoutType) -> Self {
        Self {
            layout_type,
            ..Default::default()
        }
    }

    pub fn with_constraints(mut self, width: Constraint, height: Constraint) -> Self {
        self.constraints = (width, height);
        self
    }

    pub fn with_margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_spacing(mut self, spacing: Vec2) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn with_align(mut self, align: Vec2) -> Self {
        self.align = align.clamp(Vec2::ZERO, Vec2::ONE);
        self
    }

    pub fn apply_constraint(&self, available: f32, constraint: &Constraint) -> f32 {
        match constraint {
            Constraint::None => available,
            Constraint::Min(min) => available.max(*min),
            Constraint::Max(max) => available.min(*max),
            Constraint::Fixed(size) => *size,
            Constraint::Percentage(percent) => available * percent,
            Constraint::Aspect(ratio) => available * ratio,
        }
    }
}
