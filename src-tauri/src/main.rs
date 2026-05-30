// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod modules;
mod models;
mod services;
mod event_bus;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::pet_commands::feed_pet,
            commands::pet_commands::play_with_pet,
            commands::pet_commands::pet_status,
            commands::pet_commands::get_pet_status,
            commands::system_commands::get_system_info,
            commands::animation_commands::set_animation,
            commands::animation_commands::get_available_animations,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}