use crate::modules::pet::attributes::PetAttributes;
use crate::modules::state_machine::events::AiAction;

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn make_decision(&self, attributes: &PetAttributes) -> Option<AiAction> {
        if attributes.is_hungry() {
            return Some(AiAction::Suggest("I'm hungry, please feed me!".to_string()));
        }

        if attributes.is_tired() {
            return Some(AiAction::Suggest("I'm tired, I need rest.".to_string()));
        }

        if attributes.is_sad() {
            return Some(AiAction::Suggest("I'm sad, please play with me.".to_string()));
        }

        if attributes.is_sick() {
            return Some(AiAction::Suggest("I don't feel well.".to_string()));
        }

        None
    }
}