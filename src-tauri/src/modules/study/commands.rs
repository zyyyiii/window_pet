use std::sync::Mutex;
use tauri::State;

use super::manager::StudyManager;
use super::types::{StudyMode, StudyReminderConfig, StudySnapshot};

/// 学习状态（Tauri 管理状态）
pub struct StudyState {
    pub manager: Mutex<StudyManager>,
}

impl StudyState {
    pub fn new() -> Self {
        Self {
            manager: Mutex::new(StudyManager::new()),
        }
    }
}

impl Default for StudyState {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取学习模式快照
#[tauri::command]
pub async fn get_study_snapshot(state: State<'_, StudyState>) -> Result<StudySnapshot, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_snapshot())
}

/// 设置学习模式
#[tauri::command]
pub async fn set_study_mode(
    state: State<'_, StudyState>,
    mode: String,
) -> Result<StudySnapshot, String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    let study_mode = StudyMode::from_str(&mode);
    Ok(manager.set_mode(study_mode))
}

/// 检查是否需要显示提醒
#[tauri::command]
pub async fn check_study_reminder(
    state: State<'_, StudyState>,
) -> Result<Option<String>, String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.check_reminder().map(|dt| format!("{:?}", dt)))
}

/// 获取随机学习对话
#[tauri::command]
pub async fn get_study_dialogue(
    state: State<'_, StudyState>,
) -> Result<Option<String>, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_random_dialogue())
}

/// 获取学习时长（秒）
#[tauri::command]
pub async fn get_study_duration(state: State<'_, StudyState>) -> Result<u64, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_study_duration())
}

/// 获取休息时长（秒）
#[tauri::command]
pub async fn get_break_duration(state: State<'_, StudyState>) -> Result<u64, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_break_duration())
}

/// 更新学习计时器
#[tauri::command]
pub async fn update_study_timer(state: State<'_, StudyState>) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.update();
    Ok(())
}

/// 设置提醒配置
#[tauri::command]
pub async fn set_study_reminder_config(
    state: State<'_, StudyState>,
    study_reminder_interval: u64,
    break_reminder_interval: u64,
    word_test_interval: u64,
    enable_word_test: bool,
) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    let config = StudyReminderConfig {
        study_reminder_interval,
        break_reminder_interval,
        word_test_interval,
        enable_word_test,
    };
    manager.set_reminder_config(config);
    Ok(())
}

/// 获取提醒配置
#[tauri::command]
pub async fn get_study_reminder_config(
    state: State<'_, StudyState>,
) -> Result<StudyReminderConfig, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    Ok(manager.get_reminder_config().clone())
}

/// 添加自定义学习提醒
#[tauri::command]
pub async fn add_study_reminder_message(
    state: State<'_, StudyState>,
    message: String,
) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.add_study_reminder(message);
    Ok(())
}

/// 添加自定义休息提醒
#[tauri::command]
pub async fn add_break_reminder_message(
    state: State<'_, StudyState>,
    message: String,
) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.add_break_reminder(message);
    Ok(())
}

/// 添加自定义鼓励消息
#[tauri::command]
pub async fn add_encouragement_message(
    state: State<'_, StudyState>,
    message: String,
) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.add_encouragement(message);
    Ok(())
}
