use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AnimationProviderType {
    SpriteSheet,
    Live2D,
    Spine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub version: String,
    pub default_animation: String,
    pub sprite_sheets: HashMap<String, SpriteSheetConfig>,
    pub state_mapping: HashMap<String, String>,
    pub transitions: HashMap<String, TransitionConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteSheetConfig {
    #[serde(rename = "type")]
    pub provider_type: AnimationProviderType,
    pub image: String,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_count: u32,
    pub frame_rate: f64,
    pub loop_animation: bool,
    pub columns: u32,
    #[serde(default)]
    pub offset_x: u32,
    #[serde(default)]
    pub offset_y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionConfig {
    pub from: String,
    pub to: String,
    pub duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationState {
    pub current_animation: String,
    pub frame_index: u32,
    pub elapsed_time: f64,
    pub is_playing: bool,
    pub is_transitioning: bool,
    pub transition_progress: f64,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: "idle".to_string(),
            frame_index: 0,
            elapsed_time: 0.0,
            is_playing: true,
            is_transitioning: false,
            transition_progress: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationUpdate {
    pub animation: String,
    pub frame_index: u32,
    pub source_x: u32,
    pub source_y: u32,
    pub source_width: u32,
    pub source_height: u32,
    pub image_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationTransition {
    pub from_animation: String,
    pub to_animation: String,
    pub progress: f64,
    pub from_frame: AnimationUpdate,
    pub to_frame: AnimationUpdate,
}