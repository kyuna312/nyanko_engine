use super::*;
use std::collections::HashSet;

// Extended TreeView with search, filtering, and multi-selection
impl TreeView {
    pub fn with_multi_select(mut self) -> Self {
        self.multi_select = true;
        self
    }

    pub fn with_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&TreeNode) -> bool + Send + Sync + 'static,
    {
        self.filter = Some(Arc::new(filter));
        self
    }

    pub fn with_search(mut self) -> Self {
        self.search_enabled = true;
        self.search_box = Some(
            TextEditor::new().with_placeholder("Search...").with_style(
                Style::new()
                    .with_background_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
                    .with_border_color(Vec4::new(0.4, 0.4, 0.4, 1.0))
                    .with_border_width(1.0)
                    .with_corner_radius(4.0),
            ),
        );
        self
    }

    pub fn get_selected_nodes(&self) -> Vec<Arc<RwLock<TreeNode>>> {
        let mut selected = Vec::new();
        self.collect_selected_nodes(&self.root, &mut selected);
        selected
    }

    fn collect_selected_nodes(
        &self,
        node: &Arc<RwLock<TreeNode>>,
        selected: &mut Vec<Arc<RwLock<TreeNode>>>,
    ) {
        let node_ref = node.read().unwrap();
        if node_ref.selected {
            selected.push(node.clone());
        }
        for child in &node_ref.children {
            self.collect_selected_nodes(child, selected);
        }
    }

    fn handle_selection(&mut self, node_id: u64, ctx: &UIContext) {
        if self.multi_select && ctx.modifiers.ctrl {
            // Toggle selection
            if let Some(node) = self.find_node_mut(node_id) {
                node.write().unwrap().selected = !node.read().unwrap().selected;
            }
        } else if self.multi_select && ctx.modifiers.shift {
            // Range selection
            if let Some(last_selected) = self.last_selected {
                self.select_range(last_selected, node_id);
            } else {
                if let Some(node) = self.find_node_mut(node_id) {
                    node.write().unwrap().selected = true;
                }
            }
        } else {
            // Single selection
            self.clear_selection();
            if let Some(node) = self.find_node_mut(node_id) {
                node.write().unwrap().selected = true;
            }
        }
        self.last_selected = Some(node_id);
    }

    fn select_range(&mut self, start_id: u64, end_id: u64) {
        let mut nodes_in_range = HashSet::new();
        let mut in_range = false;

        // Collect nodes between start and end
        self.traverse_nodes(&self.root, |node| {
            let node_ref = node.read().unwrap();
            if node_ref.id == start_id || node_ref.id == end_id {
                in_range = !in_range;
                nodes_in_range.insert(node_ref.id);
            } else if in_range {
                nodes_in_range.insert(node_ref.id);
            }
        });

        // Apply selection
        self.traverse_nodes(&self.root, |node| {
            let mut node_ref = node.write().unwrap();
            node_ref.selected = nodes_in_range.contains(&node_ref.id);
        });
    }

    fn update_search(&mut self) {
        if let Some(ref mut search_box) = self.search_box {
            let search_text = search_box.text().to_lowercase();
            if !search_text.is_empty() {
                self.traverse_nodes(&self.root, |node| {
                    let mut node_ref = node.write().unwrap();
                    node_ref.visible = node_ref.text.to_lowercase().contains(&search_text);
                    if node_ref.visible {
                        // Make parent nodes visible
                        let mut parent = node_ref.parent.upgrade();
                        while let Some(p) = parent {
                            p.write().unwrap().visible = true;
                            parent = p.read().unwrap().parent.upgrade();
                        }
                    }
                });
            } else {
                // Reset visibility
                self.traverse_nodes(&self.root, |node| {
                    node.write().unwrap().visible = true;
                });
            }
        }
    }

    fn draw_search_box(&self, renderer: &mut UIRenderer) {
        if let Some(ref search_box) = self.search_box {
            let rect = self.base.rect();
            let search_rect = Rect::new(
                rect.position + Vec2::new(4.0, 4.0),
                Vec2::new(rect.size.x - 8.0, 24.0),
            );
            search_box.draw_at(renderer, search_rect);
        }
    }
}
