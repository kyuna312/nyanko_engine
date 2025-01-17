use glutin::window::Window;
use std::sync::Arc;

pub struct PlatformWindow {
    window: Arc<Window>,
}

impl PlatformWindow {
    pub fn new(window: Window) -> Self {
        Self {
            window: Arc::new(window),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            window: Arc::clone(&self.window),
        }
    }
}
