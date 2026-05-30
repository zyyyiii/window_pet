export type MoodState = "happy" | "normal" | "bored" | "sleepy";

export interface MoodSnapshot {
  value: number;
  state: MoodState;
  state_str: string;
  emoji: string;
  animation_hint: string;
}

export interface MoodConfig {
  initial_value: number;
  decay_rate: number;
  interaction_boost: number;
  state_thresholds: MoodThresholds;
}

export interface MoodThresholds {
  happy: number;
  normal: number;
  bored: number;
  sleepy: number;
}

export const DEFAULT_MOOD_CONFIG: MoodConfig = {
  initial_value: 70,
  decay_rate: 0.05,
  interaction_boost: 10,
  state_thresholds: {
    happy: 75,
    normal: 50,
    bored: 25,
    sleepy: 0,
  },
};

export const MOOD_STATE_INFO: Record<MoodState, { emoji: string; label: string; color: string }> = {
  happy: { emoji: "😺", label: "开心", color: "#4CAF50" },
  normal: { emoji: "🐱", label: "普通", color: "#2196F3" },
  bored: { emoji: "😿", label: "无聊", color: "#FF9800" },
  sleepy: { emoji: "😴", label: "困倦", color: "#9C27B0" },
};

export function getMoodState(value: number): MoodState {
  if (value >= 75) return "happy";
  if (value >= 50) return "normal";
  if (value >= 25) return "bored";
  return "sleepy";
}

export function getMoodInfo(state: MoodState) {
  return MOOD_STATE_INFO[state];
}