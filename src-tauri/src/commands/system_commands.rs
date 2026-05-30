use tauri::command;
use crate::modules::state_machine::events::SystemInfo;

#[command]
pub fn get_system_info() -> Result<SystemInfo, String> {
    // TODO: Implement actual system monitoring
    Ok(SystemInfo {
        cpu_usage: 0.0,
        memory_usage: 0.0,
        network_activity: false,
    })
}