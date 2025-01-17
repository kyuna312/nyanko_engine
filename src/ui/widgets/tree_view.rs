use super::*;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct TreeView {
    base: BaseWidget,
    root: Arc<RwLock<TreeNode>>,
    selected_node: Option<u64>,
    scroll_offset: f32,
    item_height: f32,
    indent_width: f32,
    on_select: Option<Box<dyn Fn(&TreeNode) + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    id: u64,
    text: String,
    icon: Option<String>,
    expanded: bool,
    children: Vec<Arc<RwLock<TreeNode>>>,
    data: Option<Arc<dyn std::any::Any + Send + Sync>>,
}

impl TreeView {
    pub fn new() -> Self {
        let mut base = BaseWidget::new(WidgetType::Container);
        base.style = Style::new()
            .with_background_color(Vec4::new(0.15, 0.15, 0.15, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0);

        let root = Arc::new(RwLock::new(TreeNode {
            id: generate_widget_id(),
            text: "Root".to_string(),
            icon: None,
            expanded: true,
            children: Vec::new(),
            data: None,
        }));

        Self {
            base,
            root,
            selected_node: None,
            scroll_offset: 0.0,
            item_height: 24.0,
            indent_width: 20.0,
            on_select: None,
        }
    }

    pub fn with_on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&TreeNode) + Send + Sync + 'static,
    {
        self.on_select = Some(Box::new(callback));
        self
    }

    pub fn add_node(&mut self, parent_id: Option<u64>, text: &str) -> u64 {
        let node = Arc::new(RwLock::new(TreeNode {
            id: generate_widget_id(),
            text: text.to_string(),
            icon: None,
            expanded: false,
            children: Vec::new(),
            data: None,
        }));

        let node_id = node.read().unwrap().id;

        if let Some(parent_id) = parent_id {
            self.find_and_add_child(self.root.clone(), parent_id, node);
        } else {
            self.root.write().unwrap().children.push(node);
        }

        node_id
    }

    fn find_and_add_child(
        &self,
        current: Arc<RwLock<TreeNode>>,
        parent_id: u64,
        new_node: Arc<RwLock<TreeNode>>,
    ) -> bool {
        let current = current.read().unwrap();
        if current.id == parent_id {
            drop(current);
            current.write().unwrap().children.push(new_node);
            true
        } else {
            for child in &current.children {
                if self.find_and_add_child(child.clone(), parent_id, new_node.clone()) {
                    return true;
                }
            }
            false
        }
    }

    fn draw_node(
        &self,
        renderer: &mut UIRenderer,
        node: &TreeNode,
        position: Vec2,
        depth: u32,
        visible_rect: Rect,
    ) -> f32 {
        let indent = depth as f32 * self.indent_width;
        let node_rect = Rect::new(
            Vec2::new(position.x + indent, position.y),
            Vec2::new(visible_rect.size.x - indent, self.item_height),
        );

        // Skip if node is not visible
        if node_rect.position.y + node_rect.size.y < visible_rect.position.y
            || node_rect.position.y > visible_rect.position.y + visible_rect.size.y
        {
            let mut height = self.item_height;
            if node.expanded {
                for child in &node.children {
                    height += self.draw_node(
                        renderer,
                        &child.read().unwrap(),
                        Vec2::new(position.x, position.y + height),
                        depth + 1,
                        visible_rect,
                    );
                }
            }
            return height;
        }

        // Draw selection background
        if Some(node.id) == self.selected_node {
            renderer.draw_rect(node_rect, Vec4::new(0.2, 0.4, 0.8, 0.5));
        }

        // Draw expand/collapse arrow
        if !node.children.is_empty() {
            let arrow = if node.expanded { "▼" } else { "▶" };
            renderer.draw_text(
                arrow,
                Vec2::new(node_rect.position.x - 16.0, node_rect.position.y + 4.0),
                14.0,
                Vec4::ONE,
            );
        }

        // Draw icon if present
        if let Some(icon) = &node.icon {
            renderer.draw_text(
                icon,
                Vec2::new(node_rect.position.x + 4.0, node_rect.position.y + 4.0),
                14.0,
                Vec4::ONE,
            );
        }

        // Draw node text
        renderer.draw_text(
            &node.text,
            Vec2::new(node_rect.position.x + 24.0, node_rect.position.y + 4.0),
            14.0,
            Vec4::ONE,
        );

        let mut height = self.item_height;
        if node.expanded {
            for child in &node.children {
                height += self.draw_node(
                    renderer,
                    &child.read().unwrap(),
                    Vec2::new(position.x, position.y + height),
                    depth + 1,
                    visible_rect,
                );
            }
        }

        height
    }

    fn handle_click(&mut self, position: Vec2, node: Arc<RwLock<TreeNode>>, depth: u32) -> bool {
        let node = node.read().unwrap();
        let indent = depth as f32 * self.indent_width;
        let node_rect = Rect::new(
            Vec2::new(position.x + indent, position.y),
            Vec2::new(self.base.rect().size.x - indent, self.item_height),
        );

        if node_rect.contains(position) {
            // Check if click was on expand/collapse arrow
            if !node.children.is_empty() && position.x < node_rect.position.x + 16.0 {
                drop(node);
                node.write().unwrap().expanded = !node.expanded;
                return true;
            }

            // Handle node selection
            self.selected_node = Some(node.id);
            if let Some(ref callback) = self.on_select {
                callback(&node);
            }
            return true;
        }

        let mut offset = self.item_height;
        if node.expanded {
            for child in &node.children {
                if self.handle_click(
                    Vec2::new(position.x, position.y - offset),
                    child.clone(),
                    depth + 1,
                ) {
                    return true;
                }
                offset += self.calculate_node_height(&child.read().unwrap());
            }
        }

        false
    }

    fn calculate_node_height(&self, node: &TreeNode) -> f32 {
        let mut height = self.item_height;
        if node.expanded {
            for child in &node.children {
                height += self.calculate_node_height(&child.read().unwrap());
            }
        }
        height
    }
}

impl Widget for TreeView {
    fn update(&mut self, ctx: &mut UIContext) {
        let rect = self.base.rect();

        // Handle scrolling
        if ctx.focused_widget == Some(self.id()) {
            let total_height = self.calculate_node_height(&self.root.read().unwrap());
            self.scroll_offset = (self.scroll_offset - ctx.scroll_delta.y)
                .clamp(0.0, (total_height - rect.size.y).max(0.0));
        }

        // Handle clicks
        if ctx.pressed && rect.contains(ctx.mouse_position) {
            let local_pos = ctx.mouse_position - rect.position + Vec2::new(0.0, self.scroll_offset);
            self.handle_click(local_pos, self.root.clone(), 0);
        }
    }

    fn draw(&self, renderer: &mut UIRenderer) {
        let rect = self.base.rect();

        // Draw background
        renderer.draw_rect(rect, self.base.style.background_color.unwrap_or(Vec4::ZERO));

        // Set up clipping rectangle
        renderer.push_clip_rect(rect);

        // Draw tree nodes
        let visible_rect = Rect::new(
            rect.position + Vec2::new(0.0, self.scroll_offset),
            rect.size,
        );
        self.draw_node(
            renderer,
            &self.root.read().unwrap(),
            rect.position - Vec2::new(0.0, self.scroll_offset),
            0,
            visible_rect,
        );

        renderer.pop_clip_rect();

        // Draw scrollbar if needed
        let total_height = self.calculate_node_height(&self.root.read().unwrap());
        if total_height > rect.size.y {
            let scroll_ratio = rect.size.y / total_height;
            let scroll_pos = (self.scroll_offset / total_height) * rect.size.y;
            let scroll_size = rect.size.y * scroll_ratio;

            renderer.draw_rect(
                Rect::new(
                    Vec2::new(
                        rect.position.x + rect.size.x - 8.0,
                        rect.position.y + scroll_pos,
                    ),
                    Vec2::new(8.0, scroll_size),
                ),
                Vec4::new(0.3, 0.3, 0.3, 1.0),
            );
        }
    }

    fn widget_type(&self) -> WidgetType {
        self.base.widget_type()
    }
    fn id(&self) -> u64 {
        self.base.id()
    }
    fn rect(&self) -> Rect {
        self.base.rect()
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.set_rect(rect);
    }
}
