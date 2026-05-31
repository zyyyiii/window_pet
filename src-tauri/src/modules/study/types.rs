use serde::{Deserialize, Serialize};

/// 学习模式枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudyMode {
    /// 普通模式
    Normal,
    /// 学习模式
    Study,
    /// 休息模式
    Break,
}

impl StudyMode {
    pub fn as_str(&self) -> &str {
        match self {
            StudyMode::Normal => "normal",
            StudyMode::Study => "study",
            StudyMode::Break => "break",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            StudyMode::Normal => "普通模式",
            StudyMode::Study => "学习模式",
            StudyMode::Break => "休息模式",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "study" => StudyMode::Study,
            "break" => StudyMode::Break,
            _ => StudyMode::Normal,
        }
    }
}

impl Default for StudyMode {
    fn default() -> Self {
        StudyMode::Normal
    }
}

/// 学习会话数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySession {
    /// 会话ID
    pub id: String,
    /// 当前模式
    pub mode: StudyMode,
    /// 开始时间 (Unix timestamp)
    pub start_time: i64,
    /// 结束时间
    pub end_time: Option<i64>,
    /// 持续时长（秒）
    pub duration_secs: u64,
    /// 休息次数
    pub break_count: u32,
    /// 总休息时长（秒）
    pub total_break_duration: u64,
}

impl StudySession {
    pub fn new(id: String, mode: StudyMode, start_time: i64) -> Self {
        Self {
            id,
            mode,
            start_time,
            end_time: None,
            duration_secs: 0,
            break_count: 0,
            total_break_duration: 0,
        }
    }
}

/// 学习统计数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyStats {
    /// 总学习时长（秒）
    pub total_study_time: u64,
    /// 总休息时长（秒）
    pub total_break_time: u64,
    /// 会话次数
    pub session_count: u32,
    /// 连续学习天数
    pub current_streak: u32,
    /// 最长连续天数
    pub longest_streak: u32,
}

impl Default for StudyStats {
    fn default() -> Self {
        Self {
            total_study_time: 0,
            total_break_time: 0,
            session_count: 0,
            current_streak: 0,
            longest_streak: 0,
        }
    }
}

/// 学习提醒配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyReminderConfig {
    /// 学习提醒间隔（秒），默认 1800 (30分钟)
    pub study_reminder_interval: u64,
    /// 休息提醒间隔（秒），默认 600 (10分钟)
    pub break_reminder_interval: u64,
    /// 单词测试间隔（秒），默认 3600 (1小时)
    pub word_test_interval: u64,
    /// 是否启用单词测试
    pub enable_word_test: bool,
}

impl Default for StudyReminderConfig {
    fn default() -> Self {
        Self {
            study_reminder_interval: 1800,  // 30分钟
            break_reminder_interval: 600,   // 10分钟
            word_test_interval: 3600,       // 1小时
            enable_word_test: true,
        }
    }
}

/// 学习模式快照（前端通信用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySnapshot {
    /// 当前模式
    pub mode: StudyMode,
    /// 当前会话
    pub current_session: Option<StudySession>,
    /// 统计数据
    pub stats: StudyStats,
    /// 上次学习提醒时间
    pub last_study_reminder: Option<i64>,
    /// 上次休息提醒时间
    pub last_break_reminder: Option<i64>,
    /// 上次单词测试时间
    pub last_word_test: Option<i64>,
}

/// 学习对话类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StudyDialogueType {
    /// 学习提醒
    StudyReminder,
    /// 休息提醒
    BreakReminder,
    /// 单词测试
    WordTest,
    /// 鼓励
    Encouragement,
    /// 进度询问
    Progress,
}

/// 学习对话配置（供 DialogueManager 使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyDialogueConfig {
    /// 学习对话概率 (0.0 - 1.0)
    pub learning_probability: f64,
    /// 娱乐对话概率 (0.0 - 1.0)
    pub entertainment_probability: f64,
    /// 提醒间隔（秒）
    pub reminder_interval: u64,
}

impl StudyDialogueConfig {
    pub fn for_mode(mode: &StudyMode) -> Self {
        match mode {
            StudyMode::Study => Self {
                learning_probability: 0.7,
                entertainment_probability: 0.1,
                reminder_interval: 1800,
            },
            StudyMode::Break => Self {
                learning_probability: 0.1,
                entertainment_probability: 0.6,
                reminder_interval: 600,
            },
            StudyMode::Normal => Self {
                learning_probability: 0.2,
                entertainment_probability: 0.4,
                reminder_interval: 3600,
            },
        }
    }
}
