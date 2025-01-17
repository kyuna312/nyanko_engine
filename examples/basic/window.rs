use winit::event_loop::EventLoop;
use nyanko_engine::platform::PlatformWindow;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let _platform_window = PlatformWindow::new(window);

    println!("Basic window example initialized");
}
