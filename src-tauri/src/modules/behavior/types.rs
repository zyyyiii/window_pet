use serde::{Deserialize, Serialize};

/// 行为提供者类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BehaviorProviderType {
    /// 规则驱动（本地）
    Rule,
    /// AI 驱动（远程 API）
    AI(AIProvider),
}

/// AI 提供者标识
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIProvider {
    DeepSeek,
    Gemini,
    Claude,
    OpenAI,
    Custom(String),
}

impl AIProvider {
    pub fn as_str(&self) -> &str {
        match self {
            AIProvider::DeepSeek => "deepseek",
            AIProvider::Gemini => "gemini",
            AIProvider::Claude => "claude",
            AIProvider::OpenAI => "openai",
            AIProvider::Custom(name) => name,
        }
    }
}

/// 对话触发类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialogueTrigger {
    /// 触摸
    Touch,
    /// 双击
    DoubleTouch,
    /// 气泡触发
    Bubble,
    /// 学习提醒
    StudyReminder,
    /// 休息提醒
    BreakReminder,
    /// 用户主动输入
    UserInput,
    /// 定时触发
    Scheduled,
}

/// 对话类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialogueType {
    /// 日常闲聊
    Chat,
    /// 学习相关
    Study,
    /// 鼓励
    Encouragement,
    /// 提醒
    Reminder,
    /// 进度询问
    Progress,
    /// 知识问答
    Knowledge,
}

/// 对话请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueRequest {
    /// 用户输入（可选）
    pub user_input: Option<String>,
    /// 对话触发类型
    pub trigger: DialogueTrigger,
    /// 对话类型偏好
    pub preferred_type: Option<DialogueType>,
}

/// 对话响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    /// 响应文本
    pub text: String,
    /// 情绪标签
    pub emotion: Option<String>,
    /// 动画提示
    pub animation_hint: Option<String>,
    /// 对话类型
    pub dialogue_type: DialogueType,
    /// 是否需要后续对话
    pub follow_up: bool,
}

/// 提醒类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReminderType {
    /// 学习提醒
    Study,
    /// 休息提醒
    Break,
    /// 喝水提醒
    DrinkWater,
    /// 进度检查
    ProgressCheck,
    /// 自定义
    Custom(String),
}

/// 提醒优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReminderPriority {
    Low,
    Medium,
    High,
}

/// 提醒响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReminderResponse {
    /// 提醒文本
    pub text: String,
    /// 提醒优先级
    pub priority: ReminderPriority,
    /// 是否需要用户确认
    pub require_ack: bool,
}

/// 建议的行为
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestedAction {
    /// 切换到学习模式
    EnterStudyMode,
    /// 切换到休息模式
    EnterBreakMode,
    /// 恢复普通模式
    EnterNormalMode,
    /// 显示鼓励消息
    ShowEncouragement,
    /// 显示提醒
    ShowReminder(String),
    /// 无建议
    None,
}

/// 行为建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSuggestion {
    /// 建议的行为类型
    pub action: SuggestedAction,
    /// 建议理由
    pub reason: String,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
}

/// 行为上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorContext {
    /// 宠物状态
    pub pet: PetContext,
    /// 用户活动
    pub activity: ActivityContext,
    /// 学习状态
    pub study: StudyContext,
    /// 情绪状态
    pub mood: MoodContext,
    /// 对话历史
    pub dialogue_history: Vec<DialogueMessage>,
    /// 时间信息
    pub time: TimeContext,
}

/// 宠物上下文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PetContext {
    pub name: String,
    pub state: String,
    pub hunger: f64,
    pub mood_value: f64,
    pub energy: f64,
    pub cleanliness: f64,
    pub health: f64,
}

/// 活动上下文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityContext {
    pub current_state: String,
    pub window_title: String,
    pub process_name: String,
    pub idle_seconds: u64,
}

/// 学习上下文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudyContext {
    pub mode: String,
    pub study_duration_secs: u64,
    pub break_duration_secs: u64,
    pub session_count: u32,
    pub total_study_time: u64,
}

/// 情绪上下文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoodContext {
    pub state: String,
    pub value: f64,
}

/// 对话消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueMessage {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: i64,
    pub dialogue_type: Option<DialogueType>,
}

/// 消息角色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageRole {
    /// 宠物（AI）
    Pet,
    /// 用户
    User,
    /// 系统
    System,
}

/// 时间上下文
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeContext {
    pub timestamp: i64,
    pub hour: u32,
    pub weekday: u32,
    pub is_workday: bool,
}

impl Default for TimeContext {
    fn default() -> Self {
        Self {
            timestamp: 0,
            hour: 12,
            weekday: 3,
            is_workday: true,
        }
    }
}

/// AI 提供者配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    /// API 端点
    pub api_endpoint: String,
    /// API 密钥
    pub api_key: Option<String>,
    /// 模型名称
    pub model: String,
    /// 最大 token 数
    pub max_tokens: u32,
    /// 温度参数
    pub temperature: f32,
    /// 超时时间（秒）
    pub timeout_secs: u64,
}

/// 行为管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorManagerConfig {
    /// 活跃提供者名称
    pub active_provider: String,
    /// 置信度阈值
    pub confidence_threshold: f64,
    /// 是否启用行为建议
    pub enable_suggestions: bool,
    /// 对话历史最大条数
    pub max_dialogue_history: usize,
}

impl Default for BehaviorManagerConfig {
    fn default() -> Self {
        Self {
            active_provider: "rule".to_string(),
            confidence_threshold: 0.7,
            enable_suggestions: true,
            max_dialogue_history: 50,
        }
    }
}

/// 行为错误
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorError {
    NetworkError(String),
    ApiError(String),
    InsufficientContext(String),
    Timeout,
    ProviderUnavailable,
    Unknown(String),
}

impl std::fmt::Display for BehaviorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BehaviorError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            BehaviorError::ApiError(msg) => write!(f, "API 错误: {}", msg),
            BehaviorError::InsufficientContext(msg) => write!(f, "上下文不足: {}", msg),
            BehaviorError::Timeout => write!(f, "超时"),
            BehaviorError::ProviderUnavailable => write!(f, "提供者不可用"),
            BehaviorError::Unknown(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for BehaviorError {}
