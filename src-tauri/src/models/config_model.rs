use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub pet_name: String,
    pub auto_start: bool,
    pub always_on_top: bool,
    pub transparency: f64,
    pub animation_speed: f64,
    pub update_interval: u64,
    pub enable_ai: bool,
    pub enable_monitoring: bool,
    pub enable_reminders: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pet_name: "Pet".to_string(),
            auto_start: true,
            always_on_top: true,
            transparency: 0.9,
            animation_speed: 1.0,
            update_interval: 1000,
            enable_ai: false,
            enable_monitoring: false,
            enable_reminders: false,
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        // TODO: Load from file
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        // TODO: Save to file
        Ok(())
    }
}