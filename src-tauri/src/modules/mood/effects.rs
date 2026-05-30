use super::types::MoodState;

pub trait MoodEffect: Send + Sync {
    fn name(&self) -> &str;
    fn apply(&self, mood_state: MoodState, value: f64) -> MoodEffectResult;
}

#[derive(Debug, Clone)]
pub struct MoodEffectResult {
    pub animation: Option<String>,
    pub dialogue_modifier: Option<DialogueModifier>,
    pub action_modifiers: Vec<ActionModifier>,
}

#[derive(Debug, Clone)]
pub struct DialogueModifier {
    pub response_style: String,
    pub bubble_frequency: f64,
    pub touch_responses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ActionModifier {
    pub action: String,
    pub probability: f64,
    pub cooldown_modifier: f64,
}

// Animation Effect
pub struct AnimationEffect;

impl MoodEffect for AnimationEffect {
    fn name(&self) -> &str {
        "animation"
    }

    fn apply(&self, mood_state: MoodState, _value: f64) -> MoodEffectResult {
        let animation = match mood_state {
            MoodState::Happy => Some("happy".to_string()),
            MoodState::Normal => Some("idle".to_string()),
            MoodState::Bored => Some("bored".to_string()),
            MoodState::Sleepy => Some("sleepy".to_string()),
        };

        MoodEffectResult {
            animation,
            dialogue_modifier: None,
            action_modifiers: vec![],
        }
    }
}

// Dialogue Effect
pub struct DialogueEffect;

impl MoodEffect for DialogueEffect {
    fn name(&self) -> &str {
        "dialogue"
    }

    fn apply(&self, mood_state: MoodState, value: f64) -> MoodEffectResult {
        let modifier = match mood_state {
            MoodState::Happy => DialogueModifier {
                response_style: "cheerful".to_string(),
                bubble_frequency: 1.2,
                touch_responses: vec![
                    "好开心呀！".to_string(),
                    "喵哈哈哈~".to_string(),
                    "今天心情真好！".to_string(),
                    "最喜欢你了！".to_string(),
                ],
            },
            MoodState::Normal => DialogueModifier {
                response_style: "neutral".to_string(),
                bubble_frequency: 1.0,
                touch_responses: vec![
                    "喵~".to_string(),
                    "摸摸头~".to_string(),
                    "好舒服~".to_string(),
                ],
            },
            MoodState::Bored => DialogueModifier {
                response_style: "bored".to_string(),
                bubble_frequency: 0.8,
                touch_responses: vec![
                    "无聊...".to_string(),
                    "唉...".to_string(),
                    "陪我玩嘛~".to_string(),
                ],
            },
            MoodState::Sleepy => DialogueModifier {
                response_style: "sleepy".to_string(),
                bubble_frequency: 0.5,
                touch_responses: vec![
                    "好困...".to_string(),
                    "打哈欠~".to_string(),
                    "想睡觉了...".to_string(),
                ],
            },
        };

        MoodEffectResult {
            animation: None,
            dialogue_modifier: Some(modifier),
            action_modifiers: vec![],
        }
    }
}

// Action Effect
pub struct ActionEffect;

impl MoodEffect for ActionEffect {
    fn name(&self) -> &str {
        "action"
    }

    fn apply(&self, mood_state: MoodState, value: f64) -> MoodEffectResult {
        let mut modifiers = vec![];

        match mood_state {
            MoodState::Happy => {
                modifiers.push(ActionModifier {
                    action: "play".to_string(),
                    probability: 1.2,
                    cooldown_modifier: 0.8,
                });
                modifiers.push(ActionModifier {
                    action: "talk".to_string(),
                    probability: 1.1,
                    cooldown_modifier: 0.9,
                });
            }
            MoodState::Normal => {
                // No modifiers for normal state
            }
            MoodState::Bored => {
                modifiers.push(ActionModifier {
                    action: "play".to_string(),
                    probability: 1.3,
                    cooldown_modifier: 0.7,
                });
                modifiers.push(ActionModifier {
                    action: "feed".to_string(),
                    probability: 0.9,
                    cooldown_modifier: 1.1,
                });
            }
            MoodState::Sleepy => {
                modifiers.push(ActionModifier {
                    action: "rest".to_string(),
                    probability: 1.5,
                    cooldown_modifier: 0.5,
                });
                modifiers.push(ActionModifier {
                    action: "play".to_string(),
                    probability: 0.5,
                    cooldown_modifier: 1.5,
                });
            }
        }

        MoodEffectResult {
            animation: None,
            dialogue_modifier: None,
            action_modifiers: modifiers,
        }
    }
}

// Combined Effect
pub struct CombinedEffect {
    effects: Vec<Box<dyn MoodEffect>>,
}

impl CombinedEffect {
    pub fn new() -> Self {
        Self {
            effects: vec![
                Box::new(AnimationEffect),
                Box::new(DialogueEffect),
                Box::new(ActionEffect),
            ],
        }
    }

    pub fn apply_all(&self, mood_state: MoodState, value: f64) -> Vec<MoodEffectResult> {
        self.effects
            .iter()
            .map(|effect| effect.apply(mood_state, value))
            .collect()
    }
}

impl Default for CombinedEffect {
    fn default() -> Self {
        Self::new()
    }
}