use super::*;
use std::collections::HashMap;

// Custom node rendering and interaction support
impl TreeView {
    pub fn with_custom_renderer<F>(mut self, renderer: F) -> Self
    where
        F: Fn(&TreeNode, &mut UIRenderer, Rect, bool, bool) + Send + Sync + 'static,
    {
        self.custom_renderer = Some(Arc::new(renderer));
        self
    }

    pub fn with_node_tooltip<F>(mut self, tooltip: F) -> Self
    where
        F: Fn(&TreeNode) -> String + Send + Sync + 'static,
    {
        self.tooltip_provider = Some(Arc::new(tooltip));
        self
    }

    pub fn with_context_menu<F>(mut self, menu_builder: F) -> Self
    where
        F: Fn(&TreeNode) -> Vec<MenuItem> + Send + Sync + 'static,
    {
        self.context_menu_builder = Some(Arc::new(menu_builder));
        self
    }

    // Example custom renderers
    pub fn file_system_renderer() -> impl Fn(&TreeNode, &mut UIRenderer, Rect, bool, bool) {
        |node, renderer, rect, selected, hovered| {
            let icon = if node.is_folder() { "📁" } else { "📄" };
            let color = if selected {
                Vec4::new(0.2, 0.6, 1.0, 1.0)
            } else if hovered {
                Vec4::new(0.7, 0.7, 0.7, 1.0)
            } else {
                Vec4::ONE
            };

            // Draw background
            if selected || hovered {
                renderer.draw_rect(
                    rect,
                    Vec4::new(0.2, 0.2, 0.2, if selected { 0.8 } else { 0.4 }),
                );
            }

            // Draw icon
            renderer.draw_text(icon, rect.position + Vec2::new(4.0, 4.0), 14.0, color);

            // Draw text
            renderer.draw_text(
                &node.text,
                rect.position + Vec2::new(24.0, 4.0),
                14.0,
                color,
            );

            // Draw file size or item count for folders
            if let Some(metadata) = node.data.downcast_ref::<FileMetadata>() {
                let info_text = if node.is_folder() {
                    format!("{} items", metadata.item_count)
                } else {
                    format_size(metadata.size)
                };

                renderer.draw_text(
                    &info_text,
                    Vec2::new(
                        rect.position.x + rect.size.x - measure_text(&info_text, 12.0).x - 8.0,
                        rect.position.y + 6.0,
                    ),
                    12.0,
                    Vec4::new(0.7, 0.7, 0.7, 1.0),
                );
            }
        }
    }

    pub fn process_tree_renderer() -> impl Fn(&TreeNode, &mut UIRenderer, Rect, bool, bool) {
        |node, renderer, rect, selected, hovered| {
            if let Some(process) = node.data.downcast_ref::<ProcessInfo>() {
                // Draw status indicator
                let status_color = match process.status {
                    ProcessStatus::Running => Vec4::new(0.2, 0.8, 0.2, 1.0),
                    ProcessStatus::Stopped => Vec4::new(0.8, 0.2, 0.2, 1.0),
                    ProcessStatus::Suspended => Vec4::new(0.8, 0.8, 0.2, 1.0),
                };

                renderer.draw_circle(
                    rect.position + Vec2::new(8.0, rect.size.y * 0.5),
                    4.0,
                    status_color,
                );

                // Draw process info
                let text_color = if selected {
                    Vec4::new(0.2, 0.6, 1.0, 1.0)
                } else if hovered {
                    Vec4::new(0.7, 0.7, 0.7, 1.0)
                } else {
                    Vec4::ONE
                };

                renderer.draw_text(
                    &format!("{} ({})", node.text, process.pid),
                    rect.position + Vec2::new(20.0, 4.0),
                    14.0,
                    text_color,
                );

                // Draw CPU and memory usage
                let usage_text = format!(
                    "CPU: {:.1}% | Memory: {}",
                    process.cpu_usage,
                    format_size(process.memory_usage),
                );

                renderer.draw_text(
                    &usage_text,
                    Vec2::new(
                        rect.position.x + rect.size.x - measure_text(&usage_text, 12.0).x - 8.0,
                        rect.position.y + 6.0,
                    ),
                    12.0,
                    Vec4::new(0.7, 0.7, 0.7, 1.0),
                );
            }
        }
    }

    // Helper functions
    fn format_size(size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = size as f64;
        let mut unit = 0;

        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }

        format!("{:.1} {}", size, UNITS[unit])
    }

    fn show_tooltip(&self, node: &TreeNode, position: Vec2) {
        if let Some(ref provider) = self.tooltip_provider {
            let tooltip_text = provider(node);
            if !tooltip_text.is_empty() {
                self.tooltip_manager.show(tooltip_text, position);
            }
        }
    }

    fn show_context_menu(&mut self, node: &TreeNode, position: Vec2) {
        if let Some(ref builder) = self.context_menu_builder {
            let menu_items = builder(node);
            if !menu_items.is_empty() {
                let mut menu = ContextMenu::new(menu_items);
                menu.show_at(position);
                self.context_menu = Some(menu);
            }
        }
    }
}
