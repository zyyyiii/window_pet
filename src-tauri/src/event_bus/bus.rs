use std::sync::{Arc, Mutex};
use crate::modules::state_machine::events::PetEvent;

pub struct EventBus {
    subscribers: Vec<Arc<Mutex<dyn Fn(&PetEvent) + Send>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn publish(&self, event: PetEvent) {
        for subscriber in &self.subscribers {
            if let Ok(callback) = subscriber.lock() {
                callback(&event);
            }
        }
    }

    pub fn subscribe(&mut self, callback: Box<dyn Fn(&PetEvent) + Send>) {
        self.subscribers.push(Arc::new(Mutex::new(callback)));
    }
}