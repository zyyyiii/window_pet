use super::attributes::PetAttributes;
use crate::modules::state_machine::states::PetState;

#[derive(Debug, Clone)]
pub struct Pet {
    pub name: String,
    pub state: PetState,
    pub attributes: PetAttributes,
    pub position: (f64, f64),
}

impl Pet {
    pub fn new() -> Self {
        Self {
            name: "Pet".to_string(),
            state: PetState::Idle,
            attributes: PetAttributes::default(),
            position: (0.0, 0.0),
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::new()
        }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        self.position = (x, y);
    }

    pub fn wander(&mut self) {
        let dx = rand::random::<f64>() * 2.0 - 1.0;
        let dy = rand::random::<f64>() * 2.0 - 1.0;
        self.position.0 += dx;
        self.position.1 += dy;
    }

    pub fn state_name(&self) -> &str {
        match self.state {
            PetState::Idle => "idle",
            PetState::Happy => "happy",
            PetState::Hungry => "hungry",
            PetState::Sleepy => "sleepy",
            PetState::Playing => "playing",
            PetState::Studying => "studying",
            PetState::Monitoring => "monitoring",
            PetState::Talking => "talking",
        }
    }
}