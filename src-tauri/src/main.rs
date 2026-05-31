// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod models;
mod services;
mod event_bus;
mod utils;

use modules::activity::commands::ActivityState;
use modules::behavior::commands::BehaviorState;
use modules::growth::commands::GrowthState;
use modules::study::commands::StudyState;
use tauri::Manager;
use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let growth_state = GrowthState::new()
                .map_err(|e| format!("failed to initialize growth state: {e}"))?;
            app.manage(growth_state);

            let study_state = StudyState::new();
            app.manage(study_state);

            let activity_state = ActivityState::new();
            app.manage(activity_state);

            let behavior_state = BehaviorState::new();
            app.manage(behavior_state);

            let window = match app.get_webview_window("main") {
                Some(window) => window,
                None => WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("Desktop Pet")
                    .inner_size(300.0, 400.0)
                    .center()
                    .resizable(true)
                    .decorations(true)
                    .transparent(false)
                    .always_on_top(false)
                    .build()
                    .map_err(|e| format!("failed to create main window: {e}"))?,
            };

            let _ = window.show();
            let _ = window.set_focus();
            let _ = window.center();

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
            modules::growth::commands::get_growth_snapshot,
            modules::growth::commands::record_growth_interaction,
            modules::growth::commands::add_learning_points,
            modules::growth::commands::add_growth_memory,
            modules::growth::commands::get_level_info,
            modules::growth::commands::save_growth_data,
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
            modules::activity::commands::get_activity_analysis,
            modules::activity::commands::get_last_activity,
            modules::activity::commands::set_activity_detection_enabled,
            modules::activity::commands::is_activity_detection_enabled,
            modules::activity::commands::get_activity_rules,
            modules::activity::commands::add_activity_rule,
            modules::activity::commands::remove_activity_rule,
            modules::behavior::commands::generate_behavior_dialogue,
            modules::behavior::commands::generate_behavior_reminder,
            modules::behavior::commands::get_behavior_suggestion,
            modules::behavior::commands::get_behavior_providers,
            modules::behavior::commands::get_active_behavior_provider,
            modules::behavior::commands::switch_behavior_provider,
            modules::behavior::commands::register_ai_provider,
            modules::behavior::commands::get_behavior_config,
            modules::behavior::commands::update_behavior_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
