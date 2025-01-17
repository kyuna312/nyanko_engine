use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=shaders/");

    let shader_dir = PathBuf::from("shaders");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    if shader_dir.exists() {
        compile_shaders(&shader_dir, &out_dir).unwrap();
    }
}

fn compile_shaders(shader_dir: &PathBuf, out_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let compiler = shaderc::Compiler::new().ok_or("Failed to create shader compiler")?;
    let options = shaderc::CompileOptions::new().ok_or("Failed to create compiler options")?;

    for entry in std::fs::read_dir(shader_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "vert" || ext == "frag") {
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

    // Platform-specific configurations
    match target_os.as_str() {
        "windows" => setup_windows(),
        "macos" => setup_macos(),
        "linux" => setup_linux(),
        _ => panic!("Unsupported platform"),
    }

    // Shader compilation
    compile_shaders(&out_dir);

    // Resource embedding
    embed_resources(&out_dir);
}

fn setup_windows() {
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=opengl32");

    // Check for DirectX support
    if cfg!(feature = "dx11") {
        println!("cargo:rustc-link-lib=d3d11");
        println!("cargo:rustc-link-lib=dxgi");
    }
}

fn setup_macos() {
    println!("cargo:rustc-link-lib=framework=Metal");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=CoreAudio");
}

fn setup_linux() {
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rustc-link-lib=asound");
}

fn compile_shaders(out_dir: &PathBuf) {
    let shader_dir = PathBuf::from("assets/shaders");

    // Compile GLSL shaders
    if cfg!(feature = "opengl") {
        compile_glsl_shaders(&shader_dir, out_dir);
    }

    // Compile HLSL shaders for DirectX
    if cfg!(all(target_os = "windows", feature = "dx11")) {
        compile_hlsl_shaders(&shader_dir, out_dir);
    }

    // Compile Metal shaders
    if cfg!(all(target_os = "macos", feature = "metal")) {
        compile_metal_shaders(&shader_dir, out_dir);
    }
}

fn embed_resources(out_dir: &PathBuf) {
    let resource_dir = PathBuf::from("assets");

    // Generate resource embedding code
    let mut resources = Vec::new();

    // Walk the resource directory
    for entry in walkdir::WalkDir::new(&resource_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path();
            let relative_path = path.strip_prefix(&resource_dir).unwrap();
            resources.push((relative_path.to_str().unwrap(), path));
        }
    }

    // Generate resource loader code
    let mut loader_code = String::new();
    loader_code.push_str("pub fn load_embedded_resource(path: &str) -> Option<&'static [u8]> {\n");
    loader_code.push_str("    match path {\n");

    for (relative_path, path) in resources {
        loader_code.push_str(&format!(
            "        \"{}\" => Some(include_bytes!(r#\"{}\"#)),\n",
            relative_path,
            path.display()
        ));
    }

    loader_code.push_str("        _ => None,\n");
    loader_code.push_str("    }\n");
    loader_code.push_str("}\n");

    // Write the generated code
    std::fs::write(out_dir.join("resource_loader.rs"), loader_code).unwrap();
}

fn compile_glsl_shaders(shader_dir: &PathBuf, out_dir: &PathBuf) {
    use shaderc::{Compiler, ShaderKind};
    let mut compiler = Compiler::new().unwrap();

    for entry in std::fs::read_dir(shader_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if let Some(ext) = path.extension() {
            let kind = match ext.to_str().unwrap() {
                "vert" => ShaderKind::Vertex,
                "frag" => ShaderKind::Fragment,
                "comp" => ShaderKind::Compute,
                _ => continue,
            };

            let source = std::fs::read_to_string(&path).unwrap();
            let compiled = compiler
                .compile_into_spirv(&source, kind, path.to_str().unwrap(), "main", None)
                .unwrap();

            let out_path = out_dir
                .join(path.file_name().unwrap())
                .with_extension("spv");
            std::fs::write(out_path, compiled.as_binary_u8()).unwrap();
        }
    }
}

// Add HLSL and Metal shader compilation functions similarly
