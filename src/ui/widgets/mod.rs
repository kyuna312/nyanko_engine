mod button;
mod panel;
mod text_input;

pub use button::Button;
pub use panel::Panel;
pub use text_input::TextInput;

use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static WIDGET_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

fn generate_widget_id() -> u64 {
    WIDGET_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug)]
pub struct BaseWidget {
    id: u64,
    widget_type: WidgetType,
    state: WidgetState,
    rect: Rect,
    style: Style,
    layout: Layout,
    children: Vec<Arc<RwLock<dyn Widget>>>,
    parent: Option<Arc<RwLock<dyn Widget>>>,
}

impl BaseWidget {
    pub fn new(widget_type: WidgetType) -> Self {
        Self {
            id: generate_widget_id(),
            widget_type,
            state: WidgetState::default(),
            rect: Rect::new(Vec2::ZERO, Vec2::ZERO),
            style: Style::default(),
            layout: Layout::default(),
            children: Vec::new(),
            parent: None,
        }
    }
}

impl Widget for BaseWidget {
    fn widget_type(&self) -> WidgetType {
        self.widget_type
    }
    fn id(&self) -> u64 {
        self.id
    }
    fn state(&self) -> WidgetState {
        self.state
    }
    fn set_state(&mut self, state: WidgetState) {
        self.state = state;
    }
    fn rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    fn style(&self) -> &Style {
        &self.style
    }
    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
    fn layout(&self) -> &Layout {
        &self.layout
    }
    fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }
    fn children(&self) -> &[Arc<RwLock<dyn Widget>>] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Arc<RwLock<dyn Widget>>> {
        &mut self.children
    }
    fn parent(&self) -> Option<Arc<RwLock<dyn Widget>>> {
        self.parent.clone()
    }
    fn set_parent(&mut self, parent: Option<Arc<RwLock<dyn Widget>>>) {
        self.parent = parent;
    }
}
