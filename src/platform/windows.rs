use glutin::WindowedContext;
use std::path::PathBuf;
use std::ptr;
use winapi::um::winuser;

pub struct WindowsWindow {
    hwnd: *mut winapi::shared::windef::HWND__,
}

impl WindowsWindow {
    pub fn new(context: &WindowedContext<glutin::PossiblyCurrent>) -> Self {
        let hwnd = unsafe {
            let raw_handle = context.window().raw_window_handle();
            match raw_handle {
                raw_window_handle::RawWindowHandle::Win32(handle) => handle.hwnd,
                _ => ptr::null_mut(),
            }
        };

        Self { hwnd }
    }

    pub fn set_dpi_aware(&self) {
        unsafe {
            winuser::SetProcessDPIAware();
        }
    }

    pub fn enable_dark_mode(&self) {
        // Windows 10 dark mode support
        if let Some(hwnd) = unsafe { self.hwnd.as_mut() } {
            unsafe {
                let _ = winuser::AllowDarkModeForWindow(hwnd as _, true);
                winuser::SendMessageW(hwnd as _, winuser::WM_THEMECHANGED, 0, 0);
            }
        }
    }
}

pub fn get_resource_path() -> PathBuf {
    if let Some(exe_path) = std::env::current_exe().ok() {
        if let Some(exe_dir) = exe_path.parent() {
            return exe_dir.join("Resources");
        }
    }
    PathBuf::from("Resources")
}
