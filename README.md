# Nyanko Engine

A lightweight 2D game engine built in Rust with OpenGL, featuring physics simulation and sprite rendering.

![Nyanko Engine Demo](assets/textures/character.png)

## Features

- OpenGL-based rendering system
- Basic physics simulation with bouncing mechanics
- Texture loading and sprite rendering
- Smooth scaling and transition effects
- Asset management system

## Prerequisites

- Rust (latest stable version)
- OpenGL 3.3+
- CMake (for building dependencies)

## Dependencies

```toml
[dependencies]
gl = "0.14"
glfw = "0.45"
image = "0.24"
cgmath = "0.18"
log = "0.4"
env_logger = "0.10"
chrono = "0.4"
```


## Project Structure

```
nyanko_engine/
├── assets/
│ ├── textures/
│ │ └── character.png
│ └── shaders/
│ ├── sprite.vert
│ └── sprite.frag
├── src/
│ ├── graphics/
│ │ ├── renderer.rs
│ │ └── window.rs
│ ├── physics/
│ │ └── mod.rs
│ └── lib.rs
├── examples/
│ └── physics_demo.rs
└── Cargo.toml
└── README.md
```


## Quick Start

1. Clone the repository:
bash```
git clone https://github.com/kyuna312/nyanko_engine.git
cd nyanko_engine
```

2. Build the project:
bash```
cargo build
```


3. Run the physics demo:

bash```
cargo run --example physics_demo
```


## Examples

### Physics Demo
The physics demo showcases a bouncing character with smooth transitions and scaling effects:

rust
// Run the physics demo
```
cargo run --example physics_demo
```


## Asset Setup

1. Create required directories:

2. Add your textures to `assets/textures/`
3. Add shader files to `assets/shaders/`

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenGL for graphics rendering
- GLFW for window management
- Image-rs for texture loading
- cgmath for mathematics


Kyuna - [@kyuna312](https://twitter.com/yourusername)
Project Link: [https://github.com/kyuna312/nyanko_engine](https://github.com/kyuna312/nyanko_engine)