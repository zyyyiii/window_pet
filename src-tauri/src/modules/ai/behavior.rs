use crate::modules::pet::entity::Pet;
use crate::modules::state_machine::events::AiAction;

pub struct BehaviorEngine {
    enabled: bool,
}

impl BehaviorEngine {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn update(&self, pet: &mut Pet) -> Option<AiAction> {
        if !self.enabled {
            return None;
        }

        // Random behavior based on pet state
        let random = rand::random::<f64>();
        
        if random < 0.1 {
            pet.wander();
            Some(AiAction::Wander)
        } else if random < 0.2 {
            Some(AiAction::React("I feel bored".to_string()))
        } else {
            None
        }
    }
}