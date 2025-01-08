use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::any::{Any, TypeId};

pub struct ResourceManager {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add<T: 'static + Send + Sync>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|r| r.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut T> {
        self.resources
            .get_mut(&TypeId::of::<T>())
            .and_then(|r| r.downcast_mut::<T>())
    }
}

pub type SharedResourceManager = Arc<RwLock<ResourceManager>>; 