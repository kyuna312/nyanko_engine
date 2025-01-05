use glfw::{Action, Key, WindowEvent};
use glfw::Context;

/// # Window
///
/// An abstraction layer for creating a glfw window.
///
/// ## Example
/// ```
/// let mut window = Window::new(1280, 720, "Window Title");
/// window.init_gl();
///
/// while !window.should_close() {
///     window.update;
/// }
/// ```
pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Create new window with settings
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, String> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(
            width,
            height,
            title,
            glfw::WindowMode::Windowed
        ).expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);

        Ok(Window {
            glfw,
            window_handle: window,
            events,
        })
    }

    /// Load gl functions.
    pub fn init_gl(&mut self) {
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Get the current window size
    pub fn get_size(&self) -> (u32, u32) {
        let (width, height) = self.window_handle.get_size();
        (width as u32, height as u32)
    }

    /// Get the window aspect ratio
    pub fn get_aspect_ratio(&self) -> f32 {
        self.window_handle.get_size().0 as f32 / self.window_handle.get_size().1 as f32
    }

    /// Poll events and swap buffers.
    pub fn update(&mut self) {
        self.window_handle.swap_buffers();
        self.glfw.poll_events();
        
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                },
                _ => {},
            }
        }
    }

    /// Get a reference to the underlying GLFW window
    pub fn get_window_handle(&self) -> &glfw::Window {
        &self.window_handle
    }
}

// Implement Drop to ensure proper cleanup
impl Drop for Window {
    fn drop(&mut self) {
        // GLFW cleanup happens automatically when the Window is dropped
    }
}