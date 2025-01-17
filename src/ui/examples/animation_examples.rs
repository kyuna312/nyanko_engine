use super::*;

pub struct AnimatedWidgets {
    button: Button,
    progress: ProgressBar,
    slider: Slider,
    notification: Notification,
    animations: HashMap<String, Animation>,
}

impl AnimatedWidgets {
    pub fn new() -> Self {
        Self {
            button: Button::new("Click Me!"),
            progress: ProgressBar::new(0.0),
            slider: Slider::new(0.0, 0.0, 1.0),
            notification: Notification::new("Hello!"),
            animations: HashMap::new(),
        }
    }

    fn animate(&mut self, id: &str, start: f32, end: f32, duration: u64, easing: EasingFunction) {
        self.animations.insert(
            id.to_string(),
            Animation::new(start, end, duration).with_easing(easing),
        );
    }

    fn get_value(&self, id: &str) -> f32 {
        self.animations
            .get(id)
            .map(|anim| anim.value())
            .unwrap_or(0.0)
    }

    pub fn update(&mut self, ctx: &mut UIContext) {
        // Button click animation
        if self.button.clicked(ctx) {
            self.animate(
                "button_scale",
                1.0,
                1.2,
                100,
                EasingFunction::EaseOutElastic,
            );
            self.animate("progress", 0.0, 1.0, 1500, EasingFunction::EaseInOutCubic);
            self.notification.show("Button clicked!");
        }

        // Cleanup finished animations
        self.animations.retain(|_, anim| !anim.is_finished());

        // Update progress bar
        let progress = self.get_value("progress");
        self.progress.set_value(progress);

        // Notification slide animation
        if self.notification.is_visible() {
            if !self.animations.contains_key("notification") {
                self.animate("notification", -50.0, 0.0, 300, EasingFunction::EaseOutQuad);
                self.animate(
                    "notification_fade",
                    0.0,
                    1.0,
                    200,
                    EasingFunction::EaseOutQuad,
                );
            }
        }

        // Slider hover effect
        if self.slider.hovered(ctx) && !self.animations.contains_key("slider_hover") {
            self.animate("slider_hover", 0.0, 1.0, 150, EasingFunction::EaseOutQuad);
        } else if !self.slider.hovered(ctx) && !self.animations.contains_key("slider_hover") {
            self.animate("slider_hover", 1.0, 0.0, 150, EasingFunction::EaseOutQuad);
        }
    }

    pub fn draw(&self, renderer: &mut UIRenderer) {
        // Draw animated button
        let button_scale = self.get_value("button_scale");
        let button_rect = self.button.rect();
        let scaled_rect = button_rect.scaled_from_center(button_scale);

        renderer.draw_rect_rounded(scaled_rect, 4.0, Vec4::new(0.2, 0.6, 1.0, 1.0));

        // Draw progress bar with gradient animation
        let progress = self.get_value("progress");
        let progress_rect = self.progress.rect();

        renderer.draw_gradient(
            progress_rect,
            Vec4::new(0.2, 0.6, 1.0, 1.0),
            Vec4::new(0.4, 0.8, 1.0, 1.0),
            progress,
        );

        // Draw slider with hover effect
        let hover_t = self.get_value("slider_hover");
        let slider_rect = self.slider.rect();
        let handle_color = Vec4::ONE.lerp(Vec4::new(0.2, 0.6, 1.0, 1.0), hover_t);

        renderer.draw_slider(slider_rect, self.slider.value(), handle_color);

        // Draw notification with slide and fade
        if self.notification.is_visible() {
            let offset_y = self.get_value("notification");
            let opacity = self.get_value("notification_fade");
            let notification_rect = self.notification.rect();
            let animated_rect = notification_rect.translated(Vec2::new(0.0, offset_y));

            renderer.draw_notification(
                animated_rect,
                &self.notification.text(),
                Vec4::new(0.2, 0.2, 0.2, opacity),
            );
        }
    }
}

// Helper extension for Rect
trait RectExt {
    fn scaled_from_center(&self, scale: f32) -> Self;
    fn translated(&self, offset: Vec2) -> Self;
}

impl RectExt for Rect {
    fn scaled_from_center(&self, scale: f32) -> Self {
        let center = self.position + self.size * 0.5;
        let scaled_size = self.size * scale;
        Rect::new(center - scaled_size * 0.5, scaled_size)
    }

    fn translated(&self, offset: Vec2) -> Self {
        Rect::new(self.position + offset, self.size)
    }
}
