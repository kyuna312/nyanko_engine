use super::*;
use std::fmt::Write;

pub struct UIDebugger {
    show_bounds: bool,
    show_layout: bool,
    show_hierarchy: bool,
    show_metrics: bool,
    metrics: UIMetrics,
}

#[derive(Debug, Default)]
struct UIMetrics {
    widget_count: usize,
    draw_calls: usize,
    vertices: usize,
    update_time: f32,
    render_time: f32,
    frame_time: std::time::Instant,
}

impl UIDebugger {
    pub fn new() -> Self {
        Self {
            show_bounds: false,
            show_layout: false,
            show_hierarchy: false,
            show_metrics: true,
            metrics: UIMetrics::default(),
        }
    }

    pub fn begin_frame(&mut self) {
        self.metrics.frame_time = std::time::Instant::now();
        self.metrics.draw_calls = 0;
        self.metrics.vertices = 0;
    }

    pub fn end_frame(&mut self) {
        self.metrics.frame_time = std::time::Instant::now();
    }

    pub fn draw_debug_overlay(&self, root: &dyn Widget, renderer: &mut UIRenderer) {
        if self.show_bounds {
            self.draw_widget_bounds(root, renderer);
        }

        if self.show_layout {
            self.draw_layout_guides(root, renderer);
        }

        if self.show_hierarchy {
            self.draw_widget_hierarchy(root, renderer);
        }

        if self.show_metrics {
            self.draw_metrics(renderer);
        }
    }

    fn draw_widget_bounds(&self, widget: &dyn Widget, renderer: &mut UIRenderer) {
        let rect = widget.rect();
        let color = match widget.widget_type() {
            WidgetType::Container => Vec4::new(0.0, 1.0, 0.0, 0.5),
            WidgetType::Button => Vec4::new(1.0, 0.0, 0.0, 0.5),
            WidgetType::TextField => Vec4::new(0.0, 0.0, 1.0, 0.5),
            _ => Vec4::new(1.0, 1.0, 0.0, 0.5),
        };

        renderer.draw_rect_outline(rect, color, 1.0);

        // Draw padding and margin guides
        if let Some(layout) = widget.layout() {
            let padding = layout.padding;
            let margin = layout.margin;

            renderer.draw_rect_outline(
                Rect::new(rect.position + margin, rect.size - margin * 2.0),
                Vec4::new(0.0, 1.0, 1.0, 0.3),
                1.0,
            );

            renderer.draw_rect_outline(
                Rect::new(
                    rect.position + margin + padding,
                    rect.size - (margin + padding) * 2.0,
                ),
                Vec4::new(1.0, 0.0, 1.0, 0.3),
                1.0,
            );
        }

        // Recursively draw bounds for children
        for child in widget.children() {
            self.draw_widget_bounds(&*child.read(), renderer);
        }
    }

    fn draw_layout_guides(&self, widget: &dyn Widget, renderer: &mut UIRenderer) {
        let rect = widget.rect();

        if let Some(layout) = widget.layout() {
            match layout.layout_type {
                LayoutType::Horizontal => {
                    // Draw horizontal guides
                    let mut x = rect.position.x;
                    for child in widget.children() {
                        renderer.draw_line(
                            Vec2::new(x, rect.position.y),
                            Vec2::new(x, rect.position.y + rect.size.y),
                            Vec4::new(1.0, 0.5, 0.0, 0.5),
                        );
                        x += child.read().rect().size.x + layout.spacing.x;
                    }
                }
                LayoutType::Vertical => {
                    // Draw vertical guides
                    let mut y = rect.position.y;
                    for child in widget.children() {
                        renderer.draw_line(
                            Vec2::new(rect.position.x, y),
                            Vec2::new(rect.position.x + rect.size.x, y),
                            Vec4::new(1.0, 0.5, 0.0, 0.5),
                        );
                        y += child.read().rect().size.y + layout.spacing.y;
                    }
                }
                _ => {}
            }
        }

        // Recursively draw guides for children
        for child in widget.children() {
            self.draw_layout_guides(&*child.read(), renderer);
        }
    }

    fn draw_widget_hierarchy(&self, widget: &dyn Widget, renderer: &mut UIRenderer) {
        let mut text = String::new();
        self.build_hierarchy_text(widget, 0, &mut text);

        renderer.draw_text(
            &text,
            Vec2::new(10.0, 10.0),
            12.0,
            Vec4::new(1.0, 1.0, 1.0, 0.8),
        );
    }

    fn build_hierarchy_text(&self, widget: &dyn Widget, depth: usize, output: &mut String) {
        writeln!(
            output,
            "{:indent$}{:?} #{} ({:?})",
            "",
            widget.widget_type(),
            widget.id(),
            widget.rect(),
            indent = depth * 2
        )
        .unwrap();

        for child in widget.children() {
            self.build_hierarchy_text(&*child.read(), depth + 1, output);
        }
    }

    fn draw_metrics(&self, renderer: &mut UIRenderer) {
        let mut text = String::new();
        writeln!(text, "UI Metrics:").unwrap();
        writeln!(text, "Widgets: {}", self.metrics.widget_count).unwrap();
        writeln!(text, "Draw calls: {}", self.metrics.draw_calls).unwrap();
        writeln!(text, "Vertices: {}", self.metrics.vertices).unwrap();
        writeln!(text, "Update: {:.2}ms", self.metrics.update_time * 1000.0).unwrap();
        writeln!(text, "Render: {:.2}ms", self.metrics.render_time * 1000.0).unwrap();

        renderer.draw_text(
            &text,
            Vec2::new(10.0, renderer.screen_size().y - 100.0),
            12.0,
            Vec4::new(1.0, 1.0, 0.0, 0.8),
        );
    }

    pub fn track_draw_call(&mut self) {
        self.metrics.draw_calls += 1;
    }

    pub fn track_vertices(&mut self, count: usize) {
        self.metrics.vertices += count;
    }

    pub fn update_widget_count(&mut self, root: &dyn Widget) {
        self.metrics.widget_count = self.count_widgets(root);
    }

    fn count_widgets(&self, widget: &dyn Widget) -> usize {
        let mut count = 1;
        for child in widget.children() {
            count += self.count_widgets(&*child.read());
        }
        count
    }
}
