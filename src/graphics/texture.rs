use image::DynamicImage;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_memory(data: &[u8]) -> Self {
        let img = image::load_from_memory(data).expect("Failed to load texture");
        Self::from_image(&img)
    }

    pub fn from_image(img: &DynamicImage) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            let (width, height) = (img.width(), img.height());
            let data = img.as_rgba8().expect("Failed to convert image to RGBA8");

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Self {
            id,
            width: img.width(),
            height: img.height(),
        }
    }
}
