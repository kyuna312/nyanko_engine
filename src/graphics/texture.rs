use gl::types::*;
use std::path::Path;

pub struct Texture {
    id: GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        
        let img = match image::open(path) {
            Ok(img) => img,
            Err(_) => {
                let file_contents = std::fs::read(path)
                    .map_err(|e| format!("Failed to read file '{}': {}", path.display(), e))?;
                
                image::load_from_memory(&file_contents)
                    .map_err(|e| format!(
                        "Failed to load texture '{}': {}",
                        path.display(), e
                    ))?
            }
        };

        let img_rgba = img.to_rgba8();
        let width = img.width();
        let height = img.height();

        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            
            // Optimized texture parameters for better quality
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_rgba.as_ptr() as *const _,
            );
            
            // Generate mipmaps for better quality at different scales
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(Texture {
            id,
            width,
            height,
        })
    }

    pub fn new_empty(width: u32, height: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            
            // Optimized parameters for render targets
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA16F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                std::ptr::null(),
            );
        }

        Texture {
            id,
            width,
            height,
        }
    }

    #[inline]
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    #[inline]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
} 