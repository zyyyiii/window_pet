/**
 * 成长系统类型定义
 */

/** 交互类型 */
export type InteractionType =
  | "Feed"
  | "Play"
  | "Talk"
  | "Touch"
  | "Companion"
  | "Learning";

/** 等级信息 */
export interface LevelInfo {
  current_level: number;
  current_exp: number;
  exp_to_next_level: number;
  progress_percent: number;
}

/** 成长系统快照 */
export interface GrowthSnapshot {
  level_info: LevelInfo;
  companion_days: number;
  learning_points: number;
  total_interactions: number;
  memory_count: number;
}

/** 记忆条目 */
export interface Memory {
  id: string;
  content: string;
  importance: number;
  created_at: string;
  last_accessed: string;
  access_count: number;
}

/** 交互记录 */
export interface InteractionRecord {
  interaction_type: InteractionType;
  timestamp: string;
  experience_gained: number;
}

/** AI 记忆系统 */
export interface AiMemory {
  memories: Memory[];
  personality_traits: Record<string, number>;
  interaction_history: InteractionRecord[];
}

/** 完整成长数据 */
export interface GrowthData {
  level: number;
  experience: number;
  companion_days: number;
  learning_points: number;
  ai_memory: AiMemory;
  created_at: string;
  last_updated: string;
}
