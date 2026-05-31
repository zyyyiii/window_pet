use chrono::Utc;

use super::dialogue::StudyDialogue;
use super::timer::StudyTimer;
use super::types::{
    StudyDialogueConfig, StudyDialogueType, StudyMode, StudyReminderConfig, StudySnapshot,
};

/// 学习管理器
#[derive(Debug)]
pub struct StudyManager {
    /// 当前模式
    current_mode: StudyMode,
    /// 计时器
    timer: StudyTimer,
    /// 对话内容库
    dialogue: StudyDialogue,
    /// 提醒配置
    reminder_config: StudyReminderConfig,
    /// 上次学习提醒时间
    last_study_reminder: Option<i64>,
    /// 上次休息提醒时间
    last_break_reminder: Option<i64>,
    /// 上次单词测试时间
    last_word_test: Option<i64>,
}

impl StudyManager {
    pub fn new() -> Self {
        Self {
            current_mode: StudyMode::Normal,
            timer: StudyTimer::new(),
            dialogue: StudyDialogue::new(),
            reminder_config: StudyReminderConfig::default(),
            last_study_reminder: None,
            last_break_reminder: None,
            last_word_test: None,
        }
    }

    /// 获取当前模式
    pub fn get_mode(&self) -> &StudyMode {
        &self.current_mode
    }

    /// 设置模式
    pub fn set_mode(&mut self, mode: StudyMode) -> StudySnapshot {
        // 如果模式没变化，直接返回快照
        if self.current_mode == mode {
            return self.get_snapshot();
        }

        let old_mode = self.current_mode.clone();

        // 如果从普通模式切换到学习/休息模式，启动新会话
        if old_mode == StudyMode::Normal && mode != StudyMode::Normal {
            self.timer.start_session(mode.clone());
        }
        // 如果从学习/休息模式切换到普通模式，结束会话
        else if old_mode != StudyMode::Normal && mode == StudyMode::Normal {
            self.timer.end_session();
        }
        // 如果在学习和休息之间切换
        else if old_mode != StudyMode::Normal && mode != StudyMode::Normal {
            self.timer.switch_mode(mode.clone());
        }

        self.current_mode = mode;

        // 重置提醒时间
        self.last_study_reminder = None;
        self.last_break_reminder = None;
        self.last_word_test = None;

        self.get_snapshot()
    }

    /// 获取快照
    pub fn get_snapshot(&self) -> StudySnapshot {
        StudySnapshot {
            mode: self.current_mode.clone(),
            current_session: self.timer.get_current_session().cloned(),
            stats: self.timer.get_stats().clone(),
            last_study_reminder: self.last_study_reminder,
            last_break_reminder: self.last_break_reminder,
            last_word_test: self.last_word_test,
        }
    }

    /// 获取对话配置（供 DialogueManager 查询）
    pub fn get_dialogue_config(&self) -> StudyDialogueConfig {
        StudyDialogueConfig::for_mode(&self.current_mode)
    }

    /// 检查是否需要提醒
    pub fn check_reminder(&mut self) -> Option<StudyDialogueType> {
        match self.current_mode {
            StudyMode::Study => {
                // 检查学习提醒
                if self.timer.should_study_reminder(
                    self.reminder_config.study_reminder_interval,
                    self.last_study_reminder,
                ) {
                    self.last_study_reminder = Some(Utc::now().timestamp());
                    return Some(StudyDialogueType::StudyReminder);
                }

                // 检查休息提醒
                if self.timer.should_break_reminder(
                    self.reminder_config.break_reminder_interval,
                    self.last_break_reminder,
                ) {
                    self.last_break_reminder = Some(Utc::now().timestamp());
                    return Some(StudyDialogueType::BreakReminder);
                }

                // 检查单词测试
                if self.reminder_config.enable_word_test
                    && self.timer.should_word_test(
                        self.reminder_config.word_test_interval,
                        self.last_word_test,
                    )
                {
                    self.last_word_test = Some(Utc::now().timestamp());
                    return Some(StudyDialogueType::WordTest);
                }

                None
            }
            StudyMode::Break => {
                // 休息模式下偶尔也提醒休息
                if self.timer.should_break_reminder(
                    self.reminder_config.break_reminder_interval * 2,
                    self.last_break_reminder,
                ) {
                    self.last_break_reminder = Some(Utc::now().timestamp());
                    return Some(StudyDialogueType::BreakReminder);
                }

                None
            }
            StudyMode::Normal => None,
        }
    }

    /// 获取随机对话
    pub fn get_random_dialogue(&self) -> Option<String> {
        if let Some(dialogue_type) = self.dialogue.get_random_dialogue_type(&self.current_mode) {
            self.dialogue.get_dialogue(&dialogue_type)
        } else {
            None
        }
    }

    /// 获取特定类型的对话
    pub fn get_dialogue(&self, dialogue_type: &StudyDialogueType) -> Option<String> {
        self.dialogue.get_dialogue(dialogue_type)
    }

    /// 更新计时器（定期调用）
    pub fn update(&mut self) {
        self.timer.update();
    }

    /// 获取学习时长（秒）
    pub fn get_study_duration(&self) -> u64 {
        self.timer.get_session_duration()
    }

    /// 获取休息时长（秒）
    pub fn get_break_duration(&self) -> u64 {
        self.timer.get_break_duration()
    }

    /// 获取提醒配置
    pub fn get_reminder_config(&self) -> &StudyReminderConfig {
        &self.reminder_config
    }

    /// 设置提醒配置
    pub fn set_reminder_config(&mut self, config: StudyReminderConfig) {
        self.reminder_config = config;
    }

    /// 添加自定义学习提醒
    pub fn add_study_reminder(&mut self, message: String) {
        self.dialogue.add_study_reminder(message);
    }

    /// 添加自定义休息提醒
    pub fn add_break_reminder(&mut self, message: String) {
        self.dialogue.add_break_reminder(message);
    }

    /// 添加自定义鼓励消息
    pub fn add_encouragement(&mut self, message: String) {
        self.dialogue.add_encouragement(message);
    }

    /// 重置管理器
    pub fn reset(&mut self) {
        self.current_mode = StudyMode::Normal;
        self.timer.reset();
        self.last_study_reminder = None;
        self.last_break_reminder = None;
        self.last_word_test = None;
    }
}

impl Default for StudyManager {
    fn default() -> Self {
        Self::new()
    }
}
