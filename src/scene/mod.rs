use crate::ecs::{Component, Entity, World};
use crate::graphics::Renderer;
use std::any::TypeId;
use std::collections::HashMap;

pub struct Scene {
    world: World,
    entities: HashMap<String, Entity>,
    active: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            entities: HashMap::new(),
            active: true,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.world.create_entity()
    }

    pub fn create_named_entity(&mut self, name: &str) -> Entity {
        let entity = self.world.create_entity();
        self.entities.insert(name.to_string(), entity);
        entity
    }

    pub fn get_entity(&self, name: &str) -> Option<Entity> {
        self.entities.get(name).copied()
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.world.add_component(entity, component);
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.world.get_component(entity)
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        self.world.get_component_mut(entity)
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.active {
            self.world.update(delta_time);
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        if self.active {
            // Render all entities with sprite components
            for entity in self.world.query::<(Transform, SpriteRenderer)>() {
                if let (Some(transform), Some(sprite)) = (
                    self.get_component::<Transform>(entity),
                    self.get_component::<SpriteRenderer>(entity),
                ) {
                    renderer.draw_sprite(transform, sprite);
                }
            }
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn clear(&mut self) {
        self.world = World::new();
        self.entities.clear();
    }
}

// Scene Manager for handling multiple scenes
pub struct SceneManager {
    scenes: HashMap<String, Scene>,
    current_scene: Option<String>,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
            current_scene: None,
        }
    }

    pub fn add_scene(&mut self, name: &str, scene: Scene) {
        self.scenes.insert(name.to_string(), scene);
    }

    pub fn load_scene(&mut self, name: &str) {
        if let Some(current) = self.current_scene.as_ref() {
            if let Some(scene) = self.scenes.get_mut(current) {
                scene.set_active(false);
            }
        }

        if let Some(scene) = self.scenes.get_mut(name) {
            scene.set_active(true);
            self.current_scene = Some(name.to_string());
        }
    }

    pub fn get_current_scene(&self) -> Option<&Scene> {
        self.current_scene
            .as_ref()
            .and_then(|name| self.scenes.get(name))
    }

    pub fn get_current_scene_mut(&mut self) -> Option<&mut Scene> {
        self.current_scene
            .as_ref()
            .and_then(|name| self.scenes.get_mut(name))
    }

    pub fn update(&mut self, delta_time: f32) {
        if let Some(scene) = self.get_current_scene_mut() {
            scene.update(delta_time);
        }
    }

    pub fn render(&self, renderer: &mut Renderer) {
        if let Some(scene) = self.get_current_scene() {
            scene.render(renderer);
        }
    }
}
