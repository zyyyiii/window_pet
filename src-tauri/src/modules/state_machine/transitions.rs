use super::states::PetState;
use super::events::{PetEvent, UserAction};

pub struct TransitionTable;

impl TransitionTable {
    pub fn new() -> Self {
        Self
    }

    pub fn get_next_state(&self, current_state: &PetState, event: &PetEvent) -> Option<PetState> {
        match (current_state, event) {
            // Idle state transitions
            (PetState::Idle, PetEvent::TimeTick) => Some(PetState::Hungry),
            (PetState::Idle, PetEvent::UserInteract(UserAction::Feed)) => Some(PetState::Happy),
            (PetState::Idle, PetEvent::UserInteract(UserAction::Play)) => Some(PetState::Playing),
            (PetState::Idle, PetEvent::UserInteract(UserAction::Talk)) => Some(PetState::Talking),
            (PetState::Idle, PetEvent::ReminderEvent(_)) => Some(PetState::Studying),
            (PetState::Idle, PetEvent::SystemEvent(_)) => Some(PetState::Monitoring),
            
            // Happy state transitions
            (PetState::Happy, PetEvent::TimeTick) => Some(PetState::Idle),
            
            // Hungry state transitions
            (PetState::Hungry, PetEvent::UserInteract(UserAction::Feed)) => Some(PetState::Happy),
            
            // Playing state transitions
            (PetState::Playing, PetEvent::TimeTick) => Some(PetState::Idle),
            
            // Talking state transitions
            (PetState::Talking, PetEvent::TimeTick) => Some(PetState::Idle),
            
            // Studying state transitions
            (PetState::Studying, PetEvent::TimeTick) => Some(PetState::Idle),
            
            // Monitoring state transitions
            (PetState::Monitoring, PetEvent::TimeTick) => Some(PetState::Idle),
            
            // No transition
            _ => None,
        }
    }
}

impl Default for TransitionTable {
    fn default() -> Self {
        Self::new()
    }
}