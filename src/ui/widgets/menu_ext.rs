use super::*;
use std::collections::HashMap;

// Extended menu functionality
impl MenuItem {
    pub fn with_mnemonic(mut self, mnemonic: char) -> Self {
        let pos = self
            .text
            .find(mnemonic)
            .expect("Mnemonic character must exist in menu item text");
        self.mnemonic = Some((mnemonic, pos));
        self
    }

    pub fn dynamic<F>(text: &str, builder: F) -> Self
    where
        F: Fn() -> Vec<MenuItem> + Send + Sync + 'static,
    {
        Self {
            text: text.to_string(),
            submenu: None,
            dynamic_builder: Some(Arc::new(builder)),
            ..Default::default()
        }
    }
}

impl MenuBar {
    fn draw_item_with_mnemonic(
        &self,
        renderer: &mut UIRenderer,
        text: &str,
        mnemonic: Option<(char, usize)>,
        rect: Rect,
        color: Vec4,
    ) {
        if let Some((_, pos)) = mnemonic {
            // Draw text before mnemonic
            if pos > 0 {
                renderer.draw_text(
                    &text[..pos],
                    rect.position + Vec2::new(8.0, 6.0),
                    14.0,
                    color,
                );
            }

            // Draw underlined mnemonic
            renderer.draw_text(
                &text[pos..=pos],
                rect.position + Vec2::new(8.0 + measure_text(&text[..pos], 14.0).x, 6.0),
                14.0,
                color,
            );
            renderer.draw_line(
                rect.position + Vec2::new(8.0 + measure_text(&text[..=pos], 14.0).x - 2.0, 22.0),
                rect.position + Vec2::new(8.0 + measure_text(&text[..=pos], 14.0).x + 2.0, 22.0),
                1.0,
                color,
            );

            // Draw remaining text
            if pos < text.len() - 1 {
                renderer.draw_text(
                    &text[pos + 1..],
                    rect.position + Vec2::new(8.0 + measure_text(&text[..=pos], 14.0).x, 6.0),
                    14.0,
                    color,
                );
            }
        } else {
            renderer.draw_text(text, rect.position + Vec2::new(8.0, 6.0), 14.0, color);
        }
    }

    fn handle_mnemonic(&mut self, key: char) -> bool {
        if !self.active_menu.is_some() {
            // Check top-level menu items
            for (idx, item) in self.items.iter().enumerate() {
                if let Some((mnemonic, _)) = item.mnemonic {
                    if mnemonic.to_ascii_lowercase() == key.to_ascii_lowercase() {
                        self.activate_menu(idx);
                        return true;
                    }
                }
            }
        } else if let Some((_, ref mut menu)) = &mut self.active_menu {
            // Check submenu items
            return menu.handle_mnemonic(key);
        }
        false
    }

    fn build_dynamic_menu(&self, item: &MenuItem) -> Vec<MenuItem> {
        if let Some(ref builder) = item.dynamic_builder {
            builder()
        } else {
            item.submenu.clone().unwrap_or_default()
        }
    }

    // Recent files example
    pub fn create_recent_files_menu() -> MenuItem {
        MenuItem::dynamic("Recent Files", || {
            let mut items = Vec::new();

            // Read recent files from configuration
            if let Ok(recent_files) = config::get_recent_files() {
                for path in recent_files {
                    items.push(MenuItem::new(&path.to_string_lossy()).with_action(move || {
                        // Open file action
                    }));
                }

                if !items.is_empty() {
                    items.push(MenuItem::separator());
                }
            }

            items.push(MenuItem::new("Clear Recent Files").with_action(|| {
                config::clear_recent_files();
            }));

            items
        })
    }

    // Window list example
    pub fn create_window_menu() -> MenuItem {
        MenuItem::dynamic("Window", || {
            let mut items = Vec::new();

            // Get list of open windows
            for window in application::get_windows() {
                items.push(MenuItem::new(&window.title).with_action(move || {
                    window.focus();
                }));
            }

            items
        })
    }
}

impl ContextMenu {
    fn handle_mnemonic(&mut self, key: char) -> bool {
        for item in &self.items {
            if let Some((mnemonic, _)) = item.mnemonic {
                if mnemonic.to_ascii_lowercase() == key.to_ascii_lowercase() {
                    if let Some(ref action) = item.action {
                        action();
                        return true;
                    }
                    if item.submenu.is_some() || item.dynamic_builder.is_some() {
                        // Open submenu
                        return true;
                    }
                }
            }
        }
        false
    }
}
