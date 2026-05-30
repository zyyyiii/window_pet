use crate::modules::state_machine::events::Reminder;

pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Self
    }

    pub fn notify(&self, reminder: &Reminder) {
        // TODO: Implement system notification
        log::info!("Reminder: {} - {}", reminder.title, reminder.message);
    }
}