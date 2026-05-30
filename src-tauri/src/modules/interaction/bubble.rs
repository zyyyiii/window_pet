use std::time::{Duration, Instant};
use rand::Rng;
use super::types::BubbleMessage;
use super::dialogue::DialogueManager;

pub struct BubbleManager {
    last_trigger: Instant,
    min_interval: Duration,
    max_interval: Duration,
    next_interval: Duration,
    is_visible: bool,
    current_message: Option<BubbleMessage>,
}

impl BubbleManager {
    pub fn new(min_interval_secs: u64, max_interval_secs: u64) -> Self {
        let mut manager = Self {
            last_trigger: Instant::now(),
            min_interval: Duration::from_secs(min_interval_secs),
            max_interval: Duration::from_secs(max_interval_secs),
            next_interval: Duration::from_secs(min_interval_secs),
            is_visible: false,
            current_message: None,
        };
        manager.randomize_interval();
        manager
    }

    fn randomize_interval(&mut self) {
        let mut rng = rand::thread_rng();
        let secs = rng.gen_range(self.min_interval.as_secs()..=self.max_interval.as_secs());
        self.next_interval = Duration::from_secs(secs);
    }

    pub fn update(&mut self, dialogue_manager: &DialogueManager) -> Option<BubbleMessage> {
        if self.is_visible {
            if let Some(msg) = &self.current_message {
                if self.last_trigger.elapsed() >= Duration::from_millis(msg.duration) {
                    self.hide();
                }
            }
            return None;
        }

        if self.last_trigger.elapsed() >= self.next_interval {
            if let Some(message) = dialogue_manager.get_random_bubble() {
                self.show(message.clone());
                self.last_trigger = Instant::now();
                self.randomize_interval();
                return Some(message);
            }
        }

        None
    }

    pub fn show(&mut self, message: BubbleMessage) {
        self.current_message = Some(message);
        self.is_visible = true;
        self.last_trigger = Instant::now();
    }

    pub fn hide(&mut self) {
        self.current_message = None;
        self.is_visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }

    pub fn current_message(&self) -> Option<&BubbleMessage> {
        self.current_message.as_ref()
    }

    pub fn force_trigger(&mut self, dialogue_manager: &DialogueManager) -> Option<BubbleMessage> {
        if let Some(message) = dialogue_manager.get_random_bubble() {
            self.show(message.clone());
            self.last_trigger = Instant::now();
            self.randomize_interval();
            return Some(message);
        }
        None
    }
}