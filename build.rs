use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    // Create asset directories
    let dirs = [
        "assets/shaders",
        "assets/textures",
        "assets/config",
    ];

    for dir in dirs.iter() {
        let path = target_dir.join(dir);
        fs::create_dir_all(&path).expect(&format!("Failed to create directory: {}", dir));
    }

    // Copy shader files
    let shader_files = [
        "assets/shaders/sprite.vert",
        "assets/shaders/sprite.frag",
        "assets/shaders/bloom.vert",
        "assets/shaders/bloom.frag",
    ];

    for shader in shader_files.iter() {
        println!("cargo:rerun-if-changed={}", shader);
        let source = Path::new(shader);
        let target = target_dir.join(shader);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).expect("Failed to create shader directory");
        }

        if source.exists() {
            fs::copy(source, target).expect(&format!("Failed to copy shader: {}", shader));
        }
    }

    // Copy config file
    println!("cargo:rerun-if-changed=assets/config/graphics.toml");
    let config_source = Path::new("assets/config/graphics.toml");
    let config_target = target_dir.join("assets/config/graphics.toml");

    if let Some(parent) = config_target.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }

    if config_source.exists() {
        fs::copy(config_source, config_target).expect("Failed to copy config file");
    }
} 