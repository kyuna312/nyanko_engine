use image::DynamicImage;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub enum AssetType {
    Texture(Arc<RwLock<DynamicImage>>),
    Sound(Arc<Vec<u8>>),
    Data(Arc<Vec<u8>>),
}

pub struct AssetCache {
    assets: HashMap<PathBuf, AssetType>,
    base_path: PathBuf,
}

impl AssetCache {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            assets: HashMap::new(),
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    pub fn load_texture<P: AsRef<Path>>(&mut self, path: P) -> Option<Arc<RwLock<DynamicImage>>> {
        let full_path = self.base_path.join(path);

        if let Some(AssetType::Texture(texture)) = self.assets.get(&full_path) {
            return Some(texture.clone());
        }

        if let Ok(image) = image::open(&full_path) {
            let texture = Arc::new(RwLock::new(image));
            self.assets
                .insert(full_path, AssetType::Texture(texture.clone()));
            Some(texture)
        } else {
            None
        }
    }

    pub fn load_sound<P: AsRef<Path>>(&mut self, path: P) -> Option<Arc<Vec<u8>>> {
        let full_path = self.base_path.join(path);

        if let Some(AssetType::Sound(sound)) = self.assets.get(&full_path) {
            return Some(sound.clone());
        }

        if let Ok(data) = std::fs::read(&full_path) {
            let sound = Arc::new(data);
            self.assets
                .insert(full_path, AssetType::Sound(sound.clone()));
            Some(sound)
        } else {
            None
        }
    }

    pub fn load_data<P: AsRef<Path>>(&mut self, path: P) -> Option<Arc<Vec<u8>>> {
        let full_path = self.base_path.join(path);

        if let Some(AssetType::Data(data)) = self.assets.get(&full_path) {
            return Some(data.clone());
        }

        if let Ok(data) = std::fs::read(&full_path) {
            let data = Arc::new(data);
            self.assets.insert(full_path, AssetType::Data(data.clone()));
            Some(data)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.assets.clear();
    }

    pub fn remove<P: AsRef<Path>>(&mut self, path: P) {
        self.assets.remove(&self.base_path.join(path));
    }
}
pub type SharedAssetManager = Arc<RwLock<AssetManager>>;
