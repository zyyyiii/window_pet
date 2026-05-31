use chrono::{Datelike, Timelike, Utc};
use std::collections::VecDeque;
use std::sync::Mutex;

use super::types::{
    ActivityContext, BehaviorContext, DialogueMessage, MoodContext, PetContext, StudyContext,
    TimeContext,
};

/// 上下文收集器
///
/// 从各系统收集状态，聚合为统一的 BehaviorContext
pub struct ContextCollector {
    /// 对话历史
    dialogue_history: Mutex<VecDeque<DialogueMessage>>,
    /// 最大历史条数
    max_history_size: usize,
}

impl ContextCollector {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            dialogue_history: Mutex::new(VecDeque::with_capacity(max_history_size)),
            max_history_size,
        }
    }

    /// 收集当前上下文
    ///
    /// 从各系统收集状态，聚合为统一的 BehaviorContext
    pub fn collect(
        &self,
        pet_name: &str,
        pet_state: &str,
        pet_hunger: f64,
        pet_mood_value: f64,
        pet_energy: f64,
        pet_cleanliness: f64,
        pet_health: f64,
        activity_state: &str,
        window_title: &str,
        process_name: &str,
        idle_seconds: u64,
        study_mode: &str,
        study_duration_secs: u64,
        break_duration_secs: u64,
        session_count: u32,
        total_study_time: u64,
        mood_state: &str,
        mood_value: f64,
    ) -> BehaviorContext {
        BehaviorContext {
            pet: PetContext {
                name: pet_name.to_string(),
                state: pet_state.to_string(),
                hunger: pet_hunger,
                mood_value: pet_mood_value,
                energy: pet_energy,
                cleanliness: pet_cleanliness,
                health: pet_health,
            },
            activity: ActivityContext {
                current_state: activity_state.to_string(),
                window_title: window_title.to_string(),
                process_name: process_name.to_string(),
                idle_seconds,
            },
            study: StudyContext {
                mode: study_mode.to_string(),
                study_duration_secs,
                break_duration_secs,
                session_count,
                total_study_time,
            },
            mood: MoodContext {
                state: mood_state.to_string(),
                value: mood_value,
            },
            dialogue_history: self.get_recent_history(10),
            time: Self::collect_time_context(),
        }
    }

    /// 记录对话消息
    pub fn record_message(&self, message: DialogueMessage) {
        if let Ok(mut history) = self.dialogue_history.lock() {
            history.push_back(message);
            while history.len() > self.max_history_size {
                history.pop_front();
            }
        }
    }

    /// 获取最近的对话历史
    pub fn get_recent_history(&self, count: usize) -> Vec<DialogueMessage> {
        if let Ok(history) = self.dialogue_history.lock() {
            history
                .iter()
                .rev()
                .take(count)
                .cloned()
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 收集时间上下文
    fn collect_time_context() -> TimeContext {
        let now = Utc::now();
        let hour = now.hour();
        let weekday = now.weekday().num_days_from_sunday();

        TimeContext {
            timestamp: now.timestamp(),
            hour,
            weekday,
            is_workday: weekday >= 1 && weekday <= 5,
        }
    }

    /// 清空对话历史
    pub fn clear_history(&self) {
        if let Ok(mut history) = self.dialogue_history.lock() {
            history.clear();
        }
    }
}

impl Default for ContextCollector {
    fn default() -> Self {
        Self::new(50)
    }
}
