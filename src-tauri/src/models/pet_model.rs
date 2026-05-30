use serde::{Deserialize, Serialize};
use crate::modules::state_machine::states::PetState;
use crate::modules::pet::attributes::PetAttributes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetModel {
    pub id: String,
    pub name: String,
    pub state: PetState,
    pub attributes: PetAttributes,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl PetModel {
    pub fn new(name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            state: PetState::Idle,
            attributes: PetAttributes::default(),
            created_at: now,
            last_updated: now,
        }
    }

    pub fn update_state(&mut self, state: PetState) {
        self.state = state;
        self.last_updated = chrono::Utc::now();
    }

    pub fn update_attributes(&mut self, attributes: PetAttributes) {
        self.attributes = attributes;
        self.last_updated = chrono::Utc::now();
    }
}