use super::types::{StudyMode, StudySession, StudyStats};
use chrono::Utc;
use uuid::Uuid;

/// 学习计时器
#[derive(Debug)]
pub struct StudyTimer {
    /// 当前会话
    current_session: Option<StudySession>,
    /// 统计数据
    stats: StudyStats,
    /// 会话开始时间（用于计算持续时长）
    session_start: Option<i64>,
    /// 休息开始时间
    break_start: Option<i64>,
}

impl StudyTimer {
    pub fn new() -> Self {
        Self {
            current_session: None,
            stats: StudyStats::default(),
            session_start: None,
            break_start: None,
        }
    }

    /// 开始新的学习会话
    pub fn start_session(&mut self, mode: StudyMode) {
        let now = Utc::now().timestamp();
        let session_id = Uuid::new_v4().to_string();

        let session = StudySession::new(session_id, mode.clone(), now);
        self.current_session = Some(session);
        self.session_start = Some(now);
        self.break_start = None;

        // 增加会话计数
        if mode == StudyMode::Study {
            self.stats.session_count += 1;
        }
    }

    /// 切换模式
    pub fn switch_mode(&mut self, new_mode: StudyMode) {
        let now = Utc::now().timestamp();

        if let Some(ref mut session) = self.current_session {
            // 如果从学习切换到休息，记录休息开始时间
            if session.mode == StudyMode::Study && new_mode == StudyMode::Break {
                self.break_start = Some(now);
                session.break_count += 1;
            }

            // 如果从休息切换回学习，累加休息时长
            if session.mode == StudyMode::Break && new_mode == StudyMode::Study {
                if let Some(break_start) = self.break_start {
                    let break_duration = (now - break_start).max(0) as u64;
                    session.total_break_duration += break_duration;
                    self.stats.total_break_time += break_duration;
                    self.break_start = None;
                }
            }

            session.mode = new_mode;
        } else {
            // 没有当前会话，启动新会话
            self.start_session(new_mode);
        }
    }

    /// 结束当前会话
    pub fn end_session(&mut self) {
        let now = Utc::now().timestamp();

        if let Some(ref mut session) = self.current_session {
            session.end_time = Some(now);

            // 计算会话时长
            let total_duration = (now - session.start_time).max(0) as u64;
            session.duration_secs = total_duration;

            // 累加到统计
            match session.mode {
                StudyMode::Study => {
                    self.stats.total_study_time += total_duration - session.total_break_duration;
                }
                StudyMode::Break => {
                    self.stats.total_break_time += total_duration;
                }
                _ => {}
            }

            // 处理可能未结束的休息
            if let Some(break_start) = self.break_start {
                let break_duration = (now - break_start).max(0) as u64;
                session.total_break_duration += break_duration;
                self.stats.total_break_time += break_duration;
                self.break_start = None;
            }
        }

        self.current_session = None;
        self.session_start = None;
    }

    /// 更新当前会话时长（定期调用）
    pub fn update(&mut self) {
        let now = Utc::now().timestamp();

        if let Some(ref mut session) = self.current_session {
            if let Some(start) = self.session_start {
                session.duration_secs = (now - start).max(0) as u64;
            }
        }
    }

    /// 获取当前会话时长（秒）
    pub fn get_session_duration(&self) -> u64 {
        if let Some(ref session) = self.current_session {
            let now = Utc::now().timestamp();
            (now - session.start_time).max(0) as u64
        } else {
            0
        }
    }

    /// 获取当前休息时长（秒）
    pub fn get_break_duration(&self) -> u64 {
        if let Some(break_start) = self.break_start {
            let now = Utc::now().timestamp();
            (now - break_start).max(0) as u64
        } else {
            0
        }
    }

    /// 获取当前会话
    pub fn get_current_session(&self) -> Option<&StudySession> {
        self.current_session.as_ref()
    }

    /// 获取统计数据
    pub fn get_stats(&self) -> &StudyStats {
        &self.stats
    }

    /// 获取可变统计数据引用
    pub fn get_stats_mut(&mut self) -> &mut StudyStats {
        &mut self.stats
    }

    /// 设置统计数据（用于加载持久化数据）
    pub fn set_stats(&mut self, stats: StudyStats) {
        self.stats = stats;
    }

    /// 检查是否需要学习提醒
    pub fn should_study_reminder(&self, interval: u64, last_reminder: Option<i64>) -> bool {
        if let Some(ref session) = self.current_session {
            if session.mode != StudyMode::Study {
                return false;
            }

            let now = Utc::now().timestamp();
            match last_reminder {
                Some(last) => now - last >= interval as i64,
                None => now - session.start_time >= interval as i64,
            }
        } else {
            false
        }
    }

    /// 检查是否需要休息提醒
    pub fn should_break_reminder(&self, interval: u64, last_reminder: Option<i64>) -> bool {
        if let Some(ref session) = self.current_session {
            if session.mode != StudyMode::Study {
                return false;
            }

            let now = Utc::now().timestamp();
            match last_reminder {
                Some(last) => now - last >= interval as i64,
                None => now - session.start_time >= interval as i64,
            }
        } else {
            false
        }
    }

    /// 检查是否需要单词测试
    pub fn should_word_test(&self, interval: u64, last_test: Option<i64>) -> bool {
        if let Some(ref session) = self.current_session {
            if session.mode != StudyMode::Study {
                return false;
            }

            let now = Utc::now().timestamp();
            match last_test {
                Some(last) => now - last >= interval as i64,
                None => now - session.start_time >= interval as i64,
            }
        } else {
            false
        }
    }

    /// 重置计时器
    pub fn reset(&mut self) {
        self.current_session = None;
        self.session_start = None;
        self.break_start = None;
    }
}

impl Default for StudyTimer {
    fn default() -> Self {
        Self::new()
    }
}
