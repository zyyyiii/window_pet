use std::collections::HashMap;
use std::path::PathBuf;
use super::config::AnimationConfigLoader;
use super::provider::{AnimationProvider, AnimationProviderFactory};
use super::types::{AnimationConfig, AnimationState, AnimationUpdate};
use crate::modules::pet::entity::Pet;

pub struct AnimationManager {
    config: AnimationConfig,
    providers: HashMap<String, Box<dyn AnimationProvider>>,
    state: AnimationState,
    state_mapping: HashMap<String, String>,
    current_animation_key: String,
    base_path: PathBuf,
}

impl AnimationManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            config: AnimationConfig {
                version: "1.0".to_string(),
                default_animation: "idle".to_string(),
                sprite_sheets: HashMap::new(),
                state_mapping: HashMap::new(),
                transitions: HashMap::new(),
            },
            providers: HashMap::new(),
            state: AnimationState::default(),
            state_mapping: HashMap::new(),
            current_animation_key: "idle".to_string(),
            base_path,
        }
    }

    pub fn load_config(&mut self, config_path: &PathBuf) -> Result<(), String> {
        let config = AnimationConfigLoader::load(config_path)?;
        
        self.state_mapping = config.state_mapping.clone();
        
        for (name, sprite_config) in &config.sprite_sheets {
            let provider = AnimationProviderFactory::create_from_config(sprite_config);
            self.providers.insert(name.clone(), provider);
        }

        self.config = config;
        
        let default = self.config.default_animation.clone();
        self.set_animation(&default);
        
        log::info!("Animation config loaded with {} animations", self.providers.len());
        Ok(())
    }

    pub fn set_animation(&mut self, animation_name: &str) {
        if let Some(provider) = self.providers.get_mut(animation_name) {
            provider.set_animation(animation_name);
            self.current_animation_key = animation_name.to_string();
            self.state.current_animation = animation_name.to_string();
            self.state.frame_index = 0;
            self.state.elapsed_time = 0.0;
            self.state.is_playing = true;
        }
    }

    pub fn set_state_animation(&mut self, state_name: &str) {
        let animation_name = self.state_mapping.get(state_name).cloned();
        if let Some(animation_name) = animation_name {
            self.set_animation(&animation_name);
        } else {
            log::warn!("No animation mapped for state: {}", state_name);
        }
    }

    pub fn update(&mut self, pet: &Pet, delta_time: f64) -> Option<AnimationUpdate> {
        let current = self.current_animation_key.clone();
        
        if let Some(provider) = self.providers.get_mut(&current) {
            provider.update(pet, delta_time)
        } else {
            None
        }
    }

    pub fn get_state(&self) -> &AnimationState {
        &self.state
    }

    pub fn get_current_animation(&self) -> &str {
        &self.current_animation_key
    }

    pub fn get_available_animations(&self) -> Vec<String> {
        self.providers.keys().cloned().collect()
    }

    pub fn get_animation_config(&self, name: &str) -> Option<&super::types::SpriteSheetConfig> {
        self.config.sprite_sheets.get(name)
    }

    pub fn get_state_mapping(&self) -> &HashMap<String, String> {
        &self.state_mapping
    }

    pub fn get_transitions(&self) -> &HashMap<String, super::types::TransitionConfig> {
        &self.config.transitions
    }

    pub fn is_animation_available(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }

    pub fn pause(&mut self) {
        self.state.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.state.is_playing = true;
    }

    pub fn reset(&mut self) {
        if let Some(provider) = self.providers.get_mut(&self.current_animation_key) {
            provider.reset();
        }
        self.state.frame_index = 0;
        self.state.elapsed_time = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_animation_manager_creation() {
        let manager = AnimationManager::new(PathBuf::from("."));
        assert_eq!(manager.get_current_animation(), "idle");
    }
}