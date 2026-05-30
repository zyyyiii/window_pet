use super::provider::AnimationProvider;
use super::types::{AnimationUpdate, AnimationProviderType, SpriteSheetConfig};
use crate::modules::pet::entity::Pet;

pub struct SpriteSheetAnimation {
    config: SpriteSheetConfig,
    current_animation: String,
    frame_index: u32,
    elapsed_time: f64,
    is_playing: bool,
}

impl SpriteSheetAnimation {
    pub fn new(config: SpriteSheetConfig) -> Self {
        Self {
            config,
            current_animation: String::new(),
            frame_index: 0,
            elapsed_time: 0.0,
            is_playing: true,
        }
    }

    fn get_frame_source(&self, frame_index: u32) -> (u32, u32, u32, u32) {
        let col = frame_index % self.config.columns;
        let row = frame_index / self.config.columns;
        
        let x = self.config.offset_x + col * self.config.frame_width;
        let y = self.config.offset_y + row * self.config.frame_height;
        
        (x, y, self.config.frame_width, self.config.frame_height)
    }
}

impl AnimationProvider for SpriteSheetAnimation {
    fn update(&mut self, _pet: &Pet, delta_time: f64) -> Option<AnimationUpdate> {
        if !self.is_playing {
            return None;
        }

        self.elapsed_time += delta_time;

        let frame_duration = 1000.0 / self.config.frame_rate;
        
        if self.elapsed_time >= frame_duration {
            self.elapsed_time -= frame_duration;
            
            if self.frame_index + 1 >= self.config.frame_count {
                if self.config.loop_animation {
                    self.frame_index = 0;
                } else {
                    self.is_playing = false;
                    return None;
                }
            } else {
                self.frame_index += 1;
            }
        }

        let (source_x, source_y, width, height) = self.get_frame_source(self.frame_index);

        Some(AnimationUpdate {
            animation: self.current_animation.clone(),
            frame_index: self.frame_index,
            source_x,
            source_y,
            source_width: width,
            source_height: height,
            image_path: format!("animations/{}", self.config.image),
        })
    }

    fn set_animation(&mut self, animation_name: &str) {
        if self.current_animation != animation_name {
            self.current_animation = animation_name.to_string();
            self.frame_index = 0;
            self.elapsed_time = 0.0;
            self.is_playing = true;
        }
    }

    fn current_animation(&self) -> &str {
        &self.current_animation
    }

    fn is_finished(&self) -> bool {
        !self.is_playing && !self.config.loop_animation
    }

    fn reset(&mut self) {
        self.frame_index = 0;
        self.elapsed_time = 0.0;
        self.is_playing = true;
    }

    fn provider_type(&self) -> AnimationProviderType {
        AnimationProviderType::SpriteSheet
    }
}