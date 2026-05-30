use tauri::command;
use crate::modules::interaction::types::{InteractionType, InteractionResponse, DialogueNode};

#[command]
pub fn handle_interaction(interaction_type: String, x: f64, y: f64) -> Result<InteractionResponse, String> {
    let interaction = match interaction_type.as_str() {
        "touch" => InteractionType::Touch { x, y },
        "double_touch" => InteractionType::DoubleTouch { x, y },
        "right_click" => InteractionType::RightClick { x, y },
        "bubble" => InteractionType::BubbleTrigger,
        _ => return Err(format!("Unknown interaction type: {}", interaction_type)),
    };

    let mut manager = crate::modules::interaction::InteractionManager::new();
    Ok(manager.handle_interaction(interaction))
}

#[command]
pub fn get_touch_response() -> Result<String, String> {
    let manager = crate::modules::interaction::InteractionManager::new();
    Ok(manager.get_state_response())
}

#[command]
pub fn get_random_bubble() -> Result<Option<crate::modules::interaction::types::BubbleMessage>, String> {
    let manager = crate::modules::interaction::InteractionManager::new();
    Ok(manager.force_bubble())
}

#[command]
pub fn get_dialogue_node(tree_id: String, node_id: String) -> Result<Option<DialogueNode>, String> {
    let manager = crate::modules::interaction::InteractionManager::new();
    Ok(manager.get_dialogue_node(&tree_id, &node_id))
}

#[command]
pub fn hide_bubble() -> Result<(), String> {
    let manager = crate::modules::interaction::InteractionManager::new();
    manager.hide_bubble();
    Ok(())
}