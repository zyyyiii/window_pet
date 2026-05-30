use super::types::*;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

/// 等级计算公式：每级需要 level * 100 经验值
pub fn exp_required_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }
    (level as u64) * 100
}

/// 计算从 1 级到目标等级所需的总经验值
pub fn total_exp_for_level(level: u32) -> u64 {
    if level <= 1 {
        return 0;
    }
    (1..level).map(|l| exp_required_for_level(l + 1)).sum()
}

/// 根据总经验值计算当前等级
pub fn calculate_level(total_exp: u64) -> u32 {
    let mut level = 1u32;
    let mut exp_used = 0u64;
    
    loop {
        let exp_needed = exp_required_for_level(level + 1);
        if exp_used + exp_needed > total_exp {
            break;
        }
        exp_used += exp_needed;
        level += 1;
        
        // 最大等级 100
        if level >= 100 {
            break;
        }
    }
    
    level
}

/// 计算等级详情
pub fn calculate_level_info(total_exp: u64) -> LevelInfo {
    let current_level = calculate_level(total_exp);
    let exp_for_current = total_exp_for_level(current_level);
    let exp_to_next = exp_required_for_level(current_level + 1);
    let exp_in_current_level = total_exp - exp_for_current;
    
    let progress_percent = if exp_to_next > 0 {
        (exp_in_current_level as f32 / exp_to_next as f32) * 100.0
    } else {
        100.0
    };
    
    LevelInfo {
        current_level,
        current_exp: total_exp,
        exp_to_next_level: exp_to_next,
        progress_percent,
    }
}

impl GrowthSystem {
    /// 创建新的成长系统实例
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            level: 1,
            experience: 0,
            companion_days: 0,
            learning_points: 0,
            ai_memory: AiMemory::new(),
            created_at: now,
            last_updated: now,
        }
    }

    /// 添加经验值并更新等级
    pub fn add_experience(&mut self, amount: u64) -> bool {
        self.experience += amount;
        self.last_updated = Utc::now();
        
        let new_level = calculate_level(self.experience);
        let leveled_up = new_level > self.level;
        self.level = new_level;
        
        leveled_up
    }

    /// 添加学习积分
    pub fn add_learning_points(&mut self, amount: u32) {
        self.learning_points += amount;
        self.last_updated = Utc::now();
    }

    /// 更新陪伴天数
    pub fn update_companion_days(&mut self) {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.created_at);
        self.companion_days = duration.num_days().max(0) as u32;
        self.last_updated = now;
    }

    /// 记录交互并返回获得的经验值
    pub fn record_interaction(&mut self, interaction_type: InteractionType, config: &ExpRewardConfig) -> u64 {
        let exp = match interaction_type {
            InteractionType::Feed => config.feed,
            InteractionType::Play => config.play,
            InteractionType::Talk => config.talk,
            InteractionType::Touch => config.touch,
            InteractionType::Companion => config.companion_per_minute,
            InteractionType::Learning => config.learning,
        };
        
        self.add_experience(exp);
        
        self.ai_memory.interaction_history.push(InteractionRecord {
            interaction_type,
            timestamp: Utc::now(),
            experience_gained: exp,
        });
        
        exp
    }

    /// 获取成长快照
    pub fn get_snapshot(&self) -> GrowthSnapshot {
        GrowthSnapshot {
            level_info: calculate_level_info(self.experience),
            companion_days: self.companion_days,
            learning_points: self.learning_points,
            total_interactions: self.ai_memory.interaction_history.len() as u32,
            memory_count: self.ai_memory.memories.len() as u32,
        }
    }
}

impl Default for GrowthSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl AiMemory {
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
            personality_traits: HashMap::new(),
            interaction_history: Vec::new(),
        }
    }

    /// 添加记忆
    pub fn add_memory(&mut self, content: &str, importance: f32) -> String {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        
        self.memories.push(Memory {
            id: id.clone(),
            content: content.to_string(),
            importance,
            created_at: now,
            last_accessed: now,
            access_count: 0,
        });
        
        // 保持记忆数量在合理范围（最多 100 条）
        if self.memories.len() > 100 {
            // 移除最不重要的记忆
            self.memories.sort_by(|a, b| {
                a.importance.partial_cmp(&b.importance).unwrap_or(std::cmp::Ordering::Equal)
            });
            self.memories.remove(0);
        }
        
        id
    }

    /// 获取记忆并更新访问信息
    pub fn access_memory(&mut self, id: &str) -> Option<&Memory> {
        if let Some(memory) = self.memories.iter_mut().find(|m| m.id == id) {
            memory.last_accessed = Utc::now();
            memory.access_count += 1;
            Some(memory)
        } else {
            None
        }
    }

    /// 设置性格特征
    pub fn set_personality_trait(&mut self, trait_name: &str, value: f32) {
        self.personality_traits.insert(trait_name.to_string(), value.clamp(0.0, 1.0));
    }
}

impl Default for AiMemory {
    fn default() -> Self {
        Self::new()
    }
}
