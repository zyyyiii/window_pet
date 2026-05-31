use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::ai_provider::{AIProviderConfig, AIBehaviorProvider};
use super::context::ContextCollector;
use super::provider::BehaviorProvider;
use super::rule_provider::RuleBehaviorProvider;
use super::types::{
    AIProvider, BehaviorContext, BehaviorError, BehaviorManagerConfig, BehaviorSuggestion,
    DialogueMessage, DialogueRequest, DialogueResponse, MessageRole, ReminderResponse,
    ReminderType,
};

/// 行为管理器
///
/// 统一调度入口，管理多个行为提供者，根据配置选择活跃提供者。
pub struct BehaviorManager {
    /// 所有已注册的提供者
    providers: RwLock<HashMap<String, Arc<dyn BehaviorProvider>>>,
    /// 活跃提供者名称
    active_provider_name: RwLock<String>,
    /// 上下文收集器
    context_collector: Arc<ContextCollector>,
    /// 配置
    config: RwLock<BehaviorManagerConfig>,
}

impl BehaviorManager {
    pub fn new() -> Self {
        let mut providers: HashMap<String, Arc<dyn BehaviorProvider>> = HashMap::new();

        // 注册默认的规则提供者
        let rule_provider = Arc::new(RuleBehaviorProvider::new());
        providers.insert("rule".to_string(), rule_provider);

        Self {
            providers: RwLock::new(providers),
            active_provider_name: RwLock::new("rule".to_string()),
            context_collector: Arc::new(ContextCollector::new(50)),
            config: RwLock::new(BehaviorManagerConfig::default()),
        }
    }

    /// 获取活跃提供者（克隆 Arc，不持有锁）
    fn get_active_provider(&self) -> Result<Arc<dyn BehaviorProvider>, BehaviorError> {
        let name = self
            .active_provider_name
            .read()
            .map_err(|_| BehaviorError::ProviderUnavailable)?
            .clone();

        let providers = self
            .providers
            .read()
            .map_err(|_| BehaviorError::ProviderUnavailable)?;

        providers
            .get(&name)
            .cloned()
            .ok_or(BehaviorError::ProviderUnavailable)
    }

    /// 切换活跃提供者
    pub fn switch_provider(&self, name: &str) -> Result<(), BehaviorError> {
        let providers = self
            .providers
            .read()
            .map_err(|_| BehaviorError::ProviderUnavailable)?;

        if providers.contains_key(name) {
            let mut active = self
                .active_provider_name
                .write()
                .map_err(|_| BehaviorError::ProviderUnavailable)?;
            *active = name.to_string();

            let mut config = self
                .config
                .write()
                .map_err(|_| BehaviorError::ProviderUnavailable)?;
            config.active_provider = name.to_string();

            Ok(())
        } else {
            Err(BehaviorError::ProviderUnavailable)
        }
    }

    /// 注册新的 AI 提供者
    pub fn register_ai_provider(
        &self,
        provider: AIProvider,
        config: AIProviderConfig,
    ) -> Result<(), BehaviorError> {
        let ai_provider = AIBehaviorProvider::new(provider.clone(), config);
        let name = ai_provider.name().to_string();

        let mut providers = self
            .providers
            .write()
            .map_err(|_| BehaviorError::ProviderUnavailable)?;
        providers.insert(name, Arc::new(ai_provider));

        Ok(())
    }

    /// 获取已注册的提供者列表
    pub fn get_providers(&self) -> Result<Vec<String>, BehaviorError> {
        let providers = self
            .providers
            .read()
            .map_err(|_| BehaviorError::ProviderUnavailable)?;

        Ok(providers.keys().cloned().collect())
    }

    /// 获取当前活跃提供者名称
    pub fn get_active_provider_name(&self) -> Result<String, BehaviorError> {
        self.active_provider_name
            .read()
            .map(|name| name.clone())
            .map_err(|_| BehaviorError::ProviderUnavailable)
    }

    /// 生成对话（返回 future，不持有锁）
    pub fn generate_dialogue(
        &self,
        context: &BehaviorContext,
        request: &DialogueRequest,
    ) -> impl std::future::Future<Output = Result<DialogueResponse, BehaviorError>> + Send {
        let provider = self.get_active_provider();
        let context = context.clone();
        let request = request.clone();
        let context_collector = self.context_collector.clone();

        async move {
            let provider = provider?;
            let response = provider.generate_dialogue(&context, &request).await?;

            // 记录到历史
            context_collector.record_message(DialogueMessage {
                role: MessageRole::Pet,
                content: response.text.clone(),
                timestamp: chrono::Utc::now().timestamp(),
                dialogue_type: Some(response.dialogue_type.clone()),
            });

            Ok(response)
        }
    }

    /// 生成提醒（返回 future，不持有锁）
    pub fn generate_reminder(
        &self,
        context: &BehaviorContext,
        reminder_type: ReminderType,
    ) -> impl std::future::Future<Output = Result<ReminderResponse, BehaviorError>> + Send {
        let provider = self.get_active_provider();
        let context = context.clone();

        async move {
            let provider = provider?;
            provider.generate_reminder(&context, reminder_type).await
        }
    }

    /// 获取行为建议（返回 future，不持有锁）
    pub fn get_behavior_suggestion(
        &self,
        context: &BehaviorContext,
    ) -> impl std::future::Future<Output = Result<Option<BehaviorSuggestion>, BehaviorError>> + Send
    {
        let provider = self.get_active_provider();
        let context = context.clone();

        // 读取配置并克隆，然后释放锁
        let config = self
            .config
            .read()
            .map(|config| config.clone())
            .ok();

        async move {
            let config = config.ok_or(BehaviorError::ProviderUnavailable)?;

            if !config.enable_suggestions {
                return Ok(None);
            }

            let provider = provider?;
            let suggestion = provider.suggest_behavior(&context).await?;

            // 只有置信度超过阈值才返回
            if suggestion.confidence >= config.confidence_threshold {
                Ok(Some(suggestion))
            } else {
                Ok(None)
            }
        }
    }

    /// 记录用户消息
    pub fn record_user_message(&self, content: &str) {
        self.context_collector.record_message(DialogueMessage {
            role: MessageRole::User,
            content: content.to_string(),
            timestamp: chrono::Utc::now().timestamp(),
            dialogue_type: None,
        });
    }

    /// 清空对话历史
    pub fn clear_history(&self) {
        self.context_collector.clear_history();
    }

    /// 获取配置
    pub fn get_config(&self) -> Result<BehaviorManagerConfig, BehaviorError> {
        self.config
            .read()
            .map(|config| config.clone())
            .map_err(|_| BehaviorError::ProviderUnavailable)
    }

    /// 更新配置
    pub fn update_config(&self, config: BehaviorManagerConfig) -> Result<(), BehaviorError> {
        let mut current_config = self
            .config
            .write()
            .map_err(|_| BehaviorError::ProviderUnavailable)?;
        *current_config = config;
        Ok(())
    }
}

impl Default for BehaviorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 线程安全的行为管理器包装
pub struct SharedBehaviorManager {
    manager: BehaviorManager,
}

impl SharedBehaviorManager {
    pub fn new() -> Self {
        Self {
            manager: BehaviorManager::new(),
        }
    }

    pub fn get_manager(&self) -> &BehaviorManager {
        &self.manager
    }
}

impl Default for SharedBehaviorManager {
    fn default() -> Self {
        Self::new()
    }
}
