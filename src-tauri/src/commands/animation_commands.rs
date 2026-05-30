use tauri::command;

#[command]
pub fn set_animation(animation_name: String) -> Result<(), String> {
    log::info!("Setting animation: {}", animation_name);
    Ok(())
}

#[command]
pub fn get_available_animations() -> Result<Vec<String>, String> {
    Ok(vec![
        "idle".to_string(),
        "walk".to_string(),
        "sleep".to_string(),
        "look".to_string(),
        "happy".to_string(),
    ])
}