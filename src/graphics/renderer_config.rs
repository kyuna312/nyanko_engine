use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RendererConfig {
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub msaa_samples: u32,
    pub shader_path: String,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            vsync: true,
            msaa_samples: 4,
            shader_path: "shaders".to_string(),
        }
    }
}

pub trait PlatformRenderer {
    fn init(&mut self, config: &RendererConfig);
    fn create_window_surface(&mut self) -> Result<(), String>;
    fn cleanup(&mut self);
}
