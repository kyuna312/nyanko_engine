pub mod core;
pub mod resources;
pub mod assets;
pub mod ecs;
pub mod components;
pub mod scene;
pub mod graphics;
pub mod physics;

use core::{Time, Input};
use resources::ResourceManager;
use assets::AssetManager;
use scene::Scene;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct EngineConfig {
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
        }
    }
}

pub struct Engine {
    time: Time,
    input: Input,
    resource_manager: Arc<RwLock<ResourceManager>>,
    asset_manager: Arc<RwLock<AssetManager>>,
    current_scene: Scene,
    running: bool,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        let resource_manager = Arc::new(RwLock::new(ResourceManager::new()));
        let asset_manager = Arc::new(RwLock::new(AssetManager::new("assets")));
        
        Self {
            time: Time::new(1.0 / 60.0),
            input: Input::new(),
            resource_manager,
            asset_manager,
            current_scene: Scene::new(),
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.update();
            while self.time.should_run_fixed_update() {
                self.fixed_timestep_update();
            }
            self.render();
        }
    }

    pub fn update(&mut self) {
        self.time.update();
        self.input.update();
        self.current_scene.update(self.time.delta_time);
    }

    pub fn fixed_timestep_update(&mut self) {
        self.current_scene.update(self.time.fixed_delta_time);
    }

    pub fn render(&mut self) {
        if let Some(renderer) = self.resource_manager.write().unwrap().get_mut::<graphics::Renderer>() {
            renderer.begin_frame();
            self.current_scene.render(renderer);
            renderer.end_frame();
        }
    }

    // Getters
    pub fn time(&self) -> &Time { &self.time }
    pub fn input(&self) -> &Input { &self.input }
    pub fn input_mut(&mut self) -> &mut Input { &mut self.input }
    pub fn resource_manager(&self) -> Arc<RwLock<ResourceManager>> { self.resource_manager.clone() }
    pub fn asset_manager(&self) -> Arc<RwLock<AssetManager>> { self.asset_manager.clone() }
    pub fn scene(&self) -> &Scene { &self.current_scene }
    pub fn scene_mut(&mut self) -> &mut Scene { &mut self.current_scene }
}