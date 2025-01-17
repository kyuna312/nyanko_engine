use super::*;

#[derive(Debug, Clone)]
pub struct ColorPicker {
    base: BaseWidget,
    color: Vec4,
    hsv: HSV,
    dragging_hue: bool,
    dragging_sv: bool,
    on_change: Option<Box<dyn Fn(Vec4) + Send + Sync>>,
}

#[derive(Debug, Clone, Copy)]
struct HSV {
    hue: f32,        // 0-360
    saturation: f32, // 0-1
    value: f32,      // 0-1
    alpha: f32,      // 0-1
}

impl ColorPicker {
    pub fn new(initial_color: Vec4) -> Self {
        let mut base = BaseWidget::new(WidgetType::Container);
        base.style = Style::new()
            .with_background_color(Vec4::new(0.2, 0.2, 0.2, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0)
            .with_padding(Vec2::new(8.0, 8.0));

        let hsv = Self::rgb_to_hsv(initial_color);

        Self {
            base,
            color: initial_color,
            hsv,
            dragging_hue: false,
            dragging_sv: false,
            on_change: None,
        }
    }

    pub fn with_on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(Vec4) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(callback));
        self
    }

    fn update_color(&mut self) {
        let old_color = self.color;
        self.color = Self::hsv_to_rgb(self.hsv);

        if self.color != old_color {
            if let Some(ref callback) = self.on_change {
                callback(self.color);
            }
        }
    }

    fn rgb_to_hsv(rgb: Vec4) -> HSV {
        let max = rgb.x.max(rgb.y).max(rgb.z);
        let min = rgb.x.min(rgb.y).min(rgb.z);
        let delta = max - min;

        let hue = if delta == 0.0 {
            0.0
        } else if max == rgb.x {
            60.0 * (((rgb.y - rgb.z) / delta) % 6.0)
        } else if max == rgb.y {
            60.0 * ((rgb.z - rgb.x) / delta + 2.0)
        } else {
            60.0 * ((rgb.x - rgb.y) / delta + 4.0)
        };

        let saturation = if max == 0.0 { 0.0 } else { delta / max };

        HSV {
            hue: if hue < 0.0 { hue + 360.0 } else { hue },
            saturation,
            value: max,
            alpha: rgb.w,
        }
    }

    fn hsv_to_rgb(hsv: HSV) -> Vec4 {
        let c = hsv.value * hsv.saturation;
        let h = hsv.hue / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0).abs());
        let m = hsv.value - c;

        let (r, g, b) = match h as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Vec4::new(r + m, g + m, b + m, hsv.alpha)
    }

    fn draw_color_plane(&self, renderer: &mut UIRenderer, rect: Rect) {
        // Draw saturation-value plane with current hue
        let steps = 32;
        for y in 0..steps {
            for x in 0..steps {
                let s = x as f32 / (steps - 1) as f32;
                let v = 1.0 - (y as f32 / (steps - 1) as f32);

                let color = Self::hsv_to_rgb(HSV {
                    hue: self.hsv.hue,
                    saturation: s,
                    value: v,
                    alpha: 1.0,
                });

                let pixel_rect = Rect::new(
                    Vec2::new(
                        rect.position.x + rect.size.x * (x as f32 / steps as f32),
                        rect.position.y + rect.size.y * (y as f32 / steps as f32),
                    ),
                    Vec2::new(rect.size.x / steps as f32, rect.size.y / steps as f32),
                );

                renderer.draw_rect(pixel_rect, color);
            }
        }

        // Draw current S/V position
        let pos = Vec2::new(
            rect.position.x + rect.size.x * self.hsv.saturation,
            rect.position.y + rect.size.y * (1.0 - self.hsv.value),
        );
        renderer.draw_circle(pos, 4.0, Vec4::new(1.0, 1.0, 1.0, 1.0));
        renderer.draw_circle(pos, 3.0, Vec4::new(0.0, 0.0, 0.0, 1.0));
    }

    fn draw_hue_slider(&self, renderer: &mut UIRenderer, rect: Rect) {
        // Draw hue gradient
        let steps = 32;
        for i in 0..steps {
            let hue = 360.0 * (i as f32 / (steps - 1) as f32);
            let color = Self::hsv_to_rgb(HSV {
                hue,
                saturation: 1.0,
                value: 1.0,
                alpha: 1.0,
            });

            let segment_rect = Rect::new(
                Vec2::new(
                    rect.position.x,
                    rect.position.y + rect.size.y * (i as f32 / steps as f32),
                ),
                Vec2::new(rect.size.x, rect.size.y / steps as f32 + 1.0),
            );

            renderer.draw_rect(segment_rect, color);
        }

        // Draw current hue position
        let pos = Vec2::new(
            rect.position.x + rect.size.x * 0.5,
            rect.position.y + rect.size.y * (self.hsv.hue / 360.0),
        );
        renderer.draw_rect(
            Rect::new(
                Vec2::new(rect.position.x, pos.y - 2.0),
                Vec2::new(rect.size.x, 4.0),
            ),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
        );
    }
}

impl Widget for ColorPicker {
    fn update(&mut self, ctx: &mut UIContext) {
        let rect = self.base.rect();
        let color_plane_rect = Rect::new(
            rect.position + Vec2::new(8.0, 8.0),
            Vec2::new(rect.size.x - 40.0, rect.size.y - 16.0),
        );
        let hue_slider_rect = Rect::new(
            Vec2::new(rect.position.x + rect.size.x - 24.0, rect.position.y + 8.0),
            Vec2::new(16.0, rect.size.y - 16.0),
        );

        if ctx.pressed {
            if color_plane_rect.contains(ctx.mouse_position) {
                self.dragging_sv = true;
            } else if hue_slider_rect.contains(ctx.mouse_position) {
                self.dragging_hue = true;
            }
        } else {
            self.dragging_sv = false;
            self.dragging_hue = false;
        }

        if self.dragging_sv {
            let local_pos =
                (ctx.mouse_position - color_plane_rect.position) / color_plane_rect.size;
            self.hsv.saturation = local_pos.x.clamp(0.0, 1.0);
            self.hsv.value = 1.0 - local_pos.y.clamp(0.0, 1.0);
            self.update_color();
        }

        if self.dragging_hue {
            let local_y =
                (ctx.mouse_position.y - hue_slider_rect.position.y) / hue_slider_rect.size.y;
            self.hsv.hue = (local_y.clamp(0.0, 1.0) * 360.0);
            self.update_color();
        }
    }

    fn draw(&self, renderer: &mut UIRenderer) {
        let rect = self.base.rect();

        // Draw background
        renderer.draw_rect(rect, self.base.style.background_color.unwrap_or(Vec4::ZERO));

        // Draw color plane
        let color_plane_rect = Rect::new(
            rect.position + Vec2::new(8.0, 8.0),
            Vec2::new(rect.size.x - 40.0, rect.size.y - 16.0),
        );
        self.draw_color_plane(renderer, color_plane_rect);

        // Draw hue slider
        let hue_slider_rect = Rect::new(
            Vec2::new(rect.position.x + rect.size.x - 24.0, rect.position.y + 8.0),
            Vec2::new(16.0, rect.size.y - 16.0),
        );
        self.draw_hue_slider(renderer, hue_slider_rect);

        // Draw current color preview
        let preview_rect = Rect::new(
            Vec2::new(rect.position.x + 8.0, rect.position.y + rect.size.y - 40.0),
            Vec2::new(40.0, 24.0),
        );
        renderer.draw_rect(preview_rect, self.color);
    }

    // Implement other Widget trait methods
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
