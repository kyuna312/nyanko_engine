use nyanko_engine::graphics::{Renderer, RendererConfig};
use nyanko_engine::platform::PlatformWindow;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let platform_window = PlatformWindow::new(window);
    let config = RendererConfig::default();
    let _renderer = Renderer::new(platform_window, config);

    println!("Basic graphics example initialized");
}
