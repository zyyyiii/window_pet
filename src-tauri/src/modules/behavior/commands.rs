use std::sync::Mutex;
use tauri::State;

use super::ai_provider::AIProviderConfig;
use super::context::ContextCollector;
use super::manager::BehaviorManager;
use super::types::{
    AIProvider, BehaviorManagerConfig, BehaviorSuggestion,
    DialogueRequest, DialogueResponse, ReminderResponse, ReminderType,
};

/// 行为系统状态（Tauri 管理状态）
pub struct BehaviorState {
    pub manager: Mutex<BehaviorManager>,
    pub context_collector: ContextCollector,
}

impl BehaviorState {
    pub fn new() -> Self {
        Self {
            manager: Mutex::new(BehaviorManager::new()),
            context_collector: ContextCollector::new(50),
        }
    }
}

impl Default for BehaviorState {
    fn default() -> Self {
        Self::new()
    }
}

/// 生成对话
#[tauri::command]
pub async fn generate_behavior_dialogue(
    state: State<'_, BehaviorState>,
    request: DialogueRequest,
    pet_name: String,
    pet_state: String,
    pet_hunger: f64,
    pet_mood_value: f64,
    pet_energy: f64,
    pet_cleanliness: f64,
    pet_health: f64,
    activity_state: String,
    window_title: String,
    process_name: String,
    idle_seconds: u64,
    study_mode: String,
    study_duration_secs: u64,
    break_duration_secs: u64,
    session_count: u32,
    total_study_time: u64,
    mood_state: String,
    mood_value: f64,
) -> Result<DialogueResponse, String> {
    // 收集上下文（不持有锁）
    let context = state.context_collector.collect(
        &pet_name,
        &pet_state,
        pet_hunger,
        pet_mood_value,
        pet_energy,
        pet_cleanliness,
        pet_health,
        &activity_state,
        &window_title,
        &process_name,
        idle_seconds,
        &study_mode,
        study_duration_secs,
        break_duration_secs,
        session_count,
        total_study_time,
        &mood_state,
        mood_value,
    );

    // 获取管理器并生成对话
    let response = {
        let manager = state.manager.lock().map_err(|e| e.to_string())?;
        manager.generate_dialogue(&context, &request)
    };

    response.await.map_err(|e| e.to_string())
}

/// 生成提醒
#[tauri::command]
pub async fn generate_behavior_reminder(
    state: State<'_, BehaviorState>,
    reminder_type: ReminderType,
    pet_name: String,
    pet_state: String,
    pet_hunger: f64,
    pet_mood_value: f64,
    pet_energy: f64,
    pet_cleanliness: f64,
    pet_health: f64,
    activity_state: String,
    window_title: String,
    process_name: String,
    idle_seconds: u64,
    study_mode: String,
    study_duration_secs: u64,
    break_duration_secs: u64,
    session_count: u32,
    total_study_time: u64,
    mood_state: String,
    mood_value: f64,
) -> Result<ReminderResponse, String> {
    // 收集上下文（不持有锁）
    let context = state.context_collector.collect(
        &pet_name,
        &pet_state,
        pet_hunger,
        pet_mood_value,
        pet_energy,
        pet_cleanliness,
        pet_health,
        &activity_state,
        &window_title,
        &process_name,
        idle_seconds,
        &study_mode,
        study_duration_secs,
        break_duration_secs,
        session_count,
        total_study_time,
        &mood_state,
        mood_value,
    );

    // 获取管理器并生成提醒
    let response = {
        let manager = state.manager.lock().map_err(|e| e.to_string())?;
        manager.generate_reminder(&context, reminder_type)
    };

    response.await.map_err(|e| e.to_string())
}

/// 获取行为建议
#[tauri::command]
pub async fn get_behavior_suggestion(
    state: State<'_, BehaviorState>,
    pet_name: String,
    pet_state: String,
    pet_hunger: f64,
    pet_mood_value: f64,
    pet_energy: f64,
    pet_cleanliness: f64,
    pet_health: f64,
    activity_state: String,
    window_title: String,
    process_name: String,
    idle_seconds: u64,
    study_mode: String,
    study_duration_secs: u64,
    break_duration_secs: u64,
    session_count: u32,
    total_study_time: u64,
    mood_state: String,
    mood_value: f64,
) -> Result<Option<BehaviorSuggestion>, String> {
    // 收集上下文（不持有锁）
    let context = state.context_collector.collect(
        &pet_name,
        &pet_state,
        pet_hunger,
        pet_mood_value,
        pet_energy,
        pet_cleanliness,
        pet_health,
        &activity_state,
        &window_title,
        &process_name,
        idle_seconds,
        &study_mode,
        study_duration_secs,
        break_duration_secs,
        session_count,
        total_study_time,
        &mood_state,
        mood_value,
    );

    // 获取管理器并获取建议
    let response = {
        let manager = state.manager.lock().map_err(|e| e.to_string())?;
        manager.get_behavior_suggestion(&context)
    };

    response.await.map_err(|e| e.to_string())
}

/// 获取已注册的提供者列表
#[tauri::command]
pub async fn get_behavior_providers(
    state: State<'_, BehaviorState>,
) -> Result<Vec<String>, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.get_providers().map_err(|e| e.to_string())
}

/// 获取当前活跃提供者名称
#[tauri::command]
pub async fn get_active_behavior_provider(
    state: State<'_, BehaviorState>,
) -> Result<String, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager
        .get_active_provider_name()
        .map_err(|e| e.to_string())
}

/// 切换活跃提供者
#[tauri::command]
pub async fn switch_behavior_provider(
    state: State<'_, BehaviorState>,
    name: String,
) -> Result<(), String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.switch_provider(&name).map_err(|e| e.to_string())
}

/// 注册 AI 提供者
#[tauri::command]
pub async fn register_ai_provider(
    state: State<'_, BehaviorState>,
    provider: AIProvider,
    config: AIProviderConfig,
) -> Result<(), String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager
        .register_ai_provider(provider, config)
        .map_err(|e| e.to_string())
}

/// 获取行为管理器配置
#[tauri::command]
pub async fn get_behavior_config(
    state: State<'_, BehaviorState>,
) -> Result<BehaviorManagerConfig, String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.get_config().map_err(|e| e.to_string())
}

/// 更新行为管理器配置
#[tauri::command]
pub async fn update_behavior_config(
    state: State<'_, BehaviorState>,
    config: BehaviorManagerConfig,
) -> Result<(), String> {
    let manager = state.manager.lock().map_err(|e| e.to_string())?;
    manager.update_config(config).map_err(|e| e.to_string())
}
