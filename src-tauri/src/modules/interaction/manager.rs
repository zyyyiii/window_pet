use std::sync::{Arc, Mutex};
use super::types::{InteractionType, InteractionResponse, MenuItem};
use super::dialogue::DialogueManager;
use super::bubble::BubbleManager;

pub struct InteractionManager {
    dialogue_manager: Arc<Mutex<DialogueManager>>,
    bubble_manager: Arc<Mutex<BubbleManager>>,
    current_state: String,
}

impl InteractionManager {
    pub fn new() -> Self {
        Self {
            dialogue_manager: Arc::new(Mutex::new(DialogueManager::new())),
            bubble_manager: Arc::new(Mutex::new(BubbleManager::new(10, 30))),
            current_state: "idle".to_string(),
        }
    }

    pub fn handle_interaction(&mut self, interaction: InteractionType) -> InteractionResponse {
        match interaction {
            InteractionType::Touch { x, y } => self.handle_touch(x, y),
            InteractionType::DoubleTouch { x, y } => self.handle_double_touch(x, y),
            InteractionType::RightClick { x, y } => self.handle_right_click(x, y),
            InteractionType::BubbleTrigger => self.handle_bubble_trigger(),
        }
    }

    fn handle_touch(&self, _x: f64, _y: f64) -> InteractionResponse {
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        let response_text = dialogue_manager.get_touch_response();
        
        InteractionResponse {
            interaction_type: "touch".to_string(),
            animation: Some("happy".to_string()),
            dialogue: None,
            bubble: Some(super::types::BubbleMessage {
                id: "touch_response".to_string(),
                text: response_text,
                duration: 2000,
                emotion: Some("happy".to_string()),
                trigger_type: "interaction".to_string(),
            }),
            menu_items: None,
            emotion: Some("happy".to_string()),
        }
    }

    fn handle_double_touch(&self, _x: f64, _y: f64) -> InteractionResponse {
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        let dialogue_tree = dialogue_manager.get_default_dialogue().cloned();
        
        InteractionResponse {
            interaction_type: "double_touch".to_string(),
            animation: Some("talking".to_string()),
            dialogue: dialogue_tree,
            bubble: None,
            menu_items: None,
            emotion: Some("curious".to_string()),
        }
    }

    fn handle_right_click(&self, _x: f64, _y: f64) -> InteractionResponse {
        let menu_items = vec![
            MenuItem {
                id: "feed".to_string(),
                label: "喂食".to_string(),
                icon: Some("🍕".to_string()),
                action: "feed".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "play".to_string(),
                label: "玩耍".to_string(),
                icon: Some("🎮".to_string()),
                action: "play".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "talk".to_string(),
                label: "对话".to_string(),
                icon: Some("💬".to_string()),
                action: "talk".to_string(),
                disabled: false,
            },
            // 学习模式菜单项
            MenuItem {
                id: "study_mode".to_string(),
                label: "进入学习模式".to_string(),
                icon: Some("📚".to_string()),
                action: "study_mode".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "break_mode".to_string(),
                label: "进入休息模式".to_string(),
                icon: Some("☕".to_string()),
                action: "break_mode".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "normal_mode".to_string(),
                label: "恢复普通模式".to_string(),
                icon: Some("😺".to_string()),
                action: "normal_mode".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "settings".to_string(),
                label: "设置".to_string(),
                icon: Some("⚙️".to_string()),
                action: "settings".to_string(),
                disabled: false,
            },
            MenuItem {
                id: "exit".to_string(),
                label: "退出".to_string(),
                icon: Some("❌".to_string()),
                action: "exit".to_string(),
                disabled: false,
            },
        ];

        InteractionResponse {
            interaction_type: "right_click".to_string(),
            animation: None,
            dialogue: None,
            bubble: None,
            menu_items: Some(menu_items),
            emotion: None,
        }
    }

    fn handle_bubble_trigger(&self) -> InteractionResponse {
        let mut bubble_manager = self.bubble_manager.lock().unwrap();
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        
        let bubble = bubble_manager.update(&dialogue_manager);
        
        InteractionResponse {
            interaction_type: "bubble".to_string(),
            animation: None,
            dialogue: None,
            bubble,
            menu_items: None,
            emotion: None,
        }
    }

    pub fn set_state(&mut self, state: String) {
        self.current_state = state;
    }

    pub fn get_state_response(&self) -> String {
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        dialogue_manager.get_state_response(&self.current_state)
    }

    pub fn force_bubble(&self) -> Option<super::types::BubbleMessage> {
        let mut bubble_manager = self.bubble_manager.lock().unwrap();
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        bubble_manager.force_trigger(&dialogue_manager)
    }

    pub fn update_bubble(&self) -> Option<super::types::BubbleMessage> {
        let mut bubble_manager = self.bubble_manager.lock().unwrap();
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        bubble_manager.update(&dialogue_manager)
    }

    pub fn hide_bubble(&self) {
        let mut bubble_manager = self.bubble_manager.lock().unwrap();
        bubble_manager.hide();
    }

    pub fn get_dialogue_node(&self, tree_id: &str, node_id: &str) -> Option<super::types::DialogueNode> {
        let dialogue_manager = self.dialogue_manager.lock().unwrap();
        dialogue_manager.get_dialogue_node(tree_id, node_id).cloned()
    }
}