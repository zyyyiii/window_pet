use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoodState {
    Happy,
    Normal,
    Bored,
    Sleepy,
}

impl MoodState {
    pub fn as_str(&self) -> &'static str {
        match self {
            MoodState::Happy => "happy",
            MoodState::Normal => "normal",
            MoodState::Bored => "bored",
            MoodState::Sleepy => "sleepy",
        }
    }

    pub fn from_range(value: f64) -> Self {
        match value as u32 {
            75..=100 => MoodState::Happy,
            50..=74 => MoodState::Normal,
            25..=49 => MoodState::Bored,
            0..=24 => MoodState::Sleepy,
            _ => MoodState::Normal,
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            MoodState::Happy => "😺",
            MoodState::Normal => "🐱",
            MoodState::Bored => "😿",
            MoodState::Sleepy => "😴",
        }
    }

    pub fn animation_hint(&self) -> &'static str {
        match self {
            MoodState::Happy => "happy",
            MoodState::Normal => "idle",
            MoodState::Bored => "bored",
            MoodState::Sleepy => "sleepy",
        }
    }
}

impl Default for MoodState {
    fn default() -> Self {
        MoodState::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodRange {
    pub min: f64,
    pub max: f64,
}

impl MoodRange {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn default_range() -> Self {
        Self { min: 0.0, max: 100.0 }
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.max(self.min).min(self.max)
    }
}

impl Default for MoodRange {
    fn default() -> Self {
        Self::default_range()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodSnapshot {
    pub value: f64,
    pub state: MoodState,
    pub state_str: String,
    pub emoji: String,
    pub animation_hint: String,
}

impl MoodSnapshot {
    pub fn from_manager(value: f64, state: MoodState) -> Self {
        Self {
            value,
            state: state.clone(),
            state_str: state.as_str().to_string(),
            emoji: state.emoji().to_string(),
            animation_hint: state.animation_hint().to_string(),
        }
    }
}

pub type MoodModifierFn = Box<dyn Fn(f64, f64) -> f64 + Send + Sync>;