use gl::types::*;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
    format: GLenum,
}

impl Texture {
    pub fn new(width: u32, height: u32, format: GLenum) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Allocate storage
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Self {
            id,
            width,
            height,
            format,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| format!("Failed to load texture: {}", e))?;
        Self::from_dynamic_image(&img)
    }

    pub fn from_memory(data: &[u8]) -> Result<Self, String> {
        let img = image::load_from_memory(data)
            .map_err(|e| format!("Failed to load texture from memory: {}", e))?;
        Self::from_dynamic_image(&img)
    }

    fn from_dynamic_image(img: &DynamicImage) -> Result<Self, String> {
        let format = match img.color() {
            image::ColorType::Rgb8 => gl::RGB,
            image::ColorType::Rgba8 => gl::RGBA,
            _ => return Err("Unsupported image format".to_string()),
        };

        let mut texture = Self::new(img.width(), img.height(), format);
        texture.set_data(img.as_bytes());
        Ok(texture)
    }

    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn set_data(&mut self, data: &[u8]) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.format,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
