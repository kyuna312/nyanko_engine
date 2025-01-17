use super::*;

pub struct Button {
    base: BaseWidget,
    text: String,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        let mut base = BaseWidget::new(WidgetType::Button);

        // Set default button style
        base.style = Style::new()
            .with_background_color(Vec4::new(0.2, 0.6, 1.0, 1.0))
            .with_corner_radius(4.0)
            .with_padding(Vec2::new(16.0, 8.0));

        Self {
            base,
            text: text.to_string(),
            on_click: None,
        }
    }

    pub fn with_on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl Widget for Button {
    fn widget_type(&self) -> WidgetType {
        self.base.widget_type()
    }
    fn id(&self) -> u64 {
        self.base.id()
    }
    fn state(&self) -> WidgetState {
        self.base.state()
    }
    fn set_state(&mut self, state: WidgetState) {
        let old_state = self.base.state();
        self.base.set_state(state);

        // Handle click event
        if old_state.pressed && !state.pressed && state.hovered {
            if let Some(ref on_click) = self.on_click {
                on_click();
            }
        }
    }
    fn rect(&self) -> Rect {
        self.base.rect()
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.set_rect(rect);
    }
    fn style(&self) -> &Style {
        self.base.style()
    }
    fn set_style(&mut self, style: Style) {
        self.base.set_style(style);
    }
    fn layout(&self) -> &Layout {
        self.base.layout()
    }
    fn set_layout(&mut self, layout: Layout) {
        self.base.set_layout(layout);
    }
    fn children(&self) -> &[Arc<RwLock<dyn Widget>>] {
        self.base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Arc<RwLock<dyn Widget>>> {
        self.base.children_mut()
    }
    fn parent(&self) -> Option<Arc<RwLock<dyn Widget>>> {
        self.base.parent()
    }
    fn set_parent(&mut self, parent: Option<Arc<RwLock<dyn Widget>>>) {
        self.base.set_parent(parent);
    }
}
