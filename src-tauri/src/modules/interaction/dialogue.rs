use std::collections::HashMap;
use rand::Rng;
use super::types::{DialogueTree, DialogueNode, DialogueOption, BubbleMessage};

pub struct DialogueManager {
    dialogue_trees: HashMap<String, DialogueTree>,
    bubble_messages: Vec<BubbleMessage>,
    touch_responses: Vec<String>,
    state_responses: HashMap<String, Vec<String>>,
}

impl DialogueManager {
    pub fn new() -> Self {
        let mut manager = Self {
            dialogue_trees: HashMap::new(),
            bubble_messages: Vec::new(),
            touch_responses: Vec::new(),
            state_responses: HashMap::new(),
        };
        manager.load_default_content();
        manager
    }

    fn load_default_content(&mut self) {
        // Touch responses
        self.touch_responses = vec![
            "喵~".to_string(),
            "摸摸头~".to_string(),
            "好舒服~".to_string(),
            "再摸摸~".to_string(),
            "呼噜呼噜~".to_string(),
            "蹭蹭~".to_string(),
        ];

        // State-specific responses
        self.state_responses.insert("happy".to_string(), vec![
            "今天心情真好！".to_string(),
            "喵哈哈哈~".to_string(),
            "好开心呀！".to_string(),
        ]);

        self.state_responses.insert("hungry".to_string(), vec![
            "肚子好饿...".to_string(),
            "有吃的吗？".to_string(),
            "想吃小鱼干~".to_string(),
        ]);

        self.state_responses.insert("sleepy".to_string(), vec![
            "好困...".to_string(),
            "想睡觉了...".to_string(),
            "打哈欠~".to_string(),
        ]);

        self.state_responses.insert("idle".to_string(), vec![
            "无聊...".to_string(),
            "陪我玩嘛~".to_string(),
            "今天做什么好呢？".to_string(),
        ]);

        // Bubble messages
        self.bubble_messages = vec![
            BubbleMessage {
                id: "random_1".to_string(),
                text: "喵~".to_string(),
                duration: 3000,
                emotion: Some("neutral".to_string()),
                trigger_type: "random".to_string(),
            },
            BubbleMessage {
                id: "random_2".to_string(),
                text: "在想什么呢？".to_string(),
                duration: 3000,
                emotion: Some("curious".to_string()),
                trigger_type: "random".to_string(),
            },
            BubbleMessage {
                id: "random_3".to_string(),
                text: "今天天气真好~".to_string(),
                duration: 3000,
                emotion: Some("happy".to_string()),
                trigger_type: "random".to_string(),
            },
            BubbleMessage {
                id: "random_4".to_string(),
                text: "有点无聊呢".to_string(),
                duration: 3000,
                emotion: Some("neutral".to_string()),
                trigger_type: "random".to_string(),
            },
            BubbleMessage {
                id: "random_5".to_string(),
                text: "想吃零食...".to_string(),
                duration: 3000,
                emotion: Some("hungry".to_string()),
                trigger_type: "random".to_string(),
            },
        ];

        // Default dialogue tree
        let mut nodes = HashMap::new();
        nodes.insert("start".to_string(), DialogueNode {
            id: "start".to_string(),
            speaker: "pet".to_string(),
            text: "喵？有什么事吗？".to_string(),
            options: vec![
                DialogueOption {
                    id: "opt_1".to_string(),
                    text: "你今天怎么样？".to_string(),
                    next_node: Some("how_are_you".to_string()),
                    action: None,
                },
                DialogueOption {
                    id: "opt_2".to_string(),
                    text: "陪我聊天".to_string(),
                    next_node: Some("chat".to_string()),
                    action: None,
                },
                DialogueOption {
                    id: "opt_3".to_string(),
                    text: "没什么".to_string(),
                    next_node: None,
                    action: None,
                },
            ],
            auto_next: None,
            emotion: Some("curious".to_string()),
        });

        nodes.insert("how_are_you".to_string(), DialogueNode {
            id: "how_are_you".to_string(),
            speaker: "pet".to_string(),
            text: "还不错啦~就是有点无聊".to_string(),
            options: vec![
                DialogueOption {
                    id: "opt_1".to_string(),
                    text: "那我们玩吧！".to_string(),
                    next_node: None,
                    action: Some("play".to_string()),
                },
                DialogueOption {
                    id: "opt_2".to_string(),
                    text: "改天再聊".to_string(),
                    next_node: None,
                    action: None,
                },
            ],
            auto_next: None,
            emotion: Some("happy".to_string()),
        });

        nodes.insert("chat".to_string(), DialogueNode {
            id: "chat".to_string(),
            speaker: "pet".to_string(),
            text: "好呀！你想聊什么？".to_string(),
            options: vec![
                DialogueOption {
                    id: "opt_1".to_string(),
                    text: "讲个笑话".to_string(),
                    next_node: Some("joke".to_string()),
                    action: None,
                },
                DialogueOption {
                    id: "opt_2".to_string(),
                    text: "你喜欢什么？".to_string(),
                    next_node: Some("likes".to_string()),
                    action: None,
                },
                DialogueOption {
                    id: "opt_3".to_string(),
                    text: "算了".to_string(),
                    next_node: None,
                    action: None,
                },
            ],
            auto_next: None,
            emotion: Some("happy".to_string()),
        });

        nodes.insert("joke".to_string(), DialogueNode {
            id: "joke".to_string(),
            speaker: "pet".to_string(),
            text: "为什么程序员总是分不清万圣节和圣诞节？因为 Oct 31 == Dec 25！".to_string(),
            options: vec![
                DialogueOption {
                    id: "opt_1".to_string(),
                    text: "哈哈，好好笑".to_string(),
                    next_node: None,
                    action: None,
                },
            ],
            auto_next: None,
            emotion: Some("excited".to_string()),
        });

        nodes.insert("likes".to_string(), DialogueNode {
            id: "likes".to_string(),
            speaker: "pet".to_string(),
            text: "我喜欢小鱼干、晒太阳、还有和你一起玩！".to_string(),
            options: vec![
                DialogueOption {
                    id: "opt_1".to_string(),
                    text: "我也喜欢你".to_string(),
                    next_node: None,
                    action: None,
                },
            ],
            auto_next: None,
            emotion: Some("happy".to_string()),
        });

        self.dialogue_trees.insert("default".to_string(), DialogueTree {
            id: "default".to_string(),
            start_node: "start".to_string(),
            nodes,
        });
    }

    pub fn get_touch_response(&self) -> String {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.touch_responses.len());
        self.touch_responses[index].clone()
    }

    pub fn get_state_response(&self, state: &str) -> String {
        let responses = self.state_responses.get(state)
            .unwrap_or(&self.touch_responses);
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..responses.len());
        responses[index].clone()
    }

    pub fn get_random_bubble(&self) -> Option<BubbleMessage> {
        if self.bubble_messages.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.bubble_messages.len());
        Some(self.bubble_messages[index].clone())
    }

    pub fn get_dialogue_tree(&self, id: &str) -> Option<&DialogueTree> {
        self.dialogue_trees.get(id)
    }

    pub fn get_default_dialogue(&self) -> Option<&DialogueTree> {
        self.dialogue_trees.get("default")
    }

    pub fn get_dialogue_node(&self, tree_id: &str, node_id: &str) -> Option<&DialogueNode> {
        self.dialogue_trees.get(tree_id)
            .and_then(|tree| tree.nodes.get(node_id))
    }
}