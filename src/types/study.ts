/**
 * 学习模式类型
 */
export type StudyMode = 'normal' | 'study' | 'break';

/**
 * 学习会话数据
 */
export interface StudySession {
  id: string;
  mode: StudyMode;
  startTime: number;
  endTime?: number;
  durationSecs: number;
  breakCount: number;
  totalBreakDuration: number;
}

/**
 * 学习统计数据
 */
export interface StudyStats {
  totalStudyTime: number;
  totalBreakTime: number;
  sessionCount: number;
  currentStreak: number;
  longestStreak: number;
}

/**
 * 学习模式快照
 */
export interface StudySnapshot {
  mode: StudyMode;
  currentSession?: StudySession;
  stats: StudyStats;
  lastStudyReminder?: number;
  lastBreakReminder?: number;
  lastWordTest?: number;
}

/**
 * 学习提醒配置
 */
export interface StudyReminderConfig {
  studyReminderInterval: number;
  breakReminderInterval: number;
  wordTestInterval: number;
  enableWordTest: boolean;
}

/**
 * 学习对话类型
 */
export type StudyDialogueType =
  | 'StudyReminder'
  | 'BreakReminder'
  | 'WordTest'
  | 'Encouragement'
  | 'Progress';

/**
 * 格式化时长（秒）为可读字符串
 */
export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

/**
 * 获取模式显示名称
 */
export function getModeDisplayName(mode: StudyMode): string {
  switch (mode) {
    case 'study':
      return '学习模式';
    case 'break':
      return '休息模式';
    case 'normal':
    default:
      return '普通模式';
  }
}

/**
 * 获取模式图标
 */
export function getModeIcon(mode: StudyMode): string {
  switch (mode) {
    case 'study':
      return '📚';
    case 'break':
      return '☕';
    case 'normal':
    default:
      return '😺';
  }
}
