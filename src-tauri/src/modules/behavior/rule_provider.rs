use async_trait::async_trait;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::provider::BehaviorProvider;
use super::types::{
    BehaviorContext, BehaviorError, BehaviorProviderType, BehaviorSuggestion, DialogueRequest,
    DialogueResponse, DialogueTrigger, DialogueType, ReminderPriority, ReminderResponse,
    ReminderType, SuggestedAction,
};

/// 规则驱动的行为提供者
///
/// 基于本地规则生成对话和建议，不依赖外部 API。
/// 作为默认提供者和 AI 不可用时的降级方案。
pub struct RuleBehaviorProvider {
    /// 触摸响应
    touch_responses: Vec<String>,
    /// 学习提醒
    study_reminders: Vec<String>,
    /// 休息提醒
    break_reminders: Vec<String>,
    /// 鼓励消息
    encouragements: Vec<String>,
    /// 进度询问
    progress_questions: Vec<String>,
    /// 喝水提醒
    drink_water_reminders: Vec<String>,
    /// 日常对话
    chat_responses: Vec<String>,
}

impl RuleBehaviorProvider {
    pub fn new() -> Self {
        Self {
            touch_responses: vec![
                "喵~".to_string(),
                "摸摸头~".to_string(),
                "嗯？怎么了？".to_string(),
                "嘻嘻~".to_string(),
                "继续继续~".to_string(),
                "我在呢~".to_string(),
            ],
            study_reminders: vec![
                "今天的线代进度怎么样？".to_string(),
                "要不要做一道题？".to_string(),
                "已经学习30分钟了，继续加油！".to_string(),
                "专注学习，效率更高哦~".to_string(),
                "学习使我快乐！".to_string(),
                "来，我们一起学习吧！".to_string(),
                "今天的任务完成了多少？".to_string(),
                "休息够了，该学习啦~".to_string(),
            ],
            break_reminders: vec![
                "休息一下吧，别太累了~".to_string(),
                "记得喝水哦！".to_string(),
                "站起来活动活动~".to_string(),
                "看看远处，保护眼睛~".to_string(),
                "深呼吸，放松一下~".to_string(),
                "学习很重要，休息也很重要！".to_string(),
                "来，伸个懒腰~".to_string(),
                "休息是为了更好地学习！".to_string(),
            ],
            encouragements: vec![
                "你已经很努力了！".to_string(),
                "坚持就是胜利！".to_string(),
                "你真棒！继续加油！".to_string(),
                "学习的路上，我一直陪着你~".to_string(),
                "每一点进步都值得庆祝！".to_string(),
                "相信自己，你可以的！".to_string(),
                "今天的你比昨天更优秀！".to_string(),
                "知识就是力量！".to_string(),
            ],
            progress_questions: vec![
                "今天学了多久了？".to_string(),
                "学到什么有趣的内容了吗？".to_string(),
                "有没有遇到什么困难？".to_string(),
                "今天的计划完成了多少？".to_string(),
                "需要我帮你复习吗？".to_string(),
                "有没有什么想分享的？".to_string(),
            ],
            drink_water_reminders: vec![
                "记得喝水哦！💧".to_string(),
                "该喝水了~".to_string(),
                "补充水分，保持活力！".to_string(),
                "喝杯水休息一下~".to_string(),
            ],
            chat_responses: vec![
                "嗯嗯，我在听~".to_string(),
                "然后呢？".to_string(),
                "哈哈，真有趣！".to_string(),
                "我也是这么想的~".to_string(),
                "继续说~".to_string(),
                "喵~".to_string(),
            ],
        }
    }

    /// 根据上下文生成学习相关对话
    fn generate_study_dialogue(&self, context: &BehaviorContext) -> String {
        let mut rng = thread_rng();

        // 根据学习时长选择不同的对话
        if context.study.study_duration_secs > 3600 {
            // 学习超过1小时
            let messages = vec![
                "已经学习很久了，休息一下吧~".to_string(),
                "你已经很棒了！该休息了~".to_string(),
                "学习1小时了，站起来活动活动~".to_string(),
            ];
            return messages.choose(&mut rng).cloned().unwrap_or_default();
        }

        if context.study.study_duration_secs > 1800 {
            // 学习超过30分钟
            return self.study_reminders.choose(&mut rng).cloned().unwrap_or_default();
        }

        // 根据活动状态选择对话
        match context.activity.current_state.as_str() {
            "coding" => {
                let messages = vec![
                    "代码写得怎么样了？".to_string(),
                    "编程加油！".to_string(),
                    "需要我帮你看看代码吗？".to_string(),
                ];
                messages.choose(&mut rng).cloned().unwrap_or_default()
            }
            "studying" => {
                self.study_reminders.choose(&mut rng).cloned().unwrap_or_default()
            }
            _ => {
                self.progress_questions.choose(&mut rng).cloned().unwrap_or_default()
            }
        }
    }

    /// 根据上下文生成日常对话
    fn generate_chat_dialogue(&self, context: &BehaviorContext) -> String {
        let mut rng = thread_rng();

        // 根据情绪状态选择对话
        match context.mood.state.as_str() {
            "happy" => {
                let messages = vec![
                    "今天心情真好~".to_string(),
                    "嘻嘻，开心~".to_string(),
                    "喵~ 好开心！".to_string(),
                ];
                messages.choose(&mut rng).cloned().unwrap_or_default()
            }
            "bored" => {
                let messages = vec![
                    "好无聊啊~".to_string(),
                    "陪我玩嘛~".to_string(),
                    "想出去玩~".to_string(),
                ];
                messages.choose(&mut rng).cloned().unwrap_or_default()
            }
            "sleepy" => {
                let messages = vec![
                    "好困...".to_string(),
                    "想睡觉了~".to_string(),
                    "zzZ...".to_string(),
                ];
                messages.choose(&mut rng).cloned().unwrap_or_default()
            }
            _ => {
                self.chat_responses.choose(&mut rng).cloned().unwrap_or_default()
            }
        }
    }
}

impl Default for RuleBehaviorProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BehaviorProvider for RuleBehaviorProvider {
    fn name(&self) -> &str {
        "rule"
    }

    fn provider_type(&self) -> BehaviorProviderType {
        BehaviorProviderType::Rule
    }

    async fn generate_dialogue(
        &self,
        context: &BehaviorContext,
        request: &DialogueRequest,
    ) -> Result<DialogueResponse, BehaviorError> {
        let mut rng = thread_rng();

        let (text, dialogue_type, emotion, animation_hint) = match &request.trigger {
            DialogueTrigger::Touch => {
                let text = self.touch_responses.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "喵~".to_string());
                (text, DialogueType::Chat, Some("happy".to_string()), Some("happy".to_string()))
            }
            DialogueTrigger::DoubleTouch => {
                let text = "嗯？想聊什么？".to_string();
                (text, DialogueType::Chat, Some("curious".to_string()), Some("talking".to_string()))
            }
            DialogueTrigger::StudyReminder => {
                let text = self.generate_study_dialogue(context);
                (text, DialogueType::Study, Some("encouraging".to_string()), Some("studying".to_string()))
            }
            DialogueTrigger::BreakReminder => {
                let text = self.break_reminders.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "休息一下吧~".to_string());
                (text, DialogueType::Reminder, Some("caring".to_string()), Some("idle".to_string()))
            }
            DialogueTrigger::Bubble => {
                // 根据上下文选择气泡内容
                let text = if context.study.mode == "study" {
                    self.generate_study_dialogue(context)
                } else {
                    self.generate_chat_dialogue(context)
                };
                (text, DialogueType::Chat, Some("neutral".to_string()), None)
            }
            DialogueTrigger::UserInput => {
                // 用户主动输入，生成通用响应
                let text = self.chat_responses.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "嗯嗯~".to_string());
                (text, DialogueType::Chat, Some("happy".to_string()), None)
            }
            DialogueTrigger::Scheduled => {
                // 定时触发，根据上下文选择
                let text = if context.study.mode == "study" {
                    self.generate_study_dialogue(context)
                } else {
                    self.encouragements.choose(&mut rng).cloned()
                        .unwrap_or_else(|| "加油！".to_string())
                };
                (text, DialogueType::Encouragement, Some("encouraging".to_string()), None)
            }
        };

        Ok(DialogueResponse {
            text,
            emotion,
            animation_hint,
            dialogue_type,
            follow_up: false,
        })
    }

    async fn generate_reminder(
        &self,
        _context: &BehaviorContext,
        reminder_type: ReminderType,
    ) -> Result<ReminderResponse, BehaviorError> {
        let mut rng = thread_rng();

        let (text, priority) = match reminder_type {
            ReminderType::Study => {
                let text = self.study_reminders.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "该学习了！".to_string());
                (text, ReminderPriority::Medium)
            }
            ReminderType::Break => {
                let text = self.break_reminders.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "休息一下吧~".to_string());
                (text, ReminderPriority::Medium)
            }
            ReminderType::DrinkWater => {
                let text = self.drink_water_reminders.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "记得喝水哦！".to_string());
                (text, ReminderPriority::Low)
            }
            ReminderType::ProgressCheck => {
                let text = self.progress_questions.choose(&mut rng).cloned()
                    .unwrap_or_else(|| "进度怎么样了？".to_string());
                (text, ReminderPriority::Low)
            }
            ReminderType::Custom(msg) => {
                (msg, ReminderPriority::Medium)
            }
        };

        Ok(ReminderResponse {
            text,
            priority,
            require_ack: false,
        })
    }

    async fn suggest_behavior(
        &self,
        context: &BehaviorContext,
    ) -> Result<BehaviorSuggestion, BehaviorError> {
        // 基于规则的行为建议

        // 检查是否需要休息
        if context.study.mode == "study" && context.study.study_duration_secs > 3600 {
            return Ok(BehaviorSuggestion {
                action: SuggestedAction::ShowReminder(
                    "已经学习1小时了，休息一下吧~".to_string()
                ),
                reason: "学习时间过长".to_string(),
                confidence: 0.9,
            });
        }

        // 检查是否空闲
        if context.activity.current_state == "idle" && context.study.mode == "study" {
            return Ok(BehaviorSuggestion {
                action: SuggestedAction::EnterBreakMode,
                reason: "检测到空闲，建议休息".to_string(),
                confidence: 0.8,
            });
        }

        // 检查是否需要鼓励
        if context.mood.value < 30.0 {
            return Ok(BehaviorSuggestion {
                action: SuggestedAction::ShowEncouragement,
                reason: "情绪较低，需要鼓励".to_string(),
                confidence: 0.85,
            });
        }

        // 检查是否在娱乐
        if context.activity.current_state == "entertainment" && context.study.mode == "study" {
            return Ok(BehaviorSuggestion {
                action: SuggestedAction::EnterNormalMode,
                reason: "检测到娱乐活动".to_string(),
                confidence: 0.75,
            });
        }

        // 无建议
        Ok(BehaviorSuggestion {
            action: SuggestedAction::None,
            reason: "无需建议".to_string(),
            confidence: 0.0,
        })
    }

    async fn health_check(&self) -> bool {
        true // 本地规则始终可用
    }
}
