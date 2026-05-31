use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::provider::BehaviorProvider;
use super::types::{
    AIProvider, BehaviorContext, BehaviorError, BehaviorProviderType,
    BehaviorSuggestion, DialogueRequest, DialogueResponse, DialogueTrigger, DialogueType,
    ReminderPriority, ReminderResponse, ReminderType,
    SuggestedAction, MessageRole,
};

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

/// Prompt 模板
pub struct PromptTemplates {
    /// 系统提示词
    pub system_prompt: String,
    /// 对话生成模板
    pub dialogue_template: String,
    /// 提醒生成模板
    pub reminder_template: String,
    /// 行为建议模板
    pub behavior_template: String,
}

/// AI 驱动的行为提供者
///
/// 通过远程 AI API 生成对话和建议。
/// 使用 OpenAI 兼容格式，支持 DeepSeek/Gemini/Claude/OpenAI 等。
#[allow(dead_code)]
pub struct AIBehaviorProvider {
    /// 提供者类型
    provider: AIProvider,
    /// 配置
    config: AIProviderConfig,
    /// Prompt 模板
    prompts: PromptTemplates,
}

impl AIBehaviorProvider {
    pub fn new(provider: AIProvider, config: AIProviderConfig) -> Self {
        let prompts = PromptTemplates {
            system_prompt: Self::default_system_prompt(),
            dialogue_template: Self::default_dialogue_template(),
            reminder_template: Self::default_reminder_template(),
            behavior_template: Self::default_behavior_template(),
        };

        Self {
            provider,
            config,
            prompts,
        }
    }

    /// 默认系统提示词
    fn default_system_prompt() -> String {
        r#"你是一个可爱的桌面宠物猫，名叫{name}。你的性格是温暖、有趣、关心主人的学习和生活。

当前状态：
- 情绪：{mood_state}（数值：{mood_value}）
- 饥饿度：{hunger}
- 精力：{energy}

当前用户活动：{activity_state}
学习模式：{study_mode}
已学习时长：{study_duration}分钟

你的能力范围：
1. 与主人聊天，提供情感支持
2. 提醒主人学习或休息
3. 建议主人的行为（但不能直接控制）

你不能做的事：
1. 不能直接操作系统
2. 不能读取浏览器内容
3. 不能获取隐私信息

请用中文回复，语气可爱、简短（不超过50字）。"#
            .to_string()
    }

    /// 默认对话模板
    fn default_dialogue_template() -> String {
        r#"{system_prompt}

最近对话历史：
{dialogue_history}

用户当前输入：{user_input}
触发类型：{trigger_type}
期望对话类型：{preferred_type}

请生成一个合适的回应。只返回对话文本，不要其他内容。"#
            .to_string()
    }

    /// 默认提醒模板
    fn default_reminder_template() -> String {
        r#"{system_prompt}

提醒类型：{reminder_type}
学习时长：{study_duration}分钟
休息时长：{break_duration}分钟

请生成一个温馨的提醒消息。只返回提醒文本，不要其他内容。"#
            .to_string()
    }

    /// 默认行为建议模板
    fn default_behavior_template() -> String {
        r#"{system_prompt}

当前上下文：
- 用户活动：{activity_state}
- 学习模式：{study_mode}
- 学习时长：{study_duration}分钟
- 情绪状态：{mood_state}

请分析当前情况并给出行为建议。
返回 JSON 格式：
{
  "action": "enter_study_mode|enter_break_mode|enter_normal_mode|show_encouragement|show_reminder|none",
  "reason": "建议理由",
  "confidence": 0.0-1.0
}"#
            .to_string()
    }

    /// 构建系统提示词
    fn build_system_prompt(&self, context: &BehaviorContext) -> String {
        self.prompts
            .system_prompt
            .replace("{name}", &context.pet.name)
            .replace("{mood_state}", &context.mood.state)
            .replace("{mood_value}", &format!("{:.0}", context.mood.value))
            .replace("{hunger}", &format!("{:.0}", context.pet.hunger))
            .replace("{energy}", &format!("{:.0}", context.pet.energy))
            .replace("{activity_state}", &context.activity.current_state)
            .replace("{study_mode}", &context.study.mode)
            .replace(
                "{study_duration}",
                &format!("{:.0}", context.study.study_duration_secs as f64 / 60.0),
            )
    }

    /// 构建对话 prompt
    fn build_dialogue_prompt(&self, context: &BehaviorContext, request: &DialogueRequest) -> String {
        let system_prompt = self.build_system_prompt(context);

        let dialogue_history = context
            .dialogue_history
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    super::types::MessageRole::Pet => "宠物",
                    super::types::MessageRole::User => "用户",
                    super::types::MessageRole::System => "系统",
                };
                format!("{}: {}", role, msg.content)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let trigger_type = match &request.trigger {
            DialogueTrigger::Touch => "触摸",
            DialogueTrigger::DoubleTouch => "双击",
            DialogueTrigger::Bubble => "气泡",
            DialogueTrigger::StudyReminder => "学习提醒",
            DialogueTrigger::BreakReminder => "休息提醒",
            DialogueTrigger::UserInput => "用户输入",
            DialogueTrigger::Scheduled => "定时",
        };

        let preferred_type = request
            .preferred_type
            .as_ref()
            .map(|t| match t {
                DialogueType::Chat => "闲聊",
                DialogueType::Study => "学习",
                DialogueType::Encouragement => "鼓励",
                DialogueType::Reminder => "提醒",
                DialogueType::Progress => "进度",
                DialogueType::Knowledge => "知识",
            })
            .unwrap_or("任意");

        self.prompts
            .dialogue_template
            .replace("{system_prompt}", &system_prompt)
            .replace("{dialogue_history}", &dialogue_history)
            .replace(
                "{user_input}",
                request.user_input.as_deref().unwrap_or("无"),
            )
            .replace("{trigger_type}", trigger_type)
            .replace("{preferred_type}", preferred_type)
    }

    /// 构建提醒 prompt
    fn build_reminder_prompt(&self, context: &BehaviorContext, reminder_type: &ReminderType) -> String {
        let system_prompt = self.build_system_prompt(context);

        let reminder_type_str = match reminder_type {
            ReminderType::Study => "学习提醒",
            ReminderType::Break => "休息提醒",
            ReminderType::DrinkWater => "喝水提醒",
            ReminderType::ProgressCheck => "进度检查",
            ReminderType::Custom(_) => "自定义提醒",
        };

        self.prompts
            .reminder_template
            .replace("{system_prompt}", &system_prompt)
            .replace("{reminder_type}", reminder_type_str)
            .replace(
                "{study_duration}",
                &format!("{:.0}", context.study.study_duration_secs as f64 / 60.0),
            )
            .replace(
                "{break_duration}",
                &format!("{:.0}", context.study.break_duration_secs as f64 / 60.0),
            )
    }

    /// 构建行为建议 prompt
    fn build_behavior_prompt(&self, context: &BehaviorContext) -> String {
        let system_prompt = self.build_system_prompt(context);

        self.prompts
            .behavior_template
            .replace("{system_prompt}", &system_prompt)
            .replace("{activity_state}", &context.activity.current_state)
            .replace("{study_mode}", &context.study.mode)
            .replace(
                "{study_duration}",
                &format!("{:.0}", context.study.study_duration_secs as f64 / 60.0),
            )
            .replace("{mood_state}", &context.mood.state)
    }

    /// 调用 AI API（通用接口）
    async fn call_api(&self, _prompt: &str) -> Result<String, BehaviorError> {
        // TODO: 实现实际的 API 调用
        // 这里只是接口预留，实际实现需要 HTTP 客户端
        Err(BehaviorError::ProviderUnavailable)
    }

    /// 解析对话响应
    fn parse_dialogue_response(&self, response: &str) -> Result<DialogueResponse, BehaviorError> {
        // 简单解析：直接使用响应文本
        Ok(DialogueResponse {
            text: response.to_string(),
            emotion: Some("neutral".to_string()),
            animation_hint: None,
            dialogue_type: DialogueType::Chat,
            follow_up: false,
        })
    }

    /// 解析提醒响应
    fn parse_reminder_response(&self, response: &str) -> Result<ReminderResponse, BehaviorError> {
        Ok(ReminderResponse {
            text: response.to_string(),
            priority: ReminderPriority::Medium,
            require_ack: false,
        })
    }

    /// 解析行为建议
    fn parse_behavior_suggestion(
        &self,
        response: &str,
    ) -> Result<BehaviorSuggestion, BehaviorError> {
        // 尝试解析 JSON
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(response) {
            let action = match value["action"].as_str().unwrap_or("none") {
                "enter_study_mode" => SuggestedAction::EnterStudyMode,
                "enter_break_mode" => SuggestedAction::EnterBreakMode,
                "enter_normal_mode" => SuggestedAction::EnterNormalMode,
                "show_encouragement" => SuggestedAction::ShowEncouragement,
                "show_reminder" => {
                    SuggestedAction::ShowReminder(
                        value["reason"].as_str().unwrap_or("").to_string()
                    )
                }
                _ => SuggestedAction::None,
            };

            let reason = value["reason"].as_str().unwrap_or("").to_string();
            let confidence = value["confidence"].as_f64().unwrap_or(0.5);

            return Ok(BehaviorSuggestion {
                action,
                reason,
                confidence,
            });
        }

        // JSON 解析失败，返回默认建议
        Ok(BehaviorSuggestion {
            action: SuggestedAction::None,
            reason: "无法解析建议".to_string(),
            confidence: 0.0,
        })
    }
}

#[async_trait]
impl BehaviorProvider for AIBehaviorProvider {
    fn name(&self) -> &str {
        self.provider.as_str()
    }

    fn provider_type(&self) -> BehaviorProviderType {
        BehaviorProviderType::AI(self.provider.clone())
    }

    async fn generate_dialogue(
        &self,
        context: &BehaviorContext,
        request: &DialogueRequest,
    ) -> Result<DialogueResponse, BehaviorError> {
        let prompt = self.build_dialogue_prompt(context, request);
        let response = self.call_api(&prompt).await?;
        self.parse_dialogue_response(&response)
    }

    async fn generate_reminder(
        &self,
        context: &BehaviorContext,
        reminder_type: ReminderType,
    ) -> Result<ReminderResponse, BehaviorError> {
        let prompt = self.build_reminder_prompt(context, &reminder_type);
        let response = self.call_api(&prompt).await?;
        self.parse_reminder_response(&response)
    }

    async fn suggest_behavior(
        &self,
        context: &BehaviorContext,
    ) -> Result<BehaviorSuggestion, BehaviorError> {
        let prompt = self.build_behavior_prompt(context);
        let response = self.call_api(&prompt).await?;
        self.parse_behavior_suggestion(&response)
    }

    async fn health_check(&self) -> bool {
        // 尝试发送简单请求测试连通性
        // TODO: 实现实际的健康检查
        false
    }
}
