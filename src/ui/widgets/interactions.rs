use super::*;
use std::collections::HashMap;

pub struct InteractionState {
    focused_widget: Option<WidgetId>,
    hover_animations: HashMap<WidgetId, Animation>,
    press_animations: HashMap<WidgetId, Animation>,
    focus_animations: HashMap<WidgetId, Animation>,
    ripples: Vec<Ripple>,
    tooltips: HashMap<WidgetId, TooltipState>,
}

#[derive(Debug)]
struct Ripple {
    center: Vec2,
    start_time: Instant,
    color: Vec4,
    duration: Duration,
}

#[derive(Debug)]
struct TooltipState {
    text: String,
    position: Vec2,
    animation: Animation,
    delay_timer: Option<Instant>,
}

impl InteractionState {
    pub fn new() -> Self {
        Self {
            focused_widget: None,
            hover_animations: HashMap::new(),
            press_animations: HashMap::new(),
            focus_animations: HashMap::new(),
            ripples: Vec::new(),
            tooltips: HashMap::new(),
        }
    }

    pub fn handle_interaction(&mut self, widget: &mut dyn Widget, ctx: &UIContext) {
        let id = widget.id();

        // Handle hover state
        if widget.rect().contains(ctx.mouse_position) {
            if !self.hover_animations.contains_key(&id) {
                self.hover_animations.insert(
                    id,
                    Animation::new(0.0, 1.0, 150).with_easing(EasingFunction::EaseOutQuad),
                );
            }

            // Show tooltip after delay
            if let Some(tooltip) = widget.tooltip() {
                self.tooltips.entry(id).or_insert_with(|| TooltipState {
                    text: tooltip.to_string(),
                    position: ctx.mouse_position + Vec2::new(10.0, 10.0),
                    animation: Animation::new(0.0, 1.0, 200)
                        .with_easing(EasingFunction::EaseOutQuad),
                    delay_timer: Some(Instant::now()),
                });
            }
        } else {
            if self.hover_animations.contains_key(&id) {
                self.hover_animations.insert(
                    id,
                    Animation::new(1.0, 0.0, 150).with_easing(EasingFunction::EaseOutQuad),
                );
            }
            self.tooltips.remove(&id);
        }

        // Handle press state
        if ctx.pressed && widget.rect().contains(ctx.mouse_position) {
            self.press_animations.insert(
                id,
                Animation::new(0.0, 1.0, 100).with_easing(EasingFunction::EaseOutQuad),
            );

            // Create ripple effect
            self.ripples.push(Ripple {
                center: ctx.mouse_position,
                start_time: Instant::now(),
                color: Vec4::new(1.0, 1.0, 1.0, 0.3),
                duration: Duration::from_millis(600),
            });
        }

        // Handle focus state
        if ctx.focused_widget == Some(id) && self.focused_widget != Some(id) {
            self.focused_widget = Some(id);
            self.focus_animations.insert(
                id,
                Animation::new(0.0, 1.0, 200).with_easing(EasingFunction::EaseOutQuad),
            );
        } else if ctx.focused_widget != Some(id) && self.focused_widget == Some(id) {
            self.focused_widget = None;
            self.focus_animations.insert(
                id,
                Animation::new(1.0, 0.0, 200).with_easing(EasingFunction::EaseOutQuad),
            );
        }
    }

    pub fn draw_interactions(&self, widget: &dyn Widget, renderer: &mut UIRenderer) {
        let id = widget.id();
        let rect = widget.rect();

        // Draw hover effect
        if let Some(anim) = self.hover_animations.get(&id) {
            let hover_alpha = anim.value();
            renderer.draw_rect(rect, Vec4::new(1.0, 1.0, 1.0, 0.1 * hover_alpha));
        }

        // Draw press effect
        if let Some(anim) = self.press_animations.get(&id) {
            let press_alpha = anim.value();
            renderer.draw_rect(rect, Vec4::new(0.0, 0.0, 0.0, 0.1 * press_alpha));
        }

        // Draw focus ring
        if let Some(anim) = self.focus_animations.get(&id) {
            let focus_alpha = anim.value();
            renderer.draw_rect_outline(
                rect.expanded(2.0),
                Vec4::new(0.2, 0.6, 1.0, focus_alpha),
                2.0,
            );
        }

        // Draw ripples
        for ripple in &self.ripples {
            let elapsed = ripple.start_time.elapsed().as_secs_f32();
            let progress = elapsed / ripple.duration.as_secs_f32();
            if progress < 1.0 {
                let radius = progress * rect.size.length() * 1.5;
                let alpha = (1.0 - progress) * ripple.color.w;
                renderer.draw_circle(ripple.center, radius, ripple.color.with_alpha(alpha));
            }
        }

        // Draw tooltip
        if let Some(tooltip) = self.tooltips.get(&id) {
            if let Some(delay) = tooltip.delay_timer {
                if delay.elapsed() > Duration::from_millis(500) {
                    let alpha = tooltip.animation.value();
                    renderer.draw_tooltip(
                        tooltip.position,
                        &tooltip.text,
                        Vec4::new(0.1, 0.1, 0.1, 0.9 * alpha),
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        // Remove finished animations
        self.hover_animations.retain(|_, anim| !anim.is_finished());
        self.press_animations.retain(|_, anim| !anim.is_finished());
        self.focus_animations.retain(|_, anim| !anim.is_finished());

        // Remove finished ripples
        self.ripples
            .retain(|ripple| ripple.start_time.elapsed() < ripple.duration);
    }
}
