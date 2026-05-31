use tauri::State;

use super::detector::SharedActivityDetector;
use super::types::{ActivityAnalysis, ScoringRule};

/// 活动检测状态（Tauri 管理状态）
pub struct ActivityState {
    pub detector: SharedActivityDetector,
}

impl ActivityState {
    pub fn new() -> Self {
        Self {
            detector: SharedActivityDetector::new(),
        }
    }
}

impl Default for ActivityState {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取当前活动分析
#[tauri::command]
pub async fn get_activity_analysis(
    state: State<'_, ActivityState>,
) -> Result<ActivityAnalysis, String> {
    state
        .detector
        .detect()
        .ok_or_else(|| "检测失败".to_string())
}

/// 获取最后一次活动分析（不触发新的检测）
#[tauri::command]
pub async fn get_last_activity(
    state: State<'_, ActivityState>,
) -> Result<Option<ActivityAnalysis>, String> {
    Ok(state.detector.get_last_analysis())
}

/// 启用/禁用活动检测
#[tauri::command]
pub async fn set_activity_detection_enabled(
    state: State<'_, ActivityState>,
    enabled: bool,
) -> Result<(), String> {
    state.detector.set_enabled(enabled);
    Ok(())
}

/// 获取活动检测是否启用
#[tauri::command]
pub async fn is_activity_detection_enabled(
    state: State<'_, ActivityState>,
) -> Result<bool, String> {
    Ok(state.detector.is_enabled())
}

/// 获取所有评分规则
#[tauri::command]
pub async fn get_activity_rules(
    state: State<'_, ActivityState>,
) -> Result<Vec<ScoringRule>, String> {
    state
        .detector
        .get_rules()
        .ok_or_else(|| "获取规则失败".to_string())
}

/// 添加自定义评分规则
#[tauri::command]
pub async fn add_activity_rule(
    state: State<'_, ActivityState>,
    rule: ScoringRule,
) -> Result<bool, String> {
    Ok(state.detector.add_rule(rule))
}

/// 删除评分规则
#[tauri::command]
pub async fn remove_activity_rule(
    state: State<'_, ActivityState>,
    rule_id: String,
) -> Result<bool, String> {
    Ok(state.detector.remove_rule(&rule_id))
}
