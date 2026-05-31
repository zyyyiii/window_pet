use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// 用户活动状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActivityState {
    /// 学习中
    Studying,
    /// 编程中
    Coding,
    /// 娱乐中
    Entertainment,
    /// 空闲
    Idle,
    /// 未知
    Unknown,
}

impl ActivityState {
    pub fn as_str(&self) -> &str {
        match self {
            ActivityState::Studying => "studying",
            ActivityState::Coding => "coding",
            ActivityState::Entertainment => "entertainment",
            ActivityState::Idle => "idle",
            ActivityState::Unknown => "unknown",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ActivityState::Studying => "学习中",
            ActivityState::Coding => "编程中",
            ActivityState::Entertainment => "娱乐中",
            ActivityState::Idle => "空闲",
            ActivityState::Unknown => "未知",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            ActivityState::Studying => "📚",
            ActivityState::Coding => "💻",
            ActivityState::Entertainment => "🎮",
            ActivityState::Idle => "😴",
            ActivityState::Unknown => "❓",
        }
    }
}

impl Default for ActivityState {
    fn default() -> Self {
        ActivityState::Unknown
    }
}

/// 活动快照 - 单次检测的原始数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySnapshot {
    /// 前台窗口标题
    pub window_title: String,
    /// 前台进程名称（不含扩展名，小写）
    pub process_name: String,
    /// 系统空闲时间（秒）
    pub idle_seconds: u64,
    /// 检测时间戳
    pub timestamp: i64,
}

impl Default for ActivitySnapshot {
    fn default() -> Self {
        Self {
            window_title: String::new(),
            process_name: String::new(),
            idle_seconds: 0,
            timestamp: 0,
        }
    }
}

/// 各状态分数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateScores {
    pub studying: f64,
    pub coding: f64,
    pub entertainment: f64,
    pub idle: f64,
}

impl StateScores {
    pub fn new() -> Self {
        Self {
            studying: 0.0,
            coding: 0.0,
            entertainment: 0.0,
            idle: 0.0,
        }
    }

    /// 获取最高分的状态
    pub fn get_highest_state(&self) -> ActivityState {
        let scores = vec![
            (ActivityState::Studying, self.studying),
            (ActivityState::Coding, self.coding),
            (ActivityState::Entertainment, self.entertainment),
            (ActivityState::Idle, self.idle),
        ];

        scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(state, _)| state)
            .unwrap_or(ActivityState::Unknown)
    }

    /// 获取最高分
    pub fn get_highest_score(&self) -> f64 {
        self.studying
            .max(self.coding)
            .max(self.entertainment)
            .max(self.idle)
    }
}

impl Default for StateScores {
    fn default() -> Self {
        Self::new()
    }
}

/// 活动分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAnalysis {
    /// 推断的状态
    pub state: ActivityState,
    /// 各状态分数
    pub scores: StateScores,
    /// 原始快照
    pub snapshot: ActivitySnapshot,
}

impl Default for ActivityAnalysis {
    fn default() -> Self {
        Self {
            state: ActivityState::Unknown,
            scores: StateScores::new(),
            snapshot: ActivitySnapshot::default(),
        }
    }
}

/// 匹配目标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchTarget {
    /// 匹配进程名
    ProcessName,
    /// 匹配窗口标题
    WindowTitle,
}

/// 匹配方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchType {
    /// 精确匹配
    Exact,
    /// 包含匹配
    Contains,
    /// 前缀匹配
    StartsWith,
    /// 后缀匹配
    EndsWith,
}

/// 评分规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringRule {
    /// 规则ID
    pub id: String,
    /// 规则名称
    pub name: String,
    /// 匹配目标
    pub target: MatchTarget,
    /// 匹配方式
    pub match_type: MatchType,
    /// 匹配模式（小写）
    pub pattern: String,
    /// 对各状态的分数贡献
    pub scores: StateScores,
    /// 是否启用
    pub enabled: bool,
}

impl ScoringRule {
    /// 检查是否匹配
    pub fn matches(&self, snapshot: &ActivitySnapshot) -> bool {
        if !self.enabled {
            return false;
        }

        let text = match self.target {
            MatchTarget::ProcessName => &snapshot.process_name,
            MatchTarget::WindowTitle => &snapshot.window_title,
        };

        let text_lower = text.to_lowercase();
        let pattern_lower = self.pattern.to_lowercase();

        match self.match_type {
            MatchType::Exact => text_lower == pattern_lower,
            MatchType::Contains => text_lower.contains(&pattern_lower),
            MatchType::StartsWith => text_lower.starts_with(&pattern_lower),
            MatchType::EndsWith => text_lower.ends_with(&pattern_lower),
        }
    }
}

/// 状态平滑器
#[derive(Debug)]
pub struct StateSmoother {
    /// 历史记录
    history: VecDeque<ActivityState>,
    /// 窗口大小
    window_size: usize,
}

impl StateSmoother {
    pub fn new(window_size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    /// 添加新的状态并返回平滑后的状态
    pub fn smooth(&mut self, new_state: ActivityState) -> ActivityState {
        // Idle 状态立即生效，不参与平滑
        if new_state == ActivityState::Idle {
            self.history.clear();
            return ActivityState::Idle;
        }

        // 添加到历史
        if self.history.len() >= self.window_size {
            self.history.pop_front();
        }
        self.history.push_back(new_state);

        // 统计各状态出现次数
        let mut counts = std::collections::HashMap::new();
        for state in &self.history {
            *counts.entry(state.clone()).or_insert(0) += 1;
        }

        // 返回出现次数最多的状态
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(state, _)| state)
            .unwrap_or(ActivityState::Unknown)
    }

    /// 重置平滑器
    pub fn reset(&mut self) {
        self.history.clear();
    }
}

impl Default for StateSmoother {
    fn default() -> Self {
        Self::new(3)
    }
}
