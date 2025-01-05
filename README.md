# Nyanko Engine

A lightweight OpenGL-based graphics engine written in Rust, designed for rendering anime-style graphics with modern effects.

## Features

- Modern OpenGL (4.1+) rendering pipeline
- Efficient texture and shader management
- Resource caching system
- Support for anime-style rendering effects
- Real-time lighting and neon effects
- Performance-optimized rendering
- Easy-to-use API

## Prerequisites

Before building the project, ensure you have the following installed:

- Rust 1.70.0 or later
- OpenGL 4.1 or later
- GLFW 3.3 or later
- GLAD 4.6 or later
- GLM 0.9.9 or later
- Image 0.23.14 or later
- GL 0.15.0 or later
- GL 0.15.0 or later
- GL 0.15.0 or later
- GL 0.15.0 or later

bash
Ubuntu/Debian
sudo apt-get install libgl1-mesa-dev libglfw3-dev
Fedora
sudo dnf install mesa-libGL-devel glfw-devel
macOS
brew install glfw
Windows (using vcpkg)
vcpkg install glfw3:x64-windows

## Quick Start

1. Clone the repository:
```

2. Build the project:
```bash
cargo build --release
```

3. Run the character demo:
```bash
cargo run --example character_demo
```

## Project Structure

```
nyanko_engine/
├── src/
│   ├── graphics/
│   │   ├── gl_wrapper.rs    # OpenGL abstractions
│   │   ├── model.rs         # 3D model handling
│   │   ├── texture.rs       # Texture management
│   │   └── window.rs        # Window management
│   ├── resource_manager.rs   # Asset management
│   └── lib.rs               # Library root
├── examples/
│   └── character_demo.rs     # Demo application
├── assets/
│   ├── shaders/
│   │   ├── character.vert
│   │   └── character.frag
│   └── models/
│       └── character/
│           └── textures/
└── README.md
```

## Usage

Here's a simple example of how to use the engine:

```rust
use nyanko_engine::{
    graphics::{Window, ShaderProgram, Texture},
    resource_manager::ResourceManager,
};

fn main() {
    // Create a window
    let mut window = Window::new(1280, 720, "Nyanko Demo")
        .expect("Failed to create window");
    
    // Load resources
    let shader = ResourceManager::get_shader("character")
        .unwrap_or_else(|| {
            ShaderProgram::new(
                "assets/shaders/character.vert",
                "assets/shaders/character.frag"
            )
        });

    // Main loop
    while !window.should_close() {
        // Your rendering code here
        window.update();
    }
}
```

## Features in Detail

### Graphics Pipeline
- Modern shader-based rendering
- Efficient vertex buffer management
- Texture caching and optimization
- Support for instanced rendering

### Resource Management
- Automatic resource caching
- Efficient memory usage
- Thread-safe resource loading
- Automatic cleanup

### Effects
- Real-time lighting
- Neon glow effects
- Anime-style shading
- Custom shader support

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- GLFW for window management
- OpenGL for graphics rendering
- The Rust community for excellent crates and support

## Future Plans

- [ ] Implement post-processing effects
- [ ] Add physics support
- [ ] Enhance shader system
- [ ] Add animation support
- [ ] Implement particle systems
- [ ] Add GUI system

## Contact

Your Name - [@yourusername](https://twitter.com/yourusername)
Project Link: [https://github.com/yourusername/nyanko_engine](https://github.com/yourusername/nyanko_engine)