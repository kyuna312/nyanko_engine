use std::path::Path;
use tobj;
use super::model::{Model, Material};
use super::texture::Texture;

pub fn load_model(path: &str) -> Result<Model, String> {
    let obj_path = Path::new(path);
    
    if !obj_path.exists() {
        return Err(format!("Model file not found: {}", path));
    }

    let (models, _materials) = tobj::load_obj(
        obj_path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    ).map_err(|e| format!("Failed to load OBJ file: {}", e))?;

    if models.is_empty() {
        return Err("No models found in the OBJ file".to_string());
    }

    // Combine all meshes into one set of vertices and indices
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index_offset = 0;

    for model in models {
        let mesh = &model.mesh;

        // For each vertex
        for i in 0..mesh.positions.len() / 3 {
            // Position
            vertices.push(mesh.positions[i * 3]);
            vertices.push(mesh.positions[i * 3 + 1]);
            vertices.push(mesh.positions[i * 3 + 2]);

            // Normal (if available)
            if !mesh.normals.is_empty() {
                vertices.push(mesh.normals[i * 3]);
                vertices.push(mesh.normals[i * 3 + 1]);
                vertices.push(mesh.normals[i * 3 + 2]);
            } else {
                vertices.push(0.0);
                vertices.push(1.0);
                vertices.push(0.0);
            }

            // Texture coordinates (if available)
            if !mesh.texcoords.is_empty() {
                vertices.push(mesh.texcoords[i * 2]);
                vertices.push(mesh.texcoords[i * 2 + 1]);
            } else {
                vertices.push(0.0);
                vertices.push(0.0);
            }
        }

        // Add indices
        for index in &mesh.indices {
            indices.push(*index as u32 + index_offset);
        }

        index_offset = vertices.len() as u32 / 8; // 8 = 3 pos + 3 normal + 2 tex
    }

    // Create default material
    let material = Material {
        diffuse_texture: None,
        specular_texture: None,
        shininess: 32.0,
    };

    Ok(Model::new(vertices, indices, material))
} 