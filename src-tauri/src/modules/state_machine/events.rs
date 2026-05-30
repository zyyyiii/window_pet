use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PetEvent {
    TimeTick,
    UserInteract(UserAction),
    SystemEvent(SystemInfo),
    ReminderEvent(Reminder),
    AiDecision(AiAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserAction {
    Feed,
    Play,
    Pet,
    Talk,
    Command(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_activity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: String,
    pub title: String,
    pub message: String,
    pub reminder_type: ReminderType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReminderType {
    Study,
    Break,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiAction {
    Wander,
    React(String),
    Suggest(String),
}