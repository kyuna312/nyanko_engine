use fs_extra::dir::{copy, CopyOptions};
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().parent().unwrap();

    let mut options = CopyOptions::new();
    options.overwrite = true;

    // Copy assets to target directory
    copy(
        "assets",
        target_dir,
        &options
    ).expect("Failed to copy assets");
} 