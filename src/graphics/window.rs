use glfw::{Action, Context, Key, WindowEvent};
use glfw::GlfwReceiver;

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
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    width: u32,
    height: u32,
}

impl Window {
    /// Create new window with settings
    pub fn new(width: u32, height: u32, title: &str) -> Result<Window, String> {
        let mut glfw = glfw::init(glfw::fail_on_errors)
            .map_err(|e| format!("Failed to initialize GLFW: {}", e))?;
        
        // Configure GLFW
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or("Failed to create GLFW window")?;

        // Set up window properties
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);

        Ok(Window {
            glfw,
            window_handle: window,
            events,
            width,
            height,
        })
    }

    /// Load gl functions.
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    /// Get the current window size
    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Get the window aspect ratio
    pub fn get_aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    /// Poll events and swap buffers.
    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }

    /// Get a reference to the underlying GLFW window
    pub fn get_window_handle(&self) -> &glfw::PWindow {
        &self.window_handle
    }
}

// Implement Drop to ensure proper cleanup
impl Drop for Window {
    fn drop(&mut self) {
        // GLFW cleanup happens automatically when the Window is dropped
    }
}