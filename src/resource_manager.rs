use crate::graphics::{ShaderProgram, Texture};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

lazy_static! {
    static ref RESOURCE_MANAGER: Mutex<ResourceManager> = Mutex::new(ResourceManager::new());
}

pub struct ResourceManager {
    shaders: HashMap<String, Rc<ShaderProgram>>,
    textures: HashMap<String, Rc<Texture>>,
}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager {
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn get_shader(name: &str) -> Option<Rc<ShaderProgram>> {
        let manager = RESOURCE_MANAGER.lock().unwrap();
        manager.shaders.get(name).map(Rc::clone)
    }

    pub fn get_texture(name: &str) -> Option<Rc<Texture>> {
        let manager = RESOURCE_MANAGER.lock().unwrap();
        manager.textures.get(name).map(Rc::clone)
    }
}
