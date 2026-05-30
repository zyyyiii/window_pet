use tauri::command;
use crate::modules::mood::types::MoodSnapshot;

#[command]
pub fn get_mood_snapshot() -> Result<MoodSnapshot, String> {
    let manager = crate::modules::mood::MoodManager::with_default();
    Ok(manager.get_snapshot())
}

#[command]
pub fn set_mood_value(value: f64) -> Result<MoodSnapshot, String> {
    let mut manager = crate::modules::mood::MoodManager::with_default();
    manager.set_value(value);
    Ok(manager.get_snapshot())
}

#[command]
pub fn apply_mood_interaction_boost(amount: Option<f64>) -> Result<MoodSnapshot, String> {
    let mut manager = crate::modules::mood::MoodManager::with_default();
    manager.apply_interaction_boost(amount);
    Ok(manager.get_snapshot())
}

#[command]
pub fn get_mood_animation_hint() -> Result<String, String> {
    let manager = crate::modules::mood::MoodManager::with_default();
    Ok(manager.get_animation_hint())
}

#[command]
pub fn get_mood_emoji() -> Result<String, String> {
    let manager = crate::modules::mood::MoodManager::with_default();
    Ok(manager.get_emoji())
}