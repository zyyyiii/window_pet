use crate::modules::state_machine::events::SystemInfo;

pub struct AnalysisEngine;

impl AnalysisEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, info: &SystemInfo) -> SystemStatus {
        if info.cpu_usage > 80.0 {
            SystemStatus::HighLoad
        } else if info.memory_usage > 80.0 {
            SystemStatus::HighMemory
        } else if info.network_activity {
            SystemStatus::Active
        } else {
            SystemStatus::Normal
        }
    }
}

pub enum SystemStatus {
    Normal,
    HighLoad,
    HighMemory,
    Active,
}