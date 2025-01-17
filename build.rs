use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=shaders/");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Set up shader directories
    let shader_dir = PathBuf::from("shaders");
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Compile shaders if directory exists
    if shader_dir.exists() {
        compile_shaders(&shader_dir, &out_dir).unwrap();
    }

    // Platform-specific configurations
    match target_os.as_str() {
        "windows" => println!("cargo:rustc-link-lib=dylib=opengl32"),
        "linux" => println!("cargo:rustc-link-lib=dylib=GL"),
        "macos" => println!("cargo:rustc-link-lib=framework=OpenGL"),
        _ => (),
    }
}

fn compile_shaders(
    shader_dir: &PathBuf,
    out_dir: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let compiler = shaderc::Compiler::new().ok_or("Failed to create shader compiler")?;
    let options = shaderc::CompileOptions::new().ok_or("Failed to create compiler options")?;

    for entry in std::fs::read_dir(shader_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path
            .extension()
            .map_or(false, |ext| ext == "vert" || ext == "frag")
        {
            let source = std::fs::read_to_string(&path)?;
            let kind = match path.extension().unwrap().to_str().unwrap() {
                "vert" => shaderc::ShaderKind::Vertex,
                "frag" => shaderc::ShaderKind::Fragment,
                _ => continue,
            };

            let compiled = compiler.compile_into_spirv(
                &source,
                kind,
                path.to_str().unwrap(),
                "main",
                Some(&options),
            )?;

            let out_path = out_dir.join(path.file_name().unwrap());
            std::fs::write(out_path, compiled.as_binary_u8())?;
        }
    }

    Ok(())
}
