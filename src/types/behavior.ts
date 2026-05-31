/**
 * 行为提供者类型
 */
export type BehaviorProviderType = 'rule' | 'ai';

/**
 * AI 提供者标识
 */
export type AIProvider = 'deepseek' | 'gemini' | 'claude' | 'openai' | string;

/**
 * 对话触发类型
 */
export type DialogueTrigger =
  | 'touch'
  | 'double_touch'
  | 'bubble'
  | 'study_reminder'
  | 'break_reminder'
  | 'user_input'
  | 'scheduled';

/**
 * 对话类型
 */
export type DialogueType =
  | 'chat'
  | 'study'
  | 'encouragement'
  | 'reminder'
  | 'progress'
  | 'knowledge';

/**
 * 对话请求
 */
export interface DialogueRequest {
  user_input?: string;
  trigger: DialogueTrigger;
  preferred_type?: DialogueType;
}

/**
 * 对话响应
 */
export interface DialogueResponse {
  text: string;
  emotion?: string;
  animation_hint?: string;
  dialogue_type: DialogueType;
  follow_up: boolean;
}

/**
 * 提醒类型
 */
export type ReminderType =
  | 'study'
  | 'break'
  | 'drink_water'
  | 'progress_check'
  | { custom: string };

/**
 * 提醒优先级
 */
export type ReminderPriority = 'low' | 'medium' | 'high';

/**
 * 提醒响应
 */
export interface ReminderResponse {
  text: string;
  priority: ReminderPriority;
  require_ack: boolean;
}

/**
 * 建议的行为
 */
export type SuggestedAction =
  | 'enter_study_mode'
  | 'enter_break_mode'
  | 'enter_normal_mode'
  | 'show_encouragement'
  | { show_reminder: string }
  | 'none';

/**
 * 行为建议
 */
export interface BehaviorSuggestion {
  action: SuggestedAction;
  reason: string;
  confidence: number;
}

/**
 * 行为管理器配置
 */
export interface BehaviorManagerConfig {
  active_provider: string;
  confidence_threshold: number;
  enable_suggestions: boolean;
  max_dialogue_history: number;
}

/**
 * AI 提供者配置
 */
export interface AIProviderConfig {
  api_endpoint: string;
  api_key?: string;
  model: string;
  max_tokens: number;
  temperature: number;
  timeout_secs: number;
}

/**
 * 行为上下文（用于调试）
 */
export interface BehaviorContext {
  pet: {
    name: string;
    state: string;
    hunger: number;
    mood_value: number;
    energy: number;
    cleanliness: number;
    health: number;
  };
  activity: {
    current_state: string;
    window_title: string;
    process_name: string;
    idle_seconds: number;
  };
  study: {
    mode: string;
    study_duration_secs: number;
    break_duration_secs: number;
    session_count: number;
    total_study_time: number;
  };
  mood: {
    state: string;
    value: number;
  };
  dialogue_history: Array<{
    role: 'pet' | 'user' | 'system';
    content: string;
    timestamp: number;
    dialogue_type?: DialogueType;
  }>;
  time: {
    timestamp: number;
    hour: number;
    weekday: number;
    is_workday: boolean;
  };
}

/**
 * 获取对话触发类型显示名称
 */
export function getTriggerName(trigger: DialogueTrigger): string {
  switch (trigger) {
    case 'touch':
      return '触摸';
    case 'double_touch':
      return '双击';
    case 'bubble':
      return '气泡';
    case 'study_reminder':
      return '学习提醒';
    case 'break_reminder':
      return '休息提醒';
    case 'user_input':
      return '用户输入';
    case 'scheduled':
      return '定时';
    default:
      return '未知';
  }
}

/**
 * 获取对话类型显示名称
 */
export function getDialogueTypeName(type: DialogueType): string {
  switch (type) {
    case 'chat':
      return '闲聊';
    case 'study':
      return '学习';
    case 'encouragement':
      return '鼓励';
    case 'reminder':
      return '提醒';
    case 'progress':
      return '进度';
    case 'knowledge':
      return '知识';
    default:
      return '未知';
  }
}
