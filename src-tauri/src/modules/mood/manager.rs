use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use super::types::{MoodState, MoodRange, MoodSnapshot, MoodModifierFn};
use super::effects::{CombinedEffect, MoodEffectResult, DialogueModifier, ActionModifier};
use super::modifiers::{ModifierManager, MoodModifier, InteractionModifier, AttributeModifier, StateModifier};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodConfig {
    pub initial_value: f64,
    pub decay_rate: f64,
    pub interaction_boost: f64,
    pub state_thresholds: MoodThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodThresholds {
    pub happy: f64,
    pub normal: f64,
    pub bored: f64,
    pub sleepy: f64,
}

impl Default for MoodThresholds {
    fn default() -> Self {
        Self {
            happy: 75.0,
            normal: 50.0,
            bored: 25.0,
            sleepy: 0.0,
        }
    }
}

impl Default for MoodConfig {
    fn default() -> Self {
        Self {
            initial_value: 70.0,
            decay_rate: 0.05,
            interaction_boost: 10.0,
            state_thresholds: MoodThresholds::default(),
        }
    }
}

pub struct MoodManager {
    value: f64,
    state: MoodState,
    range: MoodRange,
    config: MoodConfig,
    effects: CombinedEffect,
    modifiers: ModifierManager,
    listeners: Vec<Box<dyn Fn(&MoodSnapshot) + Send + Sync>>,
}

impl MoodManager {
    pub fn new(config: MoodConfig) -> Self {
        let state = MoodState::from_range(config.initial_value);
        Self {
            value: config.initial_value,
            state,
            range: MoodRange::default(),
            config,
            effects: CombinedEffect::new(),
            modifiers: ModifierManager::new(),
            listeners: vec![],
        }
    }

    pub fn with_default() -> Self {
        Self::new(MoodConfig::default())
    }

    pub fn update(&mut self, delta_time: f64) {
        self.value = self.modifiers.apply_all(self.value, delta_time);
        self.value = self.range.clamp(self.value);
        
        let new_state = MoodState::from_range(self.value);
        if new_state != self.state {
            self.state = new_state;
            self.notify_listeners();
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_state(&self) -> MoodState {
        self.state
    }

    pub fn get_state_str(&self) -> &str {
        self.state.as_str()
    }

    pub fn get_snapshot(&self) -> MoodSnapshot {
        MoodSnapshot::from_manager(self.value, self.state)
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = self.range.clamp(value);
        let new_state = MoodState::from_range(self.value);
        if new_state != self.state {
            self.state = new_state;
            self.notify_listeners();
        }
    }

    pub fn add_modifier(&mut self, modifier: MoodModifier) {
        self.modifiers.add_modifier(modifier);
    }

    pub fn remove_modifier(&mut self, name: &str) {
        self.modifiers.remove_modifier(name);
    }

    pub fn apply_interaction_boost(&mut self, amount: Option<f64>) {
        let boost = amount.unwrap_or(self.config.interaction_boost);
        self.add_modifier(InteractionModifier::create_boost(boost));
    }

    pub fn apply_interaction_decay(&mut self, amount: Option<f64>) {
        let decay = amount.unwrap_or(5.0);
        self.add_modifier(InteractionModifier::create_decay(decay));
    }

    pub fn update_attributes(&mut self, hunger: f64, energy: f64) {
        self.remove_modifier("hunger_effect");
        self.remove_modifier("energy_effect");
        self.add_modifier(AttributeModifier::create_from_hunger(hunger));
        self.add_modifier(AttributeModifier::create_from_energy(energy));
    }

    pub fn set_state_modifier(&mut self, state: &str) {
        self.remove_modifier("state_effect");
        self.add_modifier(StateModifier::create_for_state(state));
    }

    pub fn get_effects(&self) -> Vec<MoodEffectResult> {
        self.effects.apply_all(self.state, self.value)
    }

    pub fn get_dialogue_modifier(&self) -> Option<DialogueModifier> {
        self.effects.apply_all(self.state, self.value)
            .into_iter()
            .find_map(|r| r.dialogue_modifier)
    }

    pub fn get_action_modifiers(&self) -> Vec<ActionModifier> {
        self.effects.apply_all(self.state, self.value)
            .into_iter()
            .flat_map(|r| r.action_modifiers)
            .collect()
    }

    pub fn get_animation_hint(&self) -> String {
        self.state.animation_hint().to_string()
    }

    pub fn get_emoji(&self) -> String {
        self.state.emoji().to_string()
    }

    pub fn on_change(&mut self, listener: Box<dyn Fn(&MoodSnapshot) + Send + Sync>) {
        self.listeners.push(listener);
    }

    fn notify_listeners(&self) {
        let snapshot = self.get_snapshot();
        for listener in &self.listeners {
            listener(&snapshot);
        }
    }

    pub fn reset(&mut self) {
        self.value = self.config.initial_value;
        self.state = MoodState::from_range(self.value);
        self.modifiers.clear();
        self.notify_listeners();
    }
}

impl Default for MoodManager {
    fn default() -> Self {
        Self::with_default()
    }
}

// Thread-safe wrapper
#[derive(Clone)]
pub struct SharedMoodManager {
    inner: Arc<Mutex<MoodManager>>,
}

impl SharedMoodManager {
    pub fn new(config: MoodConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(MoodManager::new(config))),
        }
    }

    pub fn with_default() -> Self {
        Self::new(MoodConfig::default())
    }

    pub fn update(&self, delta_time: f64) {
        let mut manager = self.inner.lock().unwrap();
        manager.update(delta_time);
    }

    pub fn get_snapshot(&self) -> MoodSnapshot {
        let manager = self.inner.lock().unwrap();
        manager.get_snapshot()
    }

    pub fn set_value(&self, value: f64) {
        let mut manager = self.inner.lock().unwrap();
        manager.set_value(value);
    }

    pub fn apply_interaction_boost(&self, amount: Option<f64>) {
        let mut manager = self.inner.lock().unwrap();
        manager.apply_interaction_boost(amount);
    }

    pub fn get_dialogue_modifier(&self) -> Option<DialogueModifier> {
        let manager = self.inner.lock().unwrap();
        manager.get_dialogue_modifier()
    }

    pub fn get_animation_hint(&self) -> String {
        let manager = self.inner.lock().unwrap();
        manager.get_animation_hint()
    }

    pub fn get_emoji(&self) -> String {
        let manager = self.inner.lock().unwrap();
        manager.get_emoji()
    }
}