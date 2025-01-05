use nyanko_engine::graphics::window::Window;

fn main() {
    // Create window
    let mut window = Window::new(800, 600, "Minimal Test")
        .expect("Failed to create window");
    
    // Initialize OpenGL
    window.init_gl();

    // Main loop
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}

// # Run the minimal test
// cargo run --example minimal 