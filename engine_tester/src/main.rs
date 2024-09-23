use nyanko_engine::graphics::window::Window;
use gl::types::*;

fn main() {
  let mut window = Window::new(800, 600, "Nyanko Engine");
  
  window.init_gl();

    while !window.should_close() {
        unsafe { 
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
  
}