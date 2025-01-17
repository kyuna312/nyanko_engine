use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Event: Any + Send + Sync {}

pub struct EventHandler<T: Event> {
    handlers: Vec<Box<dyn Fn(&T) + Send + Sync>>,
}

impl<T: Event> EventHandler<T> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn subscribe<F>(&mut self, handler: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    pub fn emit(&self, event: &T) {
        for handler in &self.handlers {
            handler(event);
        }
    }
}

pub struct EventSystem {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn subscribe<T: Event + 'static, F>(&mut self, handler: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<T>();
        let handler_entry = self
            .handlers
            .entry(type_id)
            .or_insert_with(|| Box::new(EventHandler::<T>::new()));

        let handler_entry = handler_entry.downcast_mut::<EventHandler<T>>().unwrap();

        handler_entry.subscribe(handler);
    }

    pub fn emit<T: Event + 'static>(&self, event: T) {
        if let Some(handler) = self.handlers.get(&TypeId::of::<T>()) {
            if let Some(handler) = handler.downcast_ref::<EventHandler<T>>() {
                handler.emit(&event);
            }
        }
    }
}
