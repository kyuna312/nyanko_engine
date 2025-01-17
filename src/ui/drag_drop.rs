use super::*;
use std::any::Any;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DragDropState {
    is_dragging: bool,
    drag_data: Option<DragData>,
    drag_start: Vec2,
    current_position: Vec2,
    drag_widget: Option<u64>,
    drop_target: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct DragData {
    data_type: String,
    payload: Arc<dyn Any + Send + Sync>,
    preview: Option<String>,
    allowed_drops: Vec<String>,
}

pub trait DragSource {
    fn can_drag(&self) -> bool;
    fn begin_drag(&self) -> Option<DragData>;
    fn end_drag(&mut self, completed: bool);
}

pub trait DropTarget {
    fn can_drop(&self, data: &DragData) -> bool;
    fn drop(&mut self, data: DragData);
    fn hover_enter(&mut self);
    fn hover_leave(&mut self);
}

impl DragDropState {
    pub fn new() -> Self {
        Self {
            is_dragging: false,
            drag_data: None,
            drag_start: Vec2::ZERO,
            current_position: Vec2::ZERO,
            drag_widget: None,
            drop_target: None,
        }
    }

    pub fn begin_drag(&mut self, widget_id: u64, position: Vec2, data: DragData) {
        self.is_dragging = true;
        self.drag_data = Some(data);
        self.drag_start = position;
        self.current_position = position;
        self.drag_widget = Some(widget_id);
    }

    pub fn update(&mut self, ctx: &mut UIContext) {
        if !self.is_dragging {
            return;
        }

        self.current_position = ctx.mouse_position;

        // End drag if mouse released
        if !ctx.pressed {
            if let Some(target) = self.drop_target {
                if let Some(data) = self.drag_data.take() {
                    ctx.drop_on_widget(target, data);
                }
            }
            self.end_drag(self.drop_target.is_some());
            return;
        }

        // Update drop target
        let old_target = self.drop_target;
        self.drop_target =
            ctx.find_drop_target(self.current_position, self.drag_data.as_ref().unwrap());

        // Handle hover events
        if old_target != self.drop_target {
            if let Some(old_id) = old_target {
                ctx.hover_leave_widget(old_id);
            }
            if let Some(new_id) = self.drop_target {
                ctx.hover_enter_widget(new_id);
            }
        }
    }

    pub fn draw(&self, renderer: &mut UIRenderer) {
        if !self.is_dragging {
            return;
        }

        // Draw drag preview
        if let Some(ref data) = self.drag_data {
            if let Some(ref preview) = data.preview {
                let preview_size = Vec2::new(32.0, 32.0); // Adjust based on preview content
                let preview_pos = self.current_position - preview_size * 0.5;

                renderer.draw_rect(
                    Rect::new(preview_pos, preview_size),
                    Vec4::new(0.2, 0.2, 0.2, 0.8),
                );

                renderer.draw_text(preview, preview_pos + Vec2::new(4.0, 4.0), 14.0, Vec4::ONE);
            }
        }

        // Draw drop target indicator
        if let Some(target_id) = self.drop_target {
            if let Some(target_rect) = renderer.get_widget_rect(target_id) {
                renderer.draw_rect_outline(target_rect, Vec4::new(0.2, 0.6, 1.0, 0.8), 2.0);
            }
        }
    }

    fn end_drag(&mut self, completed: bool) {
        if let Some(widget_id) = self.drag_widget {
            self.is_dragging = false;
            self.drag_data = None;
            self.drag_widget = None;
            self.drop_target = None;
        }
    }
}

// Extension trait for UIContext to handle drag and drop
pub trait DragDropContext {
    fn begin_drag(&mut self, widget_id: u64, data: DragData);
    fn update_drag_drop(&mut self);
    fn find_drop_target(&self, position: Vec2, data: &DragData) -> Option<u64>;
    fn drop_on_widget(&mut self, widget_id: u64, data: DragData);
    fn hover_enter_widget(&mut self, widget_id: u64);
    fn hover_leave_widget(&mut self, widget_id: u64);
}

impl DragDropContext for UIContext {
    fn begin_drag(&mut self, widget_id: u64, data: DragData) {
        self.drag_drop_state
            .begin_drag(widget_id, self.mouse_position, data);
    }

    fn update_drag_drop(&mut self) {
        self.drag_drop_state.update(self);
    }

    fn find_drop_target(&self, position: Vec2, data: &DragData) -> Option<u64> {
        // Find widget under cursor that implements DropTarget
        // and can accept the dragged data
        None // Implement actual widget hit testing
    }

    fn drop_on_widget(&mut self, widget_id: u64, data: DragData) {
        // Find widget and call its drop implementation
    }

    fn hover_enter_widget(&mut self, widget_id: u64) {
        // Notify widget of hover enter
    }

    fn hover_leave_widget(&mut self, widget_id: u64) {
        // Notify widget of hover leave
    }
}
