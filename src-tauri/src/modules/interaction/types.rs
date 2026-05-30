use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Touch { x: f64, y: f64 },
    DoubleTouch { x: f64, y: f64 },
    RightClick { x: f64, y: f64 },
    BubbleTrigger,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub id: String,
    pub text: String,
    pub next_node: Option<String>,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
    pub auto_next: Option<String>,
    pub emotion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub id: String,
    pub start_node: String,
    pub nodes: std::collections::HashMap<String, DialogueNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BubbleMessage {
    pub id: String,
    pub text: String,
    pub duration: u64,
    pub emotion: Option<String>,
    pub trigger_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub action: String,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResponse {
    pub interaction_type: String,
    pub animation: Option<String>,
    pub dialogue: Option<DialogueTree>,
    pub bubble: Option<BubbleMessage>,
    pub menu_items: Option<Vec<MenuItem>>,
    pub emotion: Option<String>,
}