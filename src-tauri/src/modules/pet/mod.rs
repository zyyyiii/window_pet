pub mod entity;
pub mod attributes;

use entity::Pet;
use attributes::PetAttributes;
use crate::modules::state_machine::states::PetState;

pub struct PetManager {
    pet: Pet,
}

impl PetManager {
    pub fn new() -> Self {
        Self {
            pet: Pet::new(),
        }
    }

    pub fn pet(&self) -> &Pet {
        &self.pet
    }

    pub fn pet_mut(&mut self) -> &mut Pet {
        &mut self.pet
    }

    pub fn update_attributes(&mut self, delta_time: f64) {
        self.pet.attributes.update(delta_time);
    }

    pub fn feed(&mut self) {
        self.pet.attributes.feed();
        log::info!("Pet fed. Hunger: {}", self.pet.attributes.hunger);
    }

    pub fn play(&mut self) {
        self.pet.attributes.play();
        log::info!("Pet played. Mood: {}", self.pet.attributes.mood);
    }

    pub fn set_state(&mut self, state: PetState) {
        self.pet.state = state;
    }

    pub fn get_status(&self) -> PetStatus {
        PetStatus {
            state: self.pet.state.clone(),
            attributes: self.pet.attributes.clone(),
            name: self.pet.name.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PetStatus {
    pub state: PetState,
    pub attributes: PetAttributes,
    pub name: String,
}