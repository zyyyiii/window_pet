use super::rules::RuleManager;
use super::types::{ActivitySnapshot, ActivityState, StateScores};

/// 评分引擎
pub struct ActivityScorer {
    rule_manager: RuleManager,
    /// 最低分数阈值，低于此值返回 Unknown
    min_score_threshold: f64,
    /// 空闲时间阈值（秒），超过此时间直接判定为 Idle
    idle_threshold: u64,
}

impl ActivityScorer {
    pub fn new() -> Self {
        Self {
            rule_manager: RuleManager::new(),
            min_score_threshold: 1.0,
            idle_threshold: 300, // 5分钟
        }
    }

    /// 计算活动分数
    pub fn score(&self, snapshot: &ActivitySnapshot) -> StateScores {
        let mut scores = StateScores::new();

        // 1. 遍历规则表，累加匹配规则的分数
        for rule in self.rule_manager.get_matching_rules(snapshot) {
            scores.studying += rule.scores.studying;
            scores.coding += rule.scores.coding;
            scores.entertainment += rule.scores.entertainment;
            scores.idle += rule.scores.idle;
        }

        // 2. 根据空闲时间计算 Idle 分数
        if snapshot.idle_seconds > self.idle_threshold {
            // 超过5分钟空闲，大幅增加 Idle 分数
            scores.idle += 10.0;
        } else if snapshot.idle_seconds > 60 {
            // 超过1分钟空闲，小幅增加 Idle 分数
            scores.idle += 3.0;
        }

        scores
    }

    /// 推断活动状态
    pub fn infer_state(&self, snapshot: &ActivitySnapshot) -> ActivityState {
        // 如果空闲时间超过阈值，直接返回 Idle
        if snapshot.idle_seconds > self.idle_threshold {
            return ActivityState::Idle;
        }

        let scores = self.score(snapshot);
        let highest_score = scores.get_highest_score();

        // 如果最高分低于阈值，返回 Unknown
        if highest_score < self.min_score_threshold {
            return ActivityState::Unknown;
        }

        scores.get_highest_state()
    }

    /// 获取规则管理器的可变引用
    pub fn get_rule_manager_mut(&mut self) -> &mut RuleManager {
        &mut self.rule_manager
    }

    /// 获取规则管理器的不可变引用
    pub fn get_rule_manager(&self) -> &RuleManager {
        &self.rule_manager
    }

    /// 设置最低分数阈值
    pub fn set_min_score_threshold(&mut self, threshold: f64) {
        self.min_score_threshold = threshold;
    }

    /// 设置空闲时间阈值
    pub fn set_idle_threshold(&mut self, threshold: u64) {
        self.idle_threshold = threshold;
    }
}

impl Default for ActivityScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vscode_detection() {
        let scorer = ActivityScorer::new();
        let snapshot = ActivitySnapshot {
            window_title: "main.rs - my-project - VS Code".to_string(),
            process_name: "code".to_string(),
            idle_seconds: 0,
            timestamp: 0,
        };

        let state = scorer.infer_state(&snapshot);
        assert_eq!(state, ActivityState::Coding);
    }

    #[test]
    fn test_typora_detection() {
        let scorer = ActivityScorer::new();
        let snapshot = ActivitySnapshot {
            window_title: "笔记.md - Typora".to_string(),
            process_name: "typora".to_string(),
            idle_seconds: 0,
            timestamp: 0,
        };

        let state = scorer.infer_state(&snapshot);
        assert_eq!(state, ActivityState::Studying);
    }

    #[test]
    fn test_steam_detection() {
        let scorer = ActivityScorer::new();
        let snapshot = ActivitySnapshot {
            window_title: "Steam".to_string(),
            process_name: "steam".to_string(),
            idle_seconds: 0,
            timestamp: 0,
        };

        let state = scorer.infer_state(&snapshot);
        assert_eq!(state, ActivityState::Entertainment);
    }

    #[test]
    fn test_idle_detection() {
        let scorer = ActivityScorer::new();
        let snapshot = ActivitySnapshot {
            window_title: "Desktop".to_string(),
            process_name: "explorer".to_string(),
            idle_seconds: 600, // 10分钟
            timestamp: 0,
        };

        let state = scorer.infer_state(&snapshot);
        assert_eq!(state, ActivityState::Idle);
    }

    #[test]
    fn test_unknown_detection() {
        let scorer = ActivityScorer::new();
        let snapshot = ActivitySnapshot {
            window_title: "Some Random App".to_string(),
            process_name: "random_app".to_string(),
            idle_seconds: 0,
            timestamp: 0,
        };

        let state = scorer.infer_state(&snapshot);
        assert_eq!(state, ActivityState::Unknown);
    }
}
