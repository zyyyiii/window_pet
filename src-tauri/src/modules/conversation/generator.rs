use super::parser::ParsedInput;
use crate::modules::pet::attributes::PetAttributes;

pub struct ResponseGenerator;

impl ResponseGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, input: &ParsedInput, attributes: &PetAttributes) -> String {
        match input {
            ParsedInput::Feed => {
                if attributes.is_hungry() {
                    "Yes please! I'm starving!".to_string()
                } else {
                    "I'm not hungry right now, but thanks!".to_string()
                }
            }
            ParsedInput::Play => {
                if attributes.is_tired() {
                    "I'm too tired to play right now.".to_string()
                } else {
                    "Let's play! What should we do?".to_string()
                }
            }
            ParsedInput::Greeting => {
                "Hello! How are you today?".to_string()
            }
            ParsedInput::StatusQuery => {
                format!(
                    "I'm feeling {}. Hunger: {}, Mood: {}, Energy: {}",
                    if attributes.is_sad() { "sad" } else { "okay" },
                    attributes.hunger as u32,
                    attributes.mood as u32,
                    attributes.energy as u32
                )
            }
            ParsedInput::Unknown(text) => {
                format!("I don't understand '{}'. Can you rephrase?", text)
            }
        }
    }
}