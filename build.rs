use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::Path;

fn main() {
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Configure the copy options
    let mut options = CopyOptions::new();
    options.overwrite = true;
    
    // Copy assets to the output directory if they exist
    let assets_path = Path::new("assets");
    if assets_path.exists() {
        copy("assets", &out_dir, &options).expect("Failed to copy assets");
    }
    
    // Copy shaders to the output directory if they exist
    let shaders_path = Path::new("shaders");
    if shaders_path.exists() {
        copy("shaders", &out_dir, &options).expect("Failed to copy shaders");
    }
    
    println!("cargo:rerun-if-changed=assets/");
    println!("cargo:rerun-if-changed=shaders/");
} 