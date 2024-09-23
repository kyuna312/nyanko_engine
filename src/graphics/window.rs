use glfw::{Action, Context, Key, WindowEvent};
use std::sync::mpsc::Receiver;

/// # Window
///
/// An abstraction layer for creating a GLFW window.
///
/// ## Example
/// ```
/// let mut window = Window::new(1280, 720, "Window Title");
/// window.init_gl();
///
/// while !window.should_close() {
///     window.update();
/// }
/// ```
pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Create a new window with the given settings.
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect("Failed to initialize GLFW");

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Self {
            glfw,
            window_handle: window,
            events,
        }
    }

    /// Load OpenGL functions.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    /// Check if the window should close.
    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Poll events and swap buffers.
    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
        self.process_events();
    }

    /// Process window events, including resizing and key presses.
    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
