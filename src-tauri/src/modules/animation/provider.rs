use super::types::{AnimationUpdate, SpriteSheetConfig};
use crate::modules::pet::entity::Pet;

pub trait AnimationProvider: Send + Sync {
    fn update(&mut self, pet: &Pet, delta_time: f64) -> Option<AnimationUpdate>;
    fn set_animation(&mut self, animation_name: &str);
    fn current_animation(&self) -> &str;
    fn is_finished(&self) -> bool;
    fn reset(&mut self);
    fn provider_type(&self) -> super::types::AnimationProviderType;
}

pub struct AnimationProviderFactory;

impl AnimationProviderFactory {
    pub fn create_sprite_sheet(config: &SpriteSheetConfig) -> Box<dyn AnimationProvider> {
        Box::new(super::sprite::SpriteSheetAnimation::new(config.clone()))
    }

    pub fn create_from_config(
        config: &SpriteSheetConfig,
    ) -> Box<dyn AnimationProvider> {
        match config.provider_type {
            super::types::AnimationProviderType::SpriteSheet => {
                Self::create_sprite_sheet(config)
            }
            super::types::AnimationProviderType::Live2D => {
                // TODO: Implement Live2D provider
                log::warn!("Live2D not implemented, falling back to sprite sheet");
                Self::create_sprite_sheet(config)
            }
            super::types::AnimationProviderType::Spine => {
                // TODO: Implement Spine provider
                log::warn!("Spine not implemented, falling back to sprite sheet");
                Self::create_sprite_sheet(config)
            }
        }
    }
}