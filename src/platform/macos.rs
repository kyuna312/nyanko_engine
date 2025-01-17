use glutin::WindowedContext;
use std::path::PathBuf;

pub struct MacOSWindow {
    // macOS specific fields
}

impl MacOSWindow {
    pub fn new(_context: &WindowedContext<glutin::PossiblyCurrent>) -> Self {
        Self {}
    }
}

pub fn get_resource_path() -> PathBuf {
    if let Some(resource_path) = std::env::current_exe().ok() {
        if let Some(path) = resource_path.parent() {
            return path.join("Resources");
        }
    }
    PathBuf::from("Resources")
}
