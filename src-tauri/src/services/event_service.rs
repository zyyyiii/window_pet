use std::sync::{Mutex, OnceLock};
use crate::event_bus::EventBus;
use crate::modules::state_machine::events::PetEvent;

static EVENT_SERVICE: OnceLock<Mutex<EventService>> = OnceLock::new();

pub struct EventService {
    event_bus: EventBus,
}

impl EventService {
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::new(),
        }
    }

    pub fn instance() -> std::sync::MutexGuard<'static, EventService> {
        EVENT_SERVICE.get_or_init(|| Mutex::new(EventService::new())).lock().unwrap()
    }

    pub fn publish(&mut self, event: PetEvent) {
        self.event_bus.publish(event);
    }

    pub fn subscribe(&mut self, callback: Box<dyn Fn(&PetEvent) + Send>) {
        self.event_bus.subscribe(callback);
    }
}