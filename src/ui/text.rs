use super::*;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct TextBlock {
    text: String,
    font_size: f32,
    color: Vec4,
    align: TextAlign,
    vertical_align: VerticalAlign,
    wrap_width: Option<f32>,
    line_height_multiplier: f32,
    ellipsis: bool,
    max_lines: Option<usize>,
    cached_layout: Option<TextLayout>,
}

#[derive(Debug, Clone)]
struct TextLayout {
    lines: Vec<TextLine>,
    size: Vec2,
}

#[derive(Debug, Clone)]
struct TextLine {
    text: String,
    width: f32,
    glyphs: Vec<PositionedGlyph>,
}

impl TextBlock {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font_size: 16.0,
            color: Vec4::new(1.0, 1.0, 1.0, 1.0),
            align: TextAlign::Left,
            vertical_align: VerticalAlign::Top,
            wrap_width: None,
            line_height_multiplier: 1.2,
            ellipsis: false,
            max_lines: None,
            cached_layout: None,
        }
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self.cached_layout = None;
        self
    }

    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self.cached_layout = None;
        self
    }

    pub fn with_vertical_align(mut self, align: VerticalAlign) -> Self {
        self.vertical_align = align;
        self.cached_layout = None;
        self
    }

    pub fn with_wrap_width(mut self, width: Option<f32>) -> Self {
        self.wrap_width = width;
        self.cached_layout = None;
        self
    }

    pub fn with_line_height(mut self, multiplier: f32) -> Self {
        self.line_height_multiplier = multiplier;
        self.cached_layout = None;
        self
    }

    pub fn with_ellipsis(mut self, ellipsis: bool) -> Self {
        self.ellipsis = ellipsis;
        self.cached_layout = None;
        self
    }

    pub fn with_max_lines(mut self, max_lines: Option<usize>) -> Self {
        self.max_lines = max_lines;
        self.cached_layout = None;
        self
    }

    pub fn layout(&mut self, font_atlas: &FontAtlas) -> &TextLayout {
        if self.cached_layout.is_none() {
            self.cached_layout = Some(self.compute_layout(font_atlas));
        }
        self.cached_layout.as_ref().unwrap()
    }

    fn compute_layout(&self, font_atlas: &FontAtlas) -> TextLayout {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;
        let mut max_width = 0.0;
        let mut total_height = 0.0;

        let words = self.text.unicode_words().collect::<Vec<_>>();
        let space_width = font_atlas.get_glyph_width(' ');

        for (i, word) in words.iter().enumerate() {
            let word_width = font_atlas.get_text_width(word);

            let fits_on_line = match self.wrap_width {
                Some(max_width) => current_width + word_width <= max_width,
                None => true,
            };

            if !fits_on_line && !current_line.is_empty() {
                // Add current line to lines
                lines.push(TextLine {
                    text: current_line,
                    width: current_width,
                    glyphs: vec![], // Will be computed when drawing
                });
                current_line = String::new();
                current_width = 0.0;
                total_height += self.font_size * self.line_height_multiplier;
            }

            // Add word to current line
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += space_width;
            }
            current_line.push_str(word);
            current_width += word_width;
            max_width = max_width.max(current_width);

            // Check max lines
            if let Some(max_lines) = self.max_lines {
                if lines.len() >= max_lines && i < words.len() - 1 {
                    if self.ellipsis {
                        current_line.push_str("...");
                    }
                    break;
                }
            }
        }

        // Add final line
        if !current_line.is_empty() {
            lines.push(TextLine {
                text: current_line,
                width: current_width,
                glyphs: vec![],
            });
            total_height += self.font_size * self.line_height_multiplier;
        }

        TextLayout {
            lines,
            size: Vec2::new(max_width, total_height),
        }
    }

    pub fn draw(&self, position: Vec2, font_atlas: &FontAtlas, renderer: &mut UIRenderer) {
        let layout = self.layout(font_atlas);
        let mut y = position.y;

        // Adjust vertical position based on alignment
        match self.vertical_align {
            VerticalAlign::Top => {}
            VerticalAlign::Middle => {
                y += (self.wrap_width.unwrap_or(0.0) - layout.size.y) * 0.5;
            }
            VerticalAlign::Bottom => {
                y += self.wrap_width.unwrap_or(0.0) - layout.size.y;
            }
        }

        for line in &layout.lines {
            let mut x = position.x;

            // Adjust horizontal position based on alignment
            match self.align {
                TextAlign::Left => {}
                TextAlign::Center => {
                    x += (self.wrap_width.unwrap_or(line.width) - line.width) * 0.5;
                }
                TextAlign::Right => {
                    x += self.wrap_width.unwrap_or(line.width) - line.width;
                }
            }

            font_atlas.draw_text(&line.text, Vec2::new(x, y), self.color, renderer);
            y += self.font_size * self.line_height_multiplier;
        }
    }
}
