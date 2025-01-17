use glutin::WindowedContext;
use std::path::PathBuf;
use std::ptr;
use x11::xlib;

pub struct LinuxWindow {
    display: *mut xlib::Display,
    window: xlib::Window,
}

impl LinuxWindow {
    pub fn new(context: &WindowedContext<glutin::PossiblyCurrent>) -> Self {
        let (display, window) = unsafe {
            let raw_handle = context.window().raw_window_handle();
            match raw_handle {
                raw_window_handle::RawWindowHandle::Xlib(handle) => {
                    (handle.display as *mut xlib::Display, handle.window)
                }
                _ => (ptr::null_mut(), 0),
            }
        };

        Self { display, window }
    }

    pub fn set_wm_class(&self, class_name: &str) {
        unsafe {
            if !self.display.is_null() {
                let class = xlib::XClassHint {
                    res_name: class_name.as_ptr() as *mut i8,
                    res_class: class_name.as_ptr() as *mut i8,
                };
                xlib::XSetClassHint(self.display, self.window, &class as *const _);
            }
        }
    }
}

pub fn get_resource_path() -> PathBuf {
    // Check XDG data directories
    if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(xdg_data).join("nyanko_engine");
        if path.exists() {
            return path;
        }
    }

    // Fallback to local directory
    PathBuf::from("Resources")
}
