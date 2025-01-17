use glutin::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    ContextBuilder, PossiblyCurrent, WindowedContext,
};
use std::path::PathBuf;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

pub struct PlatformWindow {
    context: WindowedContext<PossiblyCurrent>,
    event_loop: EventLoop<()>,
    #[cfg(target_os = "macos")]
    platform_specific: macos::MacOSWindow,
    #[cfg(target_os = "linux")]
    platform_specific: linux::LinuxWindow,
    #[cfg(target_os = "windows")]
    platform_specific: windows::WindowsWindow,
}

impl PlatformWindow {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(width, height));

        let context = unsafe {
            ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        #[cfg(target_os = "macos")]
        let platform_specific = macos::MacOSWindow::new(&context);
        #[cfg(target_os = "linux")]
        let platform_specific = linux::LinuxWindow::new(&context);
        #[cfg(target_os = "windows")]
        let platform_specific = windows::WindowsWindow::new(&context);

        Self {
            context,
            event_loop,
            platform_specific,
        }
    }

    pub fn get_resource_path() -> PathBuf {
        #[cfg(target_os = "macos")]
        return macos::get_resource_path();
        #[cfg(target_os = "linux")]
        return linux::get_resource_path();
        #[cfg(target_os = "windows")]
        return windows::get_resource_path();
    }

    pub fn run<F>(self, mut callback: F)
    where
        F: 'static + FnMut(&Window, &mut ControlFlow),
    {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                glutin::event::Event::WindowEvent { event, .. } => match event {
                    glutin::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                },
                glutin::event::Event::MainEventsCleared => {
                    callback(self.context.window(), control_flow);
                    self.context.swap_buffers().unwrap();
                }
                _ => {}
            }
        });
    }
}
