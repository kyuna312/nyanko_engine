use crate::platform::PlatformWindow;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct Engine {
    config: EngineConfig,
    window: Option<PlatformWindow>,
    renderer: Arc<RwLock<Renderer>>,
    physics: Arc<RwLock<PhysicsWorld>>,
    audio: Arc<RwLock<AudioSystem>>,
    input: Arc<RwLock<InputManager>>,
    resources: Arc<RwLock<ResourceManager>>,
    scene: Arc<RwLock<SceneManager>>,
    events: Arc<RwLock<EventSystem>>,
    debug: Arc<RwLock<DebugSystem>>,
    time: Arc<RwLock<Time>>,
    running: bool,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self {
        let window = PlatformWindow::new(&config.title, config.window_width, config.window_height);

        Self {
            renderer: Arc::new(RwLock::new(Renderer::new(&window))),
            physics: Arc::new(RwLock::new(PhysicsWorld::new(config.physics_timestep))),
            audio: Arc::new(RwLock::new(AudioSystem::new(config.audio_channels))),
            input: Arc::new(RwLock::new(InputManager::new())),
            resources: Arc::new(RwLock::new(ResourceManager::new())),
            scene: Arc::new(RwLock::new(SceneManager::new())),
            events: Arc::new(RwLock::new(EventSystem::new())),
            debug: Arc::new(RwLock::new(DebugSystem::new())),
            time: Arc::new(RwLock::new(Time::new())),
            window: Some(window),
            config,
            running: true,
        }
    }

    pub fn run<F>(&mut self, mut update: F)
    where
        F: FnMut(&mut Engine) -> bool,
    {
        while self.running {
            self.time.write().update();
            self.input.write().update();
            self.physics.write().update(self.time.read().delta_time());

            if !update(self) {
                self.running = false;
                break;
            }

            self.scene.write().update(self.time.read().delta_time());
            self.audio.write().update();

            self.renderer.write().begin_frame();
            self.scene.read().render(&mut self.renderer.write());
            self.renderer.write().end_frame();

            self.debug.write().update();
        }
    }

    // Getters for engine systems
    pub fn renderer(&self) -> Arc<RwLock<Renderer>> {
        self.renderer.clone()
    }

    pub fn physics(&self) -> Arc<RwLock<PhysicsWorld>> {
        self.physics.clone()
    }

    // ... other getters ...

    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        // Cleanup resources in reverse order
        if let Some(window) = self.window.take() {
            drop(window);
        }
    }
}
