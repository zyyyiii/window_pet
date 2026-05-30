use super::types::MoodModifierFn;

pub struct MoodModifier {
    pub name: String,
    pub priority: i32,
    pub modifier_fn: MoodModifierFn,
}

impl MoodModifier {
    pub fn new(name: &str, priority: i32, modifier_fn: MoodModifierFn) -> Self {
        Self {
            name: name.to_string(),
            priority,
            modifier_fn,
        }
    }

    pub fn apply(&self, current_value: f64, delta_time: f64) -> f64 {
        (self.modifier_fn)(current_value, delta_time)
    }
}

// Time-based mood decay
pub struct TimeModifier;

impl TimeModifier {
    pub fn create() -> MoodModifier {
        MoodModifier::new(
            "time_decay",
            0,
            Box::new(|current_value: f64, delta_time: f64| {
                let decay_rate = 0.05 * delta_time;
                (current_value - decay_rate).max(0.0)
            }),
        )
    }
}

// Interaction boost
pub struct InteractionModifier;

impl InteractionModifier {
    pub fn create_boost(amount: f64) -> MoodModifier {
        MoodModifier::new(
            "interaction_boost",
            10,
            Box::new(move |current_value: f64, _delta_time: f64| {
                (current_value + amount).min(100.0)
            }),
        )
    }

    pub fn create_decay(amount: f64) -> MoodModifier {
        MoodModifier::new(
            "interaction_decay",
            10,
            Box::new(move |current_value: f64, _delta_time: f64| {
                (current_value - amount).max(0.0)
            }),
        )
    }
}

// Attribute-based modifier
pub struct AttributeModifier;

impl AttributeModifier {
    pub fn create_from_hunger(hunger: f64) -> MoodModifier {
        MoodModifier::new(
            "hunger_effect",
            5,
            Box::new(move |current_value: f64, delta_time: f64| {
                let effect = if hunger > 70.0 {
                    -0.1 * delta_time
                } else if hunger < 30.0 {
                    0.05 * delta_time
                } else {
                    0.0
                };
                (current_value + effect).max(0.0).min(100.0)
            }),
        )
    }

    pub fn create_from_energy(energy: f64) -> MoodModifier {
        MoodModifier::new(
            "energy_effect",
            5,
            Box::new(move |current_value: f64, delta_time: f64| {
                let effect = if energy < 30.0 {
                    -0.15 * delta_time
                } else if energy > 70.0 {
                    0.03 * delta_time
                } else {
                    0.0
                };
                (current_value + effect).max(0.0).min(100.0)
            }),
        )
    }
}

// State-based modifier
pub struct StateModifier;

impl StateModifier {
    pub fn create_for_state(state: &str) -> MoodModifier {
        let effect_fn: MoodModifierFn = match state {
            "happy" => Box::new(|current: f64, dt: f64| {
                (current + 0.1 * dt).min(100.0)
            }),
            "hungry" => Box::new(|current: f64, dt: f64| {
                (current - 0.2 * dt).max(0.0)
            }),
            "sleepy" => Box::new(|current: f64, dt: f64| {
                (current - 0.15 * dt).max(0.0)
            }),
            "playing" => Box::new(|current: f64, dt: f64| {
                (current + 0.2 * dt).min(100.0)
            }),
            _ => Box::new(|current: f64, _dt: f64| current),
        };

        MoodModifier::new("state_effect", 8, effect_fn)
    }
}

// Modifier Manager
pub struct ModifierManager {
    modifiers: Vec<MoodModifier>,
}

impl ModifierManager {
    pub fn new() -> Self {
        Self {
            modifiers: vec![
                TimeModifier::create(),
            ],
        }
    }

    pub fn add_modifier(&mut self, modifier: MoodModifier) {
        self.modifiers.push(modifier);
        self.modifiers.sort_by_key(|m| m.priority);
    }

    pub fn remove_modifier(&mut self, name: &str) {
        self.modifiers.retain(|m| m.name != name);
    }

    pub fn apply_all(&self, current_value: f64, delta_time: f64) -> f64 {
        let mut value = current_value;
        for modifier in &self.modifiers {
            value = modifier.apply(value, delta_time);
        }
        value.max(0.0).min(100.0)
    }

    pub fn clear(&mut self) {
        self.modifiers.clear();
        self.modifiers.push(TimeModifier::create());
    }
}

impl Default for ModifierManager {
    fn default() -> Self {
        Self::new()
    }
}