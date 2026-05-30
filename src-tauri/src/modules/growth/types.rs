use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 成长系统核心数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthSystem {
    /// 当前等级 (1-100)
    pub level: u32,
    /// 当前经验值
    pub experience: u64,
    /// 陪伴天数
    pub companion_days: u32,
    /// 学习积分（预留）
    pub learning_points: u32,
    /// AI 记忆系统（预留）
    pub ai_memory: AiMemory,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后更新时间
    pub last_updated: DateTime<Utc>,
}

/// AI 记忆系统（预留结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMemory {
    /// 记忆列表
    pub memories: Vec<Memory>,
    /// 性格特征 (trait_name -> value 0-1)
    pub personality_traits: HashMap<String, f32>,
    /// 交互历史摘要
    pub interaction_history: Vec<InteractionRecord>,
}

/// 单条记忆
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    /// 记忆 ID
    pub id: String,
    /// 记忆内容
    pub content: String,
    /// 重要程度 (0-1)
    pub importance: f32,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 最后访问时间
    pub last_accessed: DateTime<Utc>,
    /// 访问次数
    pub access_count: u32,
}

/// 交互记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    /// 交互类型
    pub interaction_type: InteractionType,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 获得的经验值
    pub experience_gained: u64,
}

/// 交互类型（用于经验计算）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InteractionType {
    /// 喂食
    Feed,
    /// 玩耍
    Play,
    /// 对话
    Talk,
    /// 触摸
    Touch,
    /// 在线陪伴
    Companion,
    /// 学习任务
    Learning,
}

/// 等级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelInfo {
    /// 当前等级
    pub current_level: u32,
    /// 当前经验值
    pub current_exp: u64,
    /// 升级所需经验值
    pub exp_to_next_level: u64,
    /// 当前等级进度百分比 (0-100)
    pub progress_percent: f32,
}

/// 成长系统快照（用于前端显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrowthSnapshot {
    /// 等级信息
    pub level_info: LevelInfo,
    /// 陪伴天数
    pub companion_days: u32,
    /// 学习积分
    pub learning_points: u32,
    /// 总交互次数
    pub total_interactions: u32,
    /// 记忆数量
    pub memory_count: u32,
}

/// 经验值奖励配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpRewardConfig {
    pub feed: u64,
    pub play: u64,
    pub talk: u64,
    pub touch: u64,
    pub companion_per_minute: u64,
    pub learning: u64,
}

impl Default for ExpRewardConfig {
    fn default() -> Self {
        Self {
            feed: 10,
            play: 15,
            talk: 5,
            touch: 2,
            companion_per_minute: 1,
            learning: 20,
        }
    }
}
