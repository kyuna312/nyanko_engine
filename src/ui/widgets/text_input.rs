use super::*;

pub struct TextInput {
    base: BaseWidget,
    text: String,
    placeholder: String,
    cursor_position: usize,
    selection_start: Option<usize>,
    password: bool,
    on_change: Option<Box<dyn Fn(&str) + Send + Sync>>,
}

impl TextInput {
    pub fn new() -> Self {
        let mut base = BaseWidget::new(WidgetType::TextField);

        // Set default text input style
        base.style = Style::new()
            .with_background_color(Vec4::new(0.1, 0.1, 0.1, 1.0))
            .with_border_color(Vec4::new(0.3, 0.3, 0.3, 1.0))
            .with_border_width(1.0)
            .with_corner_radius(4.0)
            .with_padding(Vec2::new(8.0, 6.0));

        Self {
            base,
            text: String::new(),
            placeholder: String::new(),
            cursor_position: 0,
            selection_start: None,
            password: false,
            on_change: None,
        }
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn with_password(mut self, password: bool) -> Self {
        self.password = password;
        self
    }

    pub fn with_on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.cursor_position = self.text.len();
        if let Some(ref on_change) = self.on_change {
            on_change(&self.text);
        }
    }

    fn handle_input(&mut self, input: &str) {
        // Handle text input
        if !input.is_empty() {
            if let Some(selection_start) = self.selection_start {
                let start = selection_start.min(self.cursor_position);
                let end = selection_start.max(self.cursor_position);
                self.text.replace_range(start..end, input);
                self.cursor_position = start + input.len();
                self.selection_start = None;
            } else {
                self.text.insert_str(self.cursor_position, input);
                self.cursor_position += input.len();
            }

            if let Some(ref on_change) = self.on_change {
                on_change(&self.text);
            }
        }
    }
}

impl Widget for TextInput {
    // Implement Widget trait methods similar to Button and Panel
    // ... (basic Widget trait implementations)

    fn update(&mut self, ctx: &mut UIContext) {
        let state = self.base.state();

        // Handle focus and text input
        if state.focused {
            self.handle_input(ctx.text_input());
        }

        // Update base widget
        self.base.update(ctx);
    }
}
