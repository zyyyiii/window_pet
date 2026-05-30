use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetAttributes {
    pub hunger: f64,      // 0-100, 0 = full, 100 = starving
    pub mood: f64,        // 0-100, 0 = sad, 100 = happy
    pub energy: f64,      // 0-100, 0 = exhausted, 100 = energetic
    pub cleanliness: f64, // 0-100, 0 = dirty, 100 = clean
    pub health: f64,      // 0-100, 0 = sick, 100 = healthy
}

impl PetAttributes {
    pub fn new() -> Self {
        Self {
            hunger: 0.0,
            mood: 100.0,
            energy: 100.0,
            cleanliness: 100.0,
            health: 100.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Attributes decrease over time
        let decay_rate = 0.1 * delta_time;
        
        self.hunger = (self.hunger + decay_rate).min(100.0);
        self.mood = (self.mood - decay_rate * 0.5).max(0.0);
        self.energy = (self.energy - decay_rate * 0.3).max(0.0);
        self.cleanliness = (self.cleanliness - decay_rate * 0.2).max(0.0);
        
        // Health decreases if other attributes are low
        if self.hunger > 80.0 || self.mood < 20.0 || self.energy < 20.0 {
            self.health = (self.health - decay_rate * 0.4).max(0.0);
        } else {
            self.health = (self.health + decay_rate * 0.1).min(100.0);
        }
    }

    pub fn feed(&mut self) {
        self.hunger = (self.hunger - 30.0).max(0.0);
        self.mood = (self.mood + 10.0).min(100.0);
        self.health = (self.health + 5.0).min(100.0);
    }

    pub fn play(&mut self) {
        self.mood = (self.mood + 20.0).min(100.0);
        self.energy = (self.energy - 10.0).max(0.0);
        self.hunger = (self.hunger + 5.0).min(100.0);
    }

    pub fn rest(&mut self) {
        self.energy = (self.energy + 30.0).min(100.0);
        self.mood = (self.mood + 5.0).min(100.0);
    }

    pub fn clean(&mut self) {
        self.cleanliness = 100.0;
        self.mood = (self.mood + 5.0).min(100.0);
    }

    pub fn is_hungry(&self) -> bool {
        self.hunger > 70.0
    }

    pub fn is_tired(&self) -> bool {
        self.energy < 30.0
    }

    pub fn is_sad(&self) -> bool {
        self.mood < 30.0
    }

    pub fn is_sick(&self) -> bool {
        self.health < 30.0
    }
}

impl Default for PetAttributes {
    fn default() -> Self {
        Self::new()
    }
}