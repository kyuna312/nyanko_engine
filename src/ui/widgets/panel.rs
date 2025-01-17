use super::*;

pub struct Panel {
    base: BaseWidget,
    title: Option<String>,
    draggable: bool,
    drag_start: Option<Vec2>,
}

impl Panel {
    pub fn new() -> Self {
        let mut base = BaseWidget::new(WidgetType::Container);

        // Set default panel style
        base.style = Style::new()
            .with_background_color(Vec4::new(0.15, 0.15, 0.15, 0.95))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0)
            .with_padding(Vec2::new(8.0, 8.0));

        // Set default panel layout
        base.layout = Layout::new(LayoutType::Vertical).with_spacing(Vec2::new(4.0, 4.0));

        Self {
            base,
            title: None,
            draggable: false,
            drag_start: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    pub fn add_child(&mut self, child: Arc<RwLock<dyn Widget>>) {
        self.base.children_mut().push(child);
    }
}

impl Widget for Panel {
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

        // Handle dragging
        if self.draggable {
            if !old_state.pressed && state.pressed {
                // Start dragging
                self.drag_start = Some(self.base.rect().position);
            } else if old_state.pressed && !state.pressed {
                // Stop dragging
                self.drag_start = None;
            }
        }
    }

    fn update(&mut self, ctx: &mut UIContext) {
        // Handle dragging
        if self.draggable && self.drag_start.is_some() && self.base.state().pressed {
            let delta = ctx.mouse_position - self.drag_start.unwrap();
            let mut rect = self.base.rect();
            rect.position = self.drag_start.unwrap() + delta;
            self.base.set_rect(rect);
        }

        // Update base widget and children
        self.base.update(ctx);
    }

    // Implement other Widget trait methods by delegating to base
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
