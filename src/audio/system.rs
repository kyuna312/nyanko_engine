use super::*;
use crate::ecs::Entity;
use std::collections::HashMap;

pub struct AudioSystem {
    config: AudioConfig,
    platform_audio: PlatformAudio,
    clips: HashMap<String, Arc<AudioClip>>,
    playing_sources: HashMap<Entity, Arc<RwLock<AudioSource>>>,
    listener: AudioListener,
}

impl AudioSystem {
    pub fn new(config: AudioConfig) -> Self {
        Self {
            platform_audio: PlatformAudio::new(&config),
            config,
            clips: HashMap::new(),
            playing_sources: HashMap::new(),
            listener: AudioListener::default(),
        }
    }

    pub fn load_clip(&mut self, name: &str, data: &[u8], category: &str) -> Result<(), String> {
        let source = self.platform_audio.decode_audio(data)?;
        let clip = AudioClip {
            name: name.to_string(),
            source,
            category: category.to_string(),
        };
        self.clips.insert(name.to_string(), Arc::new(clip));
        Ok(())
    }

    pub fn play(&mut self, name: &str, entity: Entity) -> Option<Arc<RwLock<AudioSource>>> {
        let clip = self.clips.get(name)?;
        let source = Arc::new(RwLock::new(clip.source.clone()));
        self.playing_sources.insert(entity, source.clone());

        self.platform_audio.play_source(&source);
        Some(source)
    }

    pub fn stop(&mut self, entity: Entity) {
        if let Some(source) = self.playing_sources.remove(&entity) {
            self.platform_audio.stop_source(&source);
        }
    }

    pub fn set_listener(&mut self, listener: AudioListener) {
        self.listener = listener;
        self.platform_audio.update_listener(&self.listener);
    }

    pub fn update(&mut self) {
        // Remove finished sources
        self.playing_sources.retain(|_, source| {
            let source = source.read();
            source.position < source.duration() || source.loop_audio
        });

        // Update audio engine
        self.platform_audio.update();
    }

    pub fn set_category_volume(&mut self, category: &str, volume: f32) {
        if let Some(vol) = self.config.category_volumes.get_mut(category) {
            *vol = volume.clamp(0.0, 1.0);
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.config.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_category_volume(&self, category: &str) -> f32 {
        self.config
            .category_volumes
            .get(category)
            .copied()
            .unwrap_or(1.0)
    }

    pub fn get_master_volume(&self) -> f32 {
        self.config.master_volume
    }
}
