use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::event_loop::EventLoop;

pub struct Window {
    pub context: glutin::WindowedContext<glutin::PossiblyCurrent>,
    pub event_loop: EventLoop<()>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let context = ContextBuilder::new()
            .build_windowed(window_builder, &event_loop)
            .expect("Failed to create context");

        let context = unsafe {
            context.make_current()
                .expect("Failed to make context current")
        };

        Self {
            context,
            event_loop,
        }
    }
}
