use super::*;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Animation {
    start_value: f32,
    end_value: f32,
    duration: Duration,
    start_time: Instant,
    easing: EasingFunction,
}

#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    Spring,
}

impl Animation {
    pub fn new(start: f32, end: f32, duration_ms: u64) -> Self {
        Self {
            start_value: start,
            end_value: end,
            duration: Duration::from_millis(duration_ms),
            start_time: Instant::now(),
            easing: EasingFunction::EaseOutQuad,
        }
    }

    pub fn with_easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    pub fn value(&self) -> f32 {
        let elapsed = self.start_time.elapsed();
        if elapsed >= self.duration {
            return self.end_value;
        }

        let t = elapsed.as_secs_f32() / self.duration.as_secs_f32();
        let eased_t = self.easing.apply(t);
        self.start_value + (self.end_value - self.start_value) * eased_t
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed() >= self.duration
    }
}

impl EasingFunction {
    fn apply(&self, t: f32) -> f32 {
        match self {
            Self::Linear => t,
            Self::EaseInQuad => t * t,
            Self::EaseOutQuad => -t * (t - 2.0),
            Self::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::EaseInCubic => t * t * t,
            Self::EaseOutCubic => {
                let t = t - 1.0;
                t * t * t + 1.0
            }
            Self::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    let t = t - 1.0;
                    1.0 + 4.0 * t * t * t
                }
            }
            Self::EaseInElastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -(2.0f32.powf(10.0 * t - 10.0) * (t * 10.0 - 10.75).sin() * c4)
                }
            }
            Self::EaseOutElastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2.0f32.powf(-10.0 * t) * (t * 10.0 - 0.75).sin() * c4 + 1.0
                }
            }
            Self::EaseInOutElastic => {
                let c5 = (2.0 * std::f32::consts::PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2.0f32.powf(20.0 * t - 10.0) * (20.0 * t - 11.125).sin() * c5) / 2.0
                } else {
                    (2.0f32.powf(-20.0 * t + 10.0) * (20.0 * t - 11.125).sin() * c5) / 2.0 + 1.0
                }
            }
            Self::Spring => {
                let c = 3.0;
                let d = 0.5;
                let n = 3.0;
                1.0 - (-t * n * std::f32::consts::PI).exp() * (t * c).cos() * d
            }
        }
    }
}

// Example usage in widgets
impl MenuItem {
    fn animate_hover(&mut self) {
        self.hover_animation =
            Some(Animation::new(0.0, 1.0, 150).with_easing(EasingFunction::EaseOutQuad));
    }

    fn draw_with_animation(&self, renderer: &mut UIRenderer, style: &MenuStyle) {
        let hover_t = self
            .hover_animation
            .as_ref()
            .map_or(if self.hovered { 1.0 } else { 0.0 }, |anim| anim.value());

        let background = style.background.lerp(style.hover_background, hover_t);
        renderer.draw_rect(self.rect(), background);
    }
}

impl TreeNode {
    fn animate_expand(&mut self) {
        self.expand_animation =
            Some(Animation::new(0.0, 1.0, 200).with_easing(EasingFunction::Spring));
    }

    fn draw_with_animation(&self, renderer: &mut UIRenderer) {
        let expand_t = self
            .expand_animation
            .as_ref()
            .map_or(if self.expanded { 1.0 } else { 0.0 }, |anim| anim.value());

        let children_height = self
            .children
            .iter()
            .map(|child| child.rect().size.y)
            .sum::<f32>();

        renderer.push_clip_rect(Rect::new(
            self.rect().position,
            Vec2::new(
                self.rect().size.x,
                self.rect().size.y + children_height * expand_t,
            ),
        ));

        // Draw children...

        renderer.pop_clip_rect();
    }
}
