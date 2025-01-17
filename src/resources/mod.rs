use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Resource {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub loaded: bool,
}

pub struct ResourceManager {
    resources: HashMap<String, Arc<RwLock<Resource>>>,
    base_path: PathBuf,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            base_path: PathBuf::from("assets"),
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(&mut self, path: P) -> Arc<RwLock<Resource>> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        if let Some(resource) = self.resources.get(&path_str) {
            return resource.clone();
        }

        let full_path = self.base_path.join(&path);
        let resource = Arc::new(RwLock::new(Resource {
            path: full_path.clone(),
            data: Vec::new(),
            loaded: false,
        }));

        self.resources.insert(path_str, resource.clone());

        // Load resource asynchronously
        let resource_clone = resource.clone();
        std::thread::spawn(move || {
            if let Ok(data) = std::fs::read(&full_path) {
                let mut resource = resource_clone.write();
                resource.data = data;
                resource.loaded = true;
            }
        });

        resource
    }

    pub fn get<P: AsRef<std::path::Path>>(&self, path: P) -> Option<Arc<RwLock<Resource>>> {
        self.resources
            .get(&path.as_ref().to_string_lossy().to_string())
            .cloned()
    }

    pub fn unload<P: AsRef<std::path::Path>>(&mut self, path: P) {
        self.resources
            .remove(&path.as_ref().to_string_lossy().to_string());
    }
}
