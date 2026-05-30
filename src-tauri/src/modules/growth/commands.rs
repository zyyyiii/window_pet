use super::core::calculate_level_info;
use super::storage::GrowthStorage;
use super::types::*;
use std::sync::Mutex;
use tauri::State;

/// 成长系统状态（全局单例）
pub struct GrowthState {
    pub data: Mutex<GrowthSystem>,
    pub storage: GrowthStorage,
    pub config: ExpRewardConfig,
}

impl GrowthState {
    pub fn new() -> Result<Self, String> {
        let storage = GrowthStorage::new()?;
        let data = storage.load()?;
        
        Ok(Self {
            data: Mutex::new(data),
            storage,
            config: ExpRewardConfig::default(),
        })
    }
}

/// 获取成长系统快照
#[tauri::command]
pub fn get_growth_snapshot(state: State<'_, GrowthState>) -> Result<GrowthSnapshot, String> {
    let mut data = state.data.lock().map_err(|e| e.to_string())?;
    data.update_companion_days();
    Ok(data.get_snapshot())
}

/// 记录交互行为
#[tauri::command]
pub fn record_growth_interaction(
    state: State<'_, GrowthState>,
    interaction_type: InteractionType,
) -> Result<u64, String> {
    let mut data = state.data.lock().map_err(|e| e.to_string())?;
    let exp = data.record_interaction(interaction_type, &state.config);
    
    // 自动保存
    state.storage.save(&data)?;
    
    Ok(exp)
}

/// 添加学习积分
#[tauri::command]
pub fn add_learning_points(
    state: State<'_, GrowthState>,
    amount: u32,
) -> Result<u32, String> {
    let mut data = state.data.lock().map_err(|e| e.to_string())?;
    data.add_learning_points(amount);
    
    // 自动保存
    state.storage.save(&data)?;
    
    Ok(data.learning_points)
}

/// 添加记忆
#[tauri::command]
pub fn add_growth_memory(
    state: State<'_, GrowthState>,
    content: String,
    importance: f32,
) -> Result<String, String> {
    let mut data = state.data.lock().map_err(|e| e.to_string())?;
    let id = data.ai_memory.add_memory(&content, importance);
    
    // 自动保存
    state.storage.save(&data)?;
    
    Ok(id)
}

/// 获取等级信息
#[tauri::command]
pub fn get_level_info(state: State<'_, GrowthState>) -> Result<LevelInfo, String> {
    let data = state.data.lock().map_err(|e| e.to_string())?;
    Ok(calculate_level_info(data.experience))
}

/// 手动保存成长数据
#[tauri::command]
pub fn save_growth_data(state: State<'_, GrowthState>) -> Result<(), String> {
    let data = state.data.lock().map_err(|e| e.to_string())?;
    state.storage.save(&data)
}
