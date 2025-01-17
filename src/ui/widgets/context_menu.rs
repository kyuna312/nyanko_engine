use super::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ContextMenu {
    base: BaseWidget,
    items: Vec<MenuItem>,
    active_submenu: Option<(usize, Box<ContextMenu>)>,
    parent_position: Option<Vec2>,
    on_close: Option<Box<dyn Fn() + Send + Sync>>,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    text: String,
    icon: Option<String>,
    shortcut: Option<String>,
    enabled: bool,
    submenu: Option<Vec<MenuItem>>,
    action: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl MenuItem {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            icon: None,
            shortcut: None,
            enabled: true,
            submenu: None,
            action: None,
        }
    }

    pub fn separator() -> Self {
        Self {
            text: String::new(),
            icon: None,
            shortcut: None,
            enabled: false,
            submenu: None,
            action: None,
        }
    }

    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    pub fn with_submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.submenu = Some(items);
        self
    }

    pub fn with_action<F>(mut self, action: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.action = Some(Arc::new(action));
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

impl ContextMenu {
    pub fn new(items: Vec<MenuItem>) -> Self {
        let mut base = BaseWidget::new(WidgetType::Popup);
        base.style = Style::new()
            .with_background_color(Vec4::new(0.2, 0.2, 0.2, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0)
            .with_shadow_color(Vec4::new(0.0, 0.0, 0.0, 0.5))
            .with_shadow_offset(Vec2::new(0.0, 2.0))
            .with_shadow_blur(8.0);

        Self {
            base,
            items,
            active_submenu: None,
            parent_position: None,
            on_close: None,
        }
    }

    pub fn show_at(&mut self, position: Vec2) {
        self.base
            .set_rect(Rect::new(position, self.calculate_size()));
        self.parent_position = Some(position);
    }

    fn calculate_size(&self) -> Vec2 {
        let mut width = 0.0;
        let height = self.items.len() as f32 * 24.0;

        for item in &self.items {
            let mut item_width = 24.0; // Icon space
            item_width += measure_text(&item.text, 14.0).x;

            if let Some(shortcut) = &item.shortcut {
                item_width += 40.0 + measure_text(shortcut, 14.0).x;
            }

            if item.submenu.is_some() {
                item_width += 24.0;
            }

            width = width.max(item_width + 16.0);
        }

        Vec2::new(width, height)
    }

    fn handle_hover(&mut self, position: Vec2) -> bool {
        let rect = self.base.rect();
        if !rect.contains(position) {
            if let Some((idx, submenu)) = &mut self.active_submenu {
                if submenu.handle_hover(position) {
                    return true;
                }
                self.active_submenu = None;
            }
            return false;
        }

        let local_y = position.y - rect.position.y;
        let item_idx = (local_y / 24.0) as usize;

        if item_idx < self.items.len() {
            let item = &self.items[item_idx];

            if item.submenu.is_some()
                && self
                    .active_submenu
                    .as_ref()
                    .map_or(true, |(i, _)| *i != item_idx)
            {
                let submenu_pos = Vec2::new(
                    rect.position.x + rect.size.x,
                    rect.position.y + item_idx as f32 * 24.0,
                );
                let mut submenu = Box::new(ContextMenu::new(item.submenu.clone().unwrap()));
                submenu.show_at(submenu_pos);
                self.active_submenu = Some((item_idx, submenu));
            }
        }

        true
    }

    fn handle_click(&mut self, position: Vec2) -> bool {
        if let Some((_, submenu)) = &mut self.active_submenu {
            if submenu.handle_click(position) {
                return true;
            }
        }

        let rect = self.base.rect();
        if !rect.contains(position) {
            return false;
        }

        let local_y = position.y - rect.position.y;
        let item_idx = (local_y / 24.0) as usize;

        if item_idx < self.items.len() {
            let item = &self.items[item_idx];
            if item.enabled && item.submenu.is_none() {
                if let Some(action) = &item.action {
                    action();
                    if let Some(close) = &self.on_close {
                        close();
                    }
                    return true;
                }
            }
        }

        true
    }
}

impl Widget for ContextMenu {
    fn update(&mut self, ctx: &mut UIContext) {
        if ctx.pressed {
            if !self.handle_click(ctx.mouse_position) {
                if let Some(close) = &self.on_close {
                    close();
                }
            }
        } else {
            self.handle_hover(ctx.mouse_position);
        }
    }

    fn draw(&self, renderer: &mut UIRenderer) {
        let rect = self.base.rect();

        // Draw background with shadow
        renderer.draw_rect_with_shadow(
            rect,
            self.base.style.background_color.unwrap_or(Vec4::ZERO),
            self.base.style.shadow_color.unwrap_or(Vec4::ZERO),
            self.base.style.shadow_offset.unwrap_or(Vec2::ZERO),
            self.base.style.shadow_blur.unwrap_or(0.0),
        );

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_rect = Rect::new(
                Vec2::new(rect.position.x, rect.position.y + i as f32 * 24.0),
                Vec2::new(rect.size.x, 24.0),
            );

            if item.text.is_empty() {
                // Separator
                renderer.draw_rect(
                    Rect::new(
                        Vec2::new(item_rect.position.x + 4.0, item_rect.position.y + 11.0),
                        Vec2::new(item_rect.size.x - 8.0, 1.0),
                    ),
                    Vec4::new(0.3, 0.3, 0.3, 1.0),
                );
                continue;
            }

            // Draw hover highlight
            if item.enabled && item_rect.contains(renderer.mouse_position()) {
                renderer.draw_rect(item_rect, Vec4::new(0.3, 0.3, 0.3, 1.0));
            }

            // Draw icon
            if let Some(icon) = &item.icon {
                renderer.draw_text(
                    icon,
                    item_rect.position + Vec2::new(4.0, 4.0),
                    14.0,
                    if item.enabled {
                        Vec4::ONE
                    } else {
                        Vec4::new(0.5, 0.5, 0.5, 1.0)
                    },
                );
            }

            // Draw text
            renderer.draw_text(
                &item.text,
                item_rect.position + Vec2::new(24.0, 4.0),
                14.0,
                if item.enabled {
                    Vec4::ONE
                } else {
                    Vec4::new(0.5, 0.5, 0.5, 1.0)
                },
            );

            // Draw shortcut
            if let Some(shortcut) = &item.shortcut {
                renderer.draw_text(
                    shortcut,
                    Vec2::new(
                        item_rect.position.x + rect.size.x - measure_text(shortcut, 14.0).x - 8.0,
                        item_rect.position.y + 4.0,
                    ),
                    14.0,
                    if item.enabled {
                        Vec4::new(0.7, 0.7, 0.7, 1.0)
                    } else {
                        Vec4::new(0.4, 0.4, 0.4, 1.0)
                    },
                );
            }

            // Draw submenu arrow
            if item.submenu.is_some() {
                renderer.draw_text(
                    "▶",
                    Vec2::new(
                        item_rect.position.x + rect.size.x - 16.0,
                        item_rect.position.y + 4.0,
                    ),
                    14.0,
                    if item.enabled {
                        Vec4::ONE
                    } else {
                        Vec4::new(0.5, 0.5, 0.5, 1.0)
                    },
                );
            }
        }

        // Draw active submenu
        if let Some((_, submenu)) = &self.active_submenu {
            submenu.draw(renderer);
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
