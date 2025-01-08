use gl;
use image::GenericImageView;

#[derive(Clone)]
pub struct Texture {
    pub(crate) id: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Texture {
    pub fn from_path(path: &str) -> Result<Self, String> {
        let img = image::open(path).map_err(|e| e.to_string())?;
        let (width, height) = img.dimensions();
        
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            let img = img.to_rgba8();
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        
        Ok(Self { id, width, height })
    }
} 