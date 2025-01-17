use lru::LruCache;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct AssetManager {
    textures: Arc<RwLock<LruCache<String, Texture>>>,
    sounds: Arc<RwLock<LruCache<String, Sound>>>,
    max_cache_size: usize,
}

impl AssetManager {
    pub fn new(cache_size: usize) -> Self {
        Self {
            textures: Arc::new(RwLock::new(LruCache::new(cache_size))),
            sounds: Arc::new(RwLock::new(LruCache::new(cache_size))),
            max_cache_size: cache_size,
        }
    }

    pub fn get_texture(&self, path: &str) -> Option<Arc<Texture>> {
        if let Some(texture) = self.textures.read().get(path) {
            return Some(Arc::new(texture.clone()));
        }

        // Load texture if not in cache
        if let Ok(texture) = Texture::load(path) {
            self.textures.write().put(path.to_string(), texture.clone());
            return Some(Arc::new(texture));
        }
        None
    }
}
