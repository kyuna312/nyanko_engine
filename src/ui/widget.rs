use super::*;
use std::any::Any;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    Container,
    Button,
    Label,
    Image,
    TextField,
    Checkbox,
    Slider,
    ScrollView,
    List,
    Custom(u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidgetState {
    pub hovered: bool,
    pub pressed: bool,
    pub focused: bool,
    pub active: bool,
    pub enabled: bool,
    pub visible: bool,
}

impl Default for WidgetState {
    fn default() -> Self {
        Self {
            hovered: false,
            pressed: false,
            focused: false,
            active: false,
            enabled: true,
            visible: true,
        }
    }
}

pub trait Widget: Any + Send + Sync {
    fn widget_type(&self) -> WidgetType;
    fn id(&self) -> u64;
    fn state(&self) -> WidgetState;
    fn set_state(&mut self, state: WidgetState);
    fn rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
    fn style(&self) -> &Style;
    fn set_style(&mut self, style: Style);
    fn layout(&self) -> &Layout;
    fn set_layout(&mut self, layout: Layout);
    fn children(&self) -> &[Arc<RwLock<dyn Widget>>];
    fn children_mut(&mut self) -> &mut Vec<Arc<RwLock<dyn Widget>>>;
    fn parent(&self) -> Option<Arc<RwLock<dyn Widget>>>;
    fn set_parent(&mut self, parent: Option<Arc<RwLock<dyn Widget>>>);

    fn update(&mut self, ctx: &mut UIContext) {
        // Update state based on input
        let mut state = self.state();
        let rect = self.rect();

        state.hovered = rect.contains(ctx.mouse_position);
        if state.hovered {
            ctx.hovered_widget = Some(self.id());
            if ctx.pressed {
                state.pressed = true;
                ctx.active_widget = Some(self.id());
            }
        }

        self.set_state(state);

        // Update children
        for child in self.children_mut() {
            child.write().update(ctx);
        }
    }

    fn layout_widgets(&mut self, available_space: Vec2) -> Vec2 {
        let mut size = Vec2::ZERO;
        let layout = self.layout().clone();

        match layout.layout_type {
            LayoutType::None => {
                size = available_space;
            }
            LayoutType::Horizontal => {
                let mut x = 0.0;
                for child in self.children_mut() {
                    let child_size = child
                        .write()
                        .layout_widgets(Vec2::new(available_space.x - x, available_space.y));
                    let mut child = child.write();
                    let mut rect = child.rect();
                    rect.position.x = x;
                    rect.size = child_size;
                    child.set_rect(rect);
                    x += child_size.x + layout.spacing.x;
                    size.x = x;
                    size.y = size.y.max(child_size.y);
                }
            }
            LayoutType::Vertical => {
                let mut y = 0.0;
                for child in self.children_mut() {
                    let child_size = child
                        .write()
                        .layout_widgets(Vec2::new(available_space.x, available_space.y - y));
                    let mut child = child.write();
                    let mut rect = child.rect();
                    rect.position.y = y;
                    rect.size = child_size;
                    child.set_rect(rect);
                    y += child_size.y + layout.spacing.y;
                    size.y = y;
                    size.x = size.x.max(child_size.x);
                }
            }
        }

        size
    }
}
