use super::*;
use rusttype::{point, Font, PositionedGlyph, Scale};
use std::collections::HashMap;

pub struct FontAtlas {
    texture: Texture,
    glyphs: HashMap<char, GlyphInfo>,
    font: Font<'static>,
    size: f32,
    line_height: f32,
}

#[derive(Debug, Clone, Copy)]
struct GlyphInfo {
    uv_rect: Rect,
    size: Vec2,
    offset: Vec2,
    advance: f32,
}

impl FontAtlas {
    pub fn new(font_data: &[u8], size: f32) -> Result<Self, String> {
        let font =
            Font::try_from_bytes(font_data).ok_or_else(|| "Failed to load font".to_string())?;

        let scale = Scale::uniform(size);
        let v_metrics = font.v_metrics(scale);
        let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        // Pre-generate glyphs for common ASCII range
        let mut glyphs = HashMap::new();
        let mut atlas_builder = AtlasBuilder::new(512, 512);

        for c in ' '..='~' {
            let glyph = font.glyph(c).scaled(scale);
            if let Some(glyph) = glyph.positioned(point(0.0, v_metrics.ascent)) {
                if let Some(bbox) = glyph.pixel_bounding_box() {
                    let size = Vec2::new(bbox.width() as f32, bbox.height() as f32);
                    let offset = Vec2::new(bbox.min.x as f32, bbox.min.y as f32);

                    // Add glyph to atlas
                    if let Some(uv_rect) = atlas_builder.add_glyph(&glyph, size) {
                        glyphs.insert(
                            c,
                            GlyphInfo {
                                uv_rect,
                                size,
                                offset,
                                advance: glyph.unpositioned().h_metrics().advance_width,
                            },
                        );
                    }
                }
            }
        }

        // Generate texture from atlas
        let texture = atlas_builder.build()?;

        Ok(Self {
            texture,
            glyphs,
            font,
            size,
            line_height,
        })
    }

    pub fn get_text_size(&self, text: &str) -> Vec2 {
        let mut size = Vec2::ZERO;
        let mut cursor_x = 0.0;
        let mut max_height = 0.0;

        for c in text.chars() {
            if c == '\n' {
                size.y += self.line_height;
                cursor_x = 0.0;
                continue;
            }

            if let Some(info) = self.glyphs.get(&c) {
                cursor_x += info.advance;
                size.x = size.x.max(cursor_x);
                max_height = max_height.max(info.size.y);
            }
        }

        size.y += max_height;
        size
    }

    pub fn draw_text(&self, text: &str, position: Vec2, color: Vec4, renderer: &mut UIRenderer) {
        let mut cursor = position;

        for c in text.chars() {
            if c == '\n' {
                cursor.x = position.x;
                cursor.y += self.line_height;
                continue;
            }

            if let Some(info) = self.glyphs.get(&c) {
                let pos = cursor + info.offset;
                renderer.draw_glyph(pos, info.size, info.uv_rect, color, &self.texture);
                cursor.x += info.advance;
            }
        }
    }
}

struct AtlasBuilder {
    width: u32,
    height: u32,
    data: Vec<u8>,
    next_x: u32,
    next_y: u32,
    row_height: u32,
}

impl AtlasBuilder {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height) as usize],
            next_x: 0,
            next_y: 0,
            row_height: 0,
        }
    }

    fn add_glyph(&mut self, glyph: &PositionedGlyph, size: Vec2) -> Option<Rect> {
        let width = size.x.ceil() as u32;
        let height = size.y.ceil() as u32;

        // Check if we need to move to next row
        if self.next_x + width > self.width {
            self.next_x = 0;
            self.next_y += self.row_height;
            self.row_height = 0;
        }

        // Check if we have enough space
        if self.next_y + height > self.height {
            return None;
        }

        // Draw glyph to atlas
        if let Some(bbox) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as u32 + self.next_x;
                let y = y as u32 + self.next_y;
                let idx = (y * self.width + x) as usize;
                self.data[idx] = (v * 255.0) as u8;
            });
        }

        // Calculate UV coordinates
        let uv_rect = Rect::new(
            Vec2::new(
                self.next_x as f32 / self.width as f32,
                self.next_y as f32 / self.height as f32,
            ),
            Vec2::new(
                width as f32 / self.width as f32,
                height as f32 / self.height as f32,
            ),
        );

        // Update position
        self.next_x += width;
        self.row_height = self.row_height.max(height);

        Some(uv_rect)
    }

    fn build(&self) -> Result<Texture, String> {
        Texture::from_r8(&self.data, self.width, self.height)
    }
}
