use crate::modules::state_machine::events::SystemInfo;

pub struct SystemMonitor {
    enabled: bool,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self { enabled: false }
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

    pub fn get_system_info(&self) -> SystemInfo {
        // TODO: Implement actual system monitoring
        SystemInfo {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_activity: false,
        }
    }
}