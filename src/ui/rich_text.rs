use super::*;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct RichText {
    text: String,
    spans: Vec<TextSpan>,
    default_style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct TextSpan {
    range: Range<usize>,
    style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font_size: Option<f32>,
    pub color: Option<Vec4>,
    pub font_weight: Option<FontWeight>,
    pub italic: bool,
    pub underline: bool,
    pub strike_through: bool,
    pub background_color: Option<Vec4>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FontWeight {
    Light,
    Regular,
    Medium,
    Bold,
    Black,
}

impl RichText {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            spans: Vec::new(),
            default_style: TextStyle::default(),
        }
    }

    pub fn with_default_style(mut self, style: TextStyle) -> Self {
        self.default_style = style;
        self
    }

    pub fn add_span(&mut self, range: Range<usize>, style: TextStyle) {
        self.spans.push(TextSpan { range, style });
    }

    pub fn style_range(&mut self, range: Range<usize>) -> StyleBuilder {
        StyleBuilder::new(self, range)
    }

    pub fn draw(&self, position: Vec2, font_atlas: &FontAtlas, renderer: &mut UIRenderer) {
        let mut current_pos = position;
        let mut current_style = self.default_style.clone();
        let mut span_iter = self.spans.iter().peekable();

        for (idx, ch) in self.text.char_indices() {
            // Update current style if we've entered a new span
            while let Some(span) = span_iter.peek() {
                if idx >= span.range.start && idx < span.range.end {
                    current_style.merge(&span.style);
                    break;
                } else if idx >= span.range.end {
                    span_iter.next();
                    current_style = self.default_style.clone();
                } else {
                    break;
                }
            }

            // Draw the character with current style
            if let Some(glyph_info) = font_atlas.get_glyph(ch) {
                let font_size = current_style.font_size.unwrap_or(16.0);
                let scale = font_size / font_atlas.base_size();

                // Draw background if specified
                if let Some(bg_color) = current_style.background_color {
                    renderer.draw_rect(
                        Rect::new(
                            current_pos,
                            Vec2::new(glyph_info.advance * scale, font_size),
                        ),
                        bg_color,
                    );
                }

                // Draw the glyph
                let color = current_style.color.unwrap_or(Vec4::ONE);
                font_atlas.draw_glyph(ch, current_pos, scale, color, renderer);

                // Draw underline
                if current_style.underline {
                    let underline_pos = current_pos + Vec2::new(0.0, font_size * 1.1);
                    renderer.draw_line(
                        underline_pos,
                        underline_pos + Vec2::new(glyph_info.advance * scale, 0.0),
                        color,
                        1.0,
                    );
                }

                // Draw strike-through
                if current_style.strike_through {
                    let strike_pos = current_pos + Vec2::new(0.0, font_size * 0.6);
                    renderer.draw_line(
                        strike_pos,
                        strike_pos + Vec2::new(glyph_info.advance * scale, 0.0),
                        color,
                        1.0,
                    );
                }

                current_pos.x += glyph_info.advance * scale;
            }
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: None,
            color: None,
            font_weight: None,
            italic: false,
            underline: false,
            strike_through: false,
            background_color: None,
        }
    }
}

impl TextStyle {
    pub fn merge(&mut self, other: &TextStyle) {
        if let Some(size) = other.font_size {
            self.font_size = Some(size);
        }
        if let Some(color) = other.color {
            self.color = Some(color);
        }
        if let Some(weight) = other.font_weight {
            self.font_weight = Some(weight);
        }
        self.italic |= other.italic;
        self.underline |= other.underline;
        self.strike_through |= other.strike_through;
        if let Some(bg_color) = other.background_color {
            self.background_color = Some(bg_color);
        }
    }
}

pub struct StyleBuilder<'a> {
    text: &'a mut RichText,
    range: Range<usize>,
    style: TextStyle,
}

impl<'a> StyleBuilder<'a> {
    fn new(text: &'a mut RichText, range: Range<usize>) -> Self {
        Self {
            text,
            range,
            style: TextStyle::default(),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.style.font_size = Some(size);
        self
    }

    pub fn color(mut self, color: Vec4) -> Self {
        self.style.color = Some(color);
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.style.font_weight = Some(weight);
        self
    }

    pub fn italic(mut self) -> Self {
        self.style.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.style.underline = true;
        self
    }

    pub fn strike_through(mut self) -> Self {
        self.style.strike_through = true;
        self
    }

    pub fn background(mut self, color: Vec4) -> Self {
        self.style.background_color = Some(color);
        self
    }

    pub fn build(self) {
        self.text.add_span(self.range, self.style);
    }
}
