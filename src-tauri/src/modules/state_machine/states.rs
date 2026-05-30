use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetState {
    Idle,
    Happy,
    Hungry,
    Sleepy,
    Playing,
    Studying,
    Monitoring,
    Talking,
}

impl PetState {
    pub fn as_str(&self) -> &'static str {
        match self {
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

impl Default for PetState {
    fn default() -> Self {
        PetState::Idle
    }
}