use std::sync::{Mutex, OnceLock};
use crate::modules::pet::{PetManager, PetStatus};
use crate::modules::state_machine::StateMachine;
use crate::modules::state_machine::states::PetState;
use crate::modules::state_machine::events::PetEvent;

static PET_SERVICE: OnceLock<Mutex<PetService>> = OnceLock::new();

pub struct PetService {
    pet_manager: PetManager,
    state_machine: StateMachine,
}

impl PetService {
    pub fn new() -> Self {
        Self {
            pet_manager: PetManager::new(),
            state_machine: StateMachine::new(PetState::Idle),
        }
    }

    pub fn instance() -> std::sync::MutexGuard<'static, PetService> {
        PET_SERVICE.get_or_init(|| Mutex::new(PetService::new())).lock().unwrap()
    }

    pub fn feed(&mut self) {
        self.pet_manager.feed();
        self.state_machine.handle_event(PetEvent::UserInteract(
            crate::modules::state_machine::events::UserAction::Feed,
        ));
        self.pet_manager.set_state(self.state_machine.current_state().clone());
    }

    pub fn play(&mut self) {
        self.pet_manager.play();
        self.state_machine.handle_event(PetEvent::UserInteract(
            crate::modules::state_machine::events::UserAction::Play,
        ));
        self.pet_manager.set_state(self.state_machine.current_state().clone());
    }

    pub fn update(&mut self, delta_time: f64) {
        self.pet_manager.update_attributes(delta_time);
        self.state_machine.handle_event(PetEvent::TimeTick);
        self.pet_manager.set_state(self.state_machine.current_state().clone());
    }

    pub fn get_status(&self) -> PetStatus {
        self.pet_manager.get_status()
    }

    pub fn handle_event(&mut self, event: PetEvent) {
        self.state_machine.handle_event(event);
        self.pet_manager.set_state(self.state_machine.current_state().clone());
    }
}