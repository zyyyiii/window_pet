use async_trait::async_trait;

use super::types::{
    BehaviorContext, BehaviorError, BehaviorProviderType, BehaviorSuggestion, DialogueRequest,
    DialogueResponse, ReminderResponse, ReminderType,
};

/// 行为提供者统一接口
///
/// 所有行为提供者（规则驱动或 AI 驱动）都必须实现此接口。
/// AI 只能通过此接口影响对话、提醒和行为建议，不能直接操作系统。
#[async_trait]
pub trait BehaviorProvider: Send + Sync {
    /// 提供者名称
    fn name(&self) -> &str;

    /// 提供者类型
    fn provider_type(&self) -> BehaviorProviderType;

    /// 生成对话响应
    ///
    /// 根据上下文和请求生成合适的对话内容。
    /// AI 不能直接操作系统，只能返回对话文本。
    async fn generate_dialogue(
        &self,
        context: &BehaviorContext,
        request: &DialogueRequest,
    ) -> Result<DialogueResponse, BehaviorError>;

    /// 生成提醒内容
    ///
    /// 根据上下文和提醒类型生成提醒文本。
    async fn generate_reminder(
        &self,
        context: &BehaviorContext,
        reminder_type: ReminderType,
    ) -> Result<ReminderResponse, BehaviorError>;

    /// 生成行为建议
    ///
    /// 基于当前上下文给出行为建议。
    /// 建议包含置信度，只有高置信度的建议才会被执行。
    async fn suggest_behavior(
        &self,
        context: &BehaviorContext,
    ) -> Result<BehaviorSuggestion, BehaviorError>;

    /// 检查提供者是否可用
    ///
    /// 对于本地规则提供者，始终返回 true。
    /// 对于 AI 提供者，检查 API 连通性。
    async fn health_check(&self) -> bool;
}
