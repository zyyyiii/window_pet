use std::path::PathBuf;
use std::fs;
use super::types::AnimationConfig;

pub struct AnimationConfigLoader;

impl AnimationConfigLoader {
    pub fn load(path: &PathBuf) -> Result<AnimationConfig, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read animation config: {}", e))?;
        
        let config: AnimationConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse animation config: {}", e))?;
        
        Self::validate(&config)?;
        
        Ok(config)
    }

    pub fn validate(config: &AnimationConfig) -> Result<(), String> {
        if config.sprite_sheets.is_empty() {
            return Err("Animation config has no sprite sheets defined".to_string());
        }

        for (name, sprite_config) in &config.sprite_sheets {
            if sprite_config.frame_count == 0 {
                return Err(format!("Sprite sheet '{}' has zero frames", name));
            }
            if sprite_config.frame_rate <= 0.0 {
                return Err(format!("Sprite sheet '{}' has invalid frame rate", name));
            }
            if sprite_config.columns == 0 {
                return Err(format!("Sprite sheet '{}' has zero columns", name));
            }
        }

        for (state, animation) in &config.state_mapping {
            if !config.sprite_sheets.contains_key(animation) {
                return Err(format!(
                    "State '{}' maps to unknown animation '{}'",
                    state, animation
                ));
            }
        }

        Ok(())
    }

    pub fn load_from_dir(dir: &PathBuf) -> Result<AnimationConfig, String> {
        let config_path = dir.join("animations.json");
        Self::load(&config_path)
    }
}