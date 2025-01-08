use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use crate::graphics::Texture;

#[derive(Clone)]
pub enum Asset {
    Texture(Arc<Texture>),
    // Add more asset types here
}

pub struct AssetManager {
    assets: HashMap<PathBuf, Asset>,
    base_path: PathBuf,
}

impl AssetManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            assets: HashMap::new(),
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Result<Arc<Texture>, String> {
        let full_path = self.base_path.join(path);
        
        if let Some(Asset::Texture(texture)) = self.assets.get(&full_path) {
            return Ok(texture.clone());
        }

        let texture = Arc::new(Texture::from_path(full_path.to_str().unwrap())?);
        self.assets.insert(full_path, Asset::Texture(texture.clone()));
        Ok(texture)
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }
}

pub type SharedAssetManager = Arc<RwLock<AssetManager>>; 