use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RendererConfig {
    pub api: RenderAPI,
    pub vsync: bool,
    pub msaa_samples: u8,
    pub shader_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RenderAPI {
    OpenGL,
    #[cfg(target_os = "windows")]
    DirectX11,
    #[cfg(target_os = "macos")]
    Metal,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Vulkan,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            #[cfg(target_os = "macos")]
            api: RenderAPI::Metal,
            #[cfg(target_os = "windows")]
            api: RenderAPI::DirectX11,
            #[cfg(target_os = "linux")]
            api: RenderAPI::OpenGL,
            vsync: true,
            msaa_samples: 4,
            shader_path: PathBuf::from("shaders"),
        }
    }
}

pub trait PlatformRenderer {
    fn init(&mut self, config: &RendererConfig);
    fn create_window_surface(&mut self) -> Result<(), String>;
    fn cleanup(&mut self);
}
