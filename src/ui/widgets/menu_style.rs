use super::*;

#[derive(Debug, Clone)]
pub struct MenuStyle {
    pub background: Vec4,
    pub hover_background: Vec4,
    pub active_background: Vec4,
    pub text_color: Vec4,
    pub disabled_text_color: Vec4,
    pub separator_color: Vec4,
    pub border_color: Vec4,
    pub shadow_color: Vec4,
    pub corner_radius: f32,
    pub item_height: f32,
    pub padding: Vec2,
    pub separator_height: f32,
    pub shadow_offset: Vec2,
    pub shadow_blur: f32,
    pub icon_size: f32,
    pub submenu_arrow: String,
}

impl Default for MenuStyle {
    fn default() -> Self {
        Self {
            background: Vec4::new(0.2, 0.2, 0.2, 0.95),
            hover_background: Vec4::new(0.3, 0.3, 0.3, 1.0),
            active_background: Vec4::new(0.4, 0.4, 0.4, 1.0),
            text_color: Vec4::ONE,
            disabled_text_color: Vec4::new(0.5, 0.5, 0.5, 1.0),
            separator_color: Vec4::new(0.3, 0.3, 0.3, 1.0),
            border_color: Vec4::new(0.4, 0.4, 0.4, 1.0),
            shadow_color: Vec4::new(0.0, 0.0, 0.0, 0.5),
            corner_radius: 4.0,
            item_height: 24.0,
            padding: Vec2::new(8.0, 4.0),
            separator_height: 1.0,
            shadow_offset: Vec2::new(0.0, 2.0),
            shadow_blur: 8.0,
            icon_size: 16.0,
            submenu_arrow: "▶".to_string(),
        }
    }
}

impl MenuStyle {
    pub fn dark() -> Self {
        Self::default()
    }

    pub fn light() -> Self {
        Self {
            background: Vec4::new(0.95, 0.95, 0.95, 0.95),
            hover_background: Vec4::new(0.9, 0.9, 0.9, 1.0),
            active_background: Vec4::new(0.85, 0.85, 0.85, 1.0),
            text_color: Vec4::ZERO,
            disabled_text_color: Vec4::new(0.6, 0.6, 0.6, 1.0),
            separator_color: Vec4::new(0.8, 0.8, 0.8, 1.0),
            border_color: Vec4::new(0.7, 0.7, 0.7, 1.0),
            shadow_color: Vec4::new(0.0, 0.0, 0.0, 0.2),
            ..Default::default()
        }
    }

    pub fn accent(color: Vec4) -> Self {
        Self {
            hover_background: color * Vec4::new(1.0, 1.0, 1.0, 0.2),
            active_background: color * Vec4::new(1.0, 1.0, 1.0, 0.3),
            border_color: color * Vec4::new(1.0, 1.0, 1.0, 0.5),
            ..Default::default()
        }
    }
}

impl ContextMenu {
    fn draw_styled(&self, renderer: &mut UIRenderer, style: &MenuStyle) {
        let rect = self.rect();

        // Draw shadow
        if style.shadow_blur > 0.0 {
            renderer.draw_shadow(
                rect,
                style.corner_radius,
                style.shadow_color,
                style.shadow_offset,
                style.shadow_blur,
            );
        }

        // Draw background with rounded corners
        renderer.draw_rect_rounded(rect, style.corner_radius, style.background);

        // Draw border
        renderer.draw_rect_rounded_outline(rect, style.corner_radius, style.border_color, 1.0);

        // Draw menu items
        let mut y = rect.position.y + style.padding.y;
        for item in &self.items {
            if item.is_separator() {
                // Draw separator
                let separator_y = y + style.item_height * 0.5;
                renderer.draw_line(
                    Vec2::new(rect.position.x + style.padding.x, separator_y),
                    Vec2::new(rect.position.x + rect.size.x - style.padding.x, separator_y),
                    style.separator_height,
                    style.separator_color,
                );
                y += style.item_height;
                continue;
            }

            let item_rect = Rect::new(
                Vec2::new(rect.position.x, y),
                Vec2::new(rect.size.x, style.item_height),
            );

            // Draw item background
            let bg_color = if item.active {
                style.active_background
            } else if item.hovered {
                style.hover_background
            } else {
                Vec4::ZERO
            };

            if bg_color.w > 0.0 {
                renderer.draw_rect(item_rect, bg_color);
            }

            // Draw icon if present
            let mut text_x = rect.position.x + style.padding.x;
            if let Some(ref icon) = item.icon {
                renderer.draw_text(
                    icon,
                    Vec2::new(text_x, y + (style.item_height - style.icon_size) * 0.5),
                    style.icon_size,
                    if item.enabled {
                        style.text_color
                    } else {
                        style.disabled_text_color
                    },
                );
                text_x += style.icon_size + style.padding.x;
            }

            // Draw text
            renderer.draw_text(
                &item.text,
                Vec2::new(text_x, y + (style.item_height - 14.0) * 0.5),
                14.0,
                if item.enabled {
                    style.text_color
                } else {
                    style.disabled_text_color
                },
            );

            // Draw shortcut if present
            if let Some(ref shortcut) = item.shortcut {
                renderer.draw_text(
                    shortcut,
                    Vec2::new(
                        rect.position.x + rect.size.x
                            - style.padding.x
                            - measure_text(shortcut, 12.0).x,
                        y + (style.item_height - 12.0) * 0.5,
                    ),
                    12.0,
                    if item.enabled {
                        style.disabled_text_color
                    } else {
                        style.disabled_text_color * 0.7
                    },
                );
            }

            // Draw submenu arrow
            if item.has_submenu() {
                renderer.draw_text(
                    &style.submenu_arrow,
                    Vec2::new(
                        rect.position.x + rect.size.x - style.padding.x - 16.0,
                        y + (style.item_height - 14.0) * 0.5,
                    ),
                    14.0,
                    if item.enabled {
                        style.text_color
                    } else {
                        style.disabled_text_color
                    },
                );
            }

            y += style.item_height;
        }
    }
}
