mod component;
mod entity;
mod pool;
mod system;

pub use component::Component;
pub use entity::Entity;
pub use pool::ComponentPool;
pub use system::System;

use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub struct World {
    entities: Vec<Entity>,
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    systems: Vec<Box<dyn System>>,
    next_entity_id: u64,
    component_pools: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            systems: Vec::new(),
            next_entity_id: 0,
            component_pools: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        self.entities.push(entity);
        entity
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();

        let components = self
            .components
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<u64, T>::new()))
            .downcast_mut::<HashMap<u64, T>>()
            .unwrap();

        components.insert(entity.id(), component);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|c| c.downcast_ref::<HashMap<u64, T>>())
            .and_then(|components| components.get(&entity.id()))
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|c| c.downcast_mut::<HashMap<u64, T>>())
            .and_then(|components| components.get_mut(&entity.id()))
    }

    pub fn query<T: Component>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<T>();
        if let Some(components) = self
            .components
            .get(&type_id)
            .and_then(|c| c.downcast_ref::<HashMap<u64, T>>())
        {
            components.keys().map(|&id| Entity::new(id)).collect()
        } else {
            Vec::new()
        }
    }

    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }

    pub fn update(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.update(self, delta_time);
        }
    }
}
