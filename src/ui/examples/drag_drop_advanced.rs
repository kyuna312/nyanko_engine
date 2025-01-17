use super::*;
use std::sync::Arc;

// Example of dragging between different widget types
pub struct AdvancedDragDrop {
    color_picker: ColorPicker,
    tree_view: TreeView,
    text_editor: TextEditor,
    drag_drop_state: DragDropState,
}

#[derive(Debug, Clone)]
enum DragPayload {
    Color(Vec4),
    Text(String),
    Node(Arc<RwLock<TreeNode>>),
}

impl AdvancedDragDrop {
    pub fn new() -> Self {
        Self {
            color_picker: ColorPicker::new(Vec4::ONE),
            tree_view: TreeView::new(),
            text_editor: TextEditor::new(),
            drag_drop_state: DragDropState::new(),
        }
    }
}

pub struct ColorPicker {
    color: Vec4,
}

impl DragSource for ColorPicker {
    fn can_drag(&self) -> bool {
        true
    }

    fn begin_drag(&self) -> Option<DragData> {
        Some(DragData {
            data_type: "color".to_string(),
            payload: Arc::new(DragPayload::Color(self.color)),
            preview: Some("🎨".to_string()),
            allowed_drops: vec!["text".to_string(), "node".to_string()],
        })
    }

    fn end_drag(&mut self, _completed: bool) {}
}

pub struct TextEditor {
    text: String,
    selection: (usize, usize),
}

impl DragSource for TextEditor {
    fn can_drag(&self) -> bool {
        self.has_selection()
    }

    fn begin_drag(&self) -> Option<DragData> {
        Some(DragData {
            data_type: "text".to_string(),
            payload: Arc::new(DragPayload::Text(self.selected_text().to_string())),
            preview: Some("📝".to_string()),
            allowed_drops: vec!["node".to_string()],
        })
    }

    fn end_drag(&mut self, completed: bool) {
        if completed {
            self.clear_selection();
        }
    }
}

impl DropTarget for TextEditor {
    fn can_drop(&self, data: &DragData) -> bool {
        true
    }

    fn drop(&mut self, data: DragData) {
        if let Some(payload) = data.payload.downcast_ref::<DragPayload>() {
            match payload {
                DragPayload::Text(text) => {
                    self.insert_text(text);
                }
                DragPayload::Color(color) => {
                    self.insert_text(&format!(
                        "rgba({}, {}, {}, {})",
                        (color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                        (color.w * 255.0) as u8,
                    ));
                }
                _ => {}
            }
        }
    }

    fn hover_enter(&mut self) {
        self.highlight = true;
    }

    fn hover_leave(&mut self) {
        self.highlight = false;
    }
}

// Custom drag preview rendering
fn draw_drag_preview(&self, renderer: &mut UIRenderer, data: &DragData, position: Vec2) {
    if let Some(payload) = data.payload.downcast_ref::<DragPayload>() {
        match payload {
            DragPayload::Color(color) => {
                // Draw color swatch
                let preview_size = Vec2::new(32.0, 32.0);
                let preview_pos = position - preview_size * 0.5;

                renderer.draw_rect(Rect::new(preview_pos, preview_size), *color);
                renderer.draw_rect_outline(
                    Rect::new(preview_pos, preview_size),
                    Vec4::new(0.3, 0.3, 0.3, 1.0),
                    1.0,
                );
            }
            DragPayload::Text(text) => {
                // Draw text preview
                let preview_size = Vec2::new(200.0, 32.0);
                let preview_pos = position - preview_size * 0.5;

                renderer.draw_rect(
                    Rect::new(preview_pos, preview_size),
                    Vec4::new(0.2, 0.2, 0.2, 0.8),
                );
                renderer.draw_text(
                    &format!("📝 {}", text),
                    preview_pos + Vec2::new(8.0, 8.0),
                    14.0,
                    Vec4::ONE,
                );
            }
            DragPayload::Node(node) => {
                // Draw node preview
                let preview_size = Vec2::new(200.0, 32.0);
                let preview_pos = position - preview_size * 0.5;

                renderer.draw_rect(
                    Rect::new(preview_pos, preview_size),
                    Vec4::new(0.2, 0.2, 0.2, 0.8),
                );
                renderer.draw_text(
                    &format!("🌳 {}", node.read().unwrap().text),
                    preview_pos + Vec2::new(8.0, 8.0),
                    14.0,
                    Vec4::ONE,
                );
            }
        }
    }
}

fn update(&mut self, ctx: &mut UIContext) {
    self.color_picker.update(ctx);
    self.tree_view.update(ctx);
    self.text_editor.update(ctx);

    // Handle drag and drop
    self.drag_drop_state.update(ctx);
}

fn draw(&self, renderer: &mut UIRenderer) {
    self.color_picker.draw(renderer);
    self.tree_view.draw(renderer);
    self.text_editor.draw(renderer);

    // Draw drag preview
    if let Some(data) = self.drag_drop_state.drag_data() {
        self.draw_drag_preview(renderer, data, self.drag_drop_state.current_position());
    }
}
