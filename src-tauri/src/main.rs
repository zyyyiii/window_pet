// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod models;
mod services;
mod event_bus;
mod utils;

use modules::growth::commands::GrowthState;
use modules::study::commands::StudyState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // 初始化成长系统状态
            let growth_state = GrowthState::new()
                .map_err(|e| format!("初始化成长系统失败: {}", e))?;
            app.manage(growth_state);

            // 初始化学习系统状态
            let study_state = StudyState::new();
            app.manage(study_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::pet_commands::feed_pet,
            commands::pet_commands::play_with_pet,
            commands::pet_commands::pet_status,
            commands::pet_commands::get_pet_status,
            commands::system_commands::get_system_info,
            commands::animation_commands::set_animation,
            commands::animation_commands::get_available_animations,
            commands::interaction_commands::handle_interaction,
            commands::interaction_commands::get_touch_response,
            commands::interaction_commands::get_random_bubble,
            commands::interaction_commands::get_dialogue_node,
            commands::interaction_commands::hide_bubble,
            commands::mood_commands::get_mood_snapshot,
            commands::mood_commands::set_mood_value,
            commands::mood_commands::apply_mood_interaction_boost,
            commands::mood_commands::get_mood_animation_hint,
            commands::mood_commands::get_mood_emoji,
            // 成长系统命令
            modules::growth::commands::get_growth_snapshot,
            modules::growth::commands::record_growth_interaction,
            modules::growth::commands::add_learning_points,
            modules::growth::commands::add_growth_memory,
            modules::growth::commands::get_level_info,
            modules::growth::commands::save_growth_data,
            // 学习系统命令
            modules::study::commands::get_study_snapshot,
            modules::study::commands::set_study_mode,
            modules::study::commands::check_study_reminder,
            modules::study::commands::get_study_dialogue,
            modules::study::commands::get_study_duration,
            modules::study::commands::get_break_duration,
            modules::study::commands::update_study_timer,
            modules::study::commands::set_study_reminder_config,
            modules::study::commands::get_study_reminder_config,
            modules::study::commands::add_study_reminder_message,
            modules::study::commands::add_break_reminder_message,
            modules::study::commands::add_encouragement_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}