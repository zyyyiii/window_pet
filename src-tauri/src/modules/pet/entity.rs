use super::attributes::PetAttributes;
use crate::modules::state_machine::states::PetState;

#[derive(Debug, Clone)]
pub struct Pet {
    pub name: String,
    pub state: PetState,
    pub attributes: PetAttributes,
    pub position: (f64, f64),
    pub animation: String,
}

impl Pet {
    pub fn new() -> Self {
        Self {
            name: "Pet".to_string(),
            state: PetState::Idle,
            attributes: PetAttributes::default(),
            position: (0.0, 0.0),
            animation: "idle".to_string(),
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::new()
        }
    }

    pub fn update_animation(&mut self) {
        self.animation = match self.state {
            PetState::Idle => "idle".to_string(),
            PetState::Happy => "happy".to_string(),
            PetState::Hungry => "hungry".to_string(),
            PetState::Sleepy => "sleepy".to_string(),
            PetState::Playing => "playing".to_string(),
            PetState::Studying => "studying".to_string(),
            PetState::Monitoring => "monitoring".to_string(),
            PetState::Talking => "talking".to_string(),
        };
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    pub fn wander(&mut self) {
        // Random movement logic
        let dx = rand::random::<f64>() * 2.0 - 1.0;
        let dy = rand::random::<f64>() * 2.0 - 1.0;
        self.position.0 += dx;
        self.position.1 += dy;
    }
}