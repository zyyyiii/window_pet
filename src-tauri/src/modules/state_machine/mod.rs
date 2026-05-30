pub mod states;
pub mod events;
pub mod transitions;

use states::PetState;
use events::PetEvent;
use transitions::TransitionTable;

pub struct StateMachine {
    current_state: PetState,
    transition_table: TransitionTable,
}

impl StateMachine {
    pub fn new(initial_state: PetState) -> Self {
        Self {
            current_state: initial_state,
            transition_table: TransitionTable::default(),
        }
    }

    pub fn current_state(&self) -> &PetState {
        &self.current_state
    }

    pub fn handle_event(&mut self, event: PetEvent) -> Option<PetState> {
        if let Some(new_state) = self.transition_table.get_next_state(&self.current_state, &event) {
            let old_state = self.current_state.clone();
            self.current_state = new_state.clone();
            log::info!("State transition: {:?} -> {:?} (event: {:?})", old_state, new_state, event);
            Some(new_state)
        } else {
            log::debug!("No transition for state {:?} with event {:?}", self.current_state, event);
            None
        }
    }

    pub fn set_state(&mut self, state: PetState) {
        self.current_state = state;
    }
}