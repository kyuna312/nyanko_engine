use std::any::{Any, TypeId};
use std::collections::HashMap;

pub type EntityId = u64;

pub trait Component: Any + 'static {}

pub struct Entity {
    id: EntityId,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn get_component<T: Component>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|c| c.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|c| c.downcast_mut::<T>())
    }
} 