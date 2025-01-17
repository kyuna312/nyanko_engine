use super::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MenuBar {
    base: BaseWidget,
    items: Vec<MenuBarItem>,
    active_menu: Option<(usize, ContextMenu)>,
    height: f32,
}

#[derive(Debug, Clone)]
pub struct MenuBarItem {
    text: String,
    menu: Vec<MenuItem>,
    enabled: bool,
}

impl MenuBarItem {
    pub fn new(text: &str, menu: Vec<MenuItem>) -> Self {
        Self {
            text: text.to_string(),
            menu,
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

impl MenuBar {
    pub fn new() -> Self {
        let mut base = BaseWidget::new(WidgetType::Container);
        base.style = Style::new()
            .with_background_color(Vec4::new(0.2, 0.2, 0.2, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(0.0, 0.0, 0.0, 1.0);

        Self {
            base,
            items: Vec::new(),
            active_menu: None,
            height: 28.0,
        }
    }

    pub fn add_menu(&mut self, text: &str, menu: Vec<MenuItem>) -> &mut Self {
        self.items.push(MenuBarItem::new(text, menu));
        self
    }

    fn calculate_item_positions(&self) -> Vec<Rect> {
        let mut positions = Vec::new();
        let mut x = 8.0;
        let rect = self.base.rect();

        for item in &self.items {
            let width = measure_text(&item.text, 14.0).x + 16.0;
            positions.push(Rect::new(
                Vec2::new(rect.position.x + x, rect.position.y),
                Vec2::new(width, self.height),
            ));
            x += width;
        }

        positions
    }

    fn handle_click(&mut self, position: Vec2) -> bool {
        let positions = self.calculate_item_positions();

        // Check if click is on menu bar item
        for (idx, item_rect) in positions.iter().enumerate() {
            if item_rect.contains(position) && self.items[idx].enabled {
                // Close existing menu if clicking different item
                if let Some((current_idx, _)) = &self.active_menu {
                    if *current_idx != idx {
                        self.active_menu = None;
                    }
                }

                // Toggle menu
                if self.active_menu.is_none() {
                    let mut menu = ContextMenu::new(self.items[idx].menu.clone());
                    menu.show_at(Vec2::new(
                        item_rect.position.x,
                        item_rect.position.y + self.height,
                    ));
                    menu.on_close = Some(Box::new(move || {
                        // Handle menu close
                    }));
                    self.active_menu = Some((idx, menu));
                } else {
                    self.active_menu = None;
                }
                return true;
            }
        }

        // Check if click is in active menu
        if let Some((_, ref mut menu)) = &mut self.active_menu {
            if menu.handle_click(position) {
                return true;
            }
        }

        false
    }

    fn handle_hover(&mut self, position: Vec2) {
        let positions = self.calculate_item_positions();

        // Handle hover between menu items when menu is active
        if let Some((current_idx, _)) = self.active_menu {
            for (idx, item_rect) in positions.iter().enumerate() {
                if item_rect.contains(position) && self.items[idx].enabled && idx != current_idx {
                    let mut menu = ContextMenu::new(self.items[idx].menu.clone());
                    menu.show_at(Vec2::new(
                        item_rect.position.x,
                        item_rect.position.y + self.height,
                    ));
                    self.active_menu = Some((idx, menu));
                    break;
                }
            }
        }
    }
}

impl Widget for MenuBar {
    fn update(&mut self, ctx: &mut UIContext) {
        if ctx.pressed {
            if !self.handle_click(ctx.mouse_position) {
                self.active_menu = None;
            }
        } else {
            self.handle_hover(ctx.mouse_position);
        }

        // Update active menu
        if let Some((_, ref mut menu)) = &mut self.active_menu {
            menu.update(ctx);
        }
    }

    fn draw(&self, renderer: &mut UIRenderer) {
        let rect = self.base.rect();

        // Draw background
        renderer.draw_rect(rect, self.base.style.background_color.unwrap_or(Vec4::ZERO));

        // Draw bottom border
        renderer.draw_rect(
            Rect::new(
                Vec2::new(rect.position.x, rect.position.y + rect.size.y - 1.0),
                Vec2::new(rect.size.x, 1.0),
            ),
            self.base.style.border_color.unwrap_or(Vec4::ZERO),
        );

        // Draw menu items
        let positions = self.calculate_item_positions();
        for (idx, (item, item_rect)) in self.items.iter().zip(positions.iter()).enumerate() {
            let is_active = self
                .active_menu
                .as_ref()
                .map_or(false, |(active_idx, _)| *active_idx == idx);
            let is_hovered = item.enabled && item_rect.contains(renderer.mouse_position());

            // Draw item background
            if is_active || is_hovered {
                renderer.draw_rect(*item_rect, Vec4::new(0.3, 0.3, 0.3, 1.0));
            }

            // Draw item text
            renderer.draw_text(
                &item.text,
                item_rect.position + Vec2::new(8.0, 6.0),
                14.0,
                if item.enabled {
                    Vec4::ONE
                } else {
                    Vec4::new(0.5, 0.5, 0.5, 1.0)
                },
            );
        }

        // Draw active menu
        if let Some((_, ref menu)) = &self.active_menu {
            menu.draw(renderer);
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
