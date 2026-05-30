use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::modules::state_machine::events::Reminder;

pub struct Scheduler {
    reminders: HashMap<String, Reminder>,
    enabled: bool,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            reminders: HashMap::new(),
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn add_reminder(&mut self, reminder: Reminder) {
        self.reminders.insert(reminder.id.clone(), reminder);
    }

    pub fn remove_reminder(&mut self, id: &str) {
        self.reminders.remove(id);
    }

    pub fn get_due_reminders(&self, _now: DateTime<Utc>) -> Vec<&Reminder> {
        // TODO: Implement time-based reminder checking
        Vec::new()
    }
}