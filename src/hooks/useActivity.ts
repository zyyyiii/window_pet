import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';

/**
 * 活动状态类型
 */
export type ActivityState = 'studying' | 'coding' | 'entertainment' | 'idle' | 'unknown';

/**
 * 状态分数
 */
export interface StateScores {
  studying: number;
  coding: number;
  entertainment: number;
  idle: number;
}

/**
 * 活动快照
 */
export interface ActivitySnapshot {
  windowTitle: string;
  processName: string;
  idleSeconds: number;
  timestamp: number;
}

/**
 * 活动分析结果
 */
export interface ActivityAnalysis {
  state: ActivityState;
  scores: StateScores;
  snapshot: ActivitySnapshot;
}

/**
 * 评分规则
 */
export interface ScoringRule {
  id: string;
  name: string;
  target: 'ProcessName' | 'WindowTitle';
  matchType: 'Exact' | 'Contains' | 'StartsWith' | 'EndsWith';
  pattern: string;
  scores: StateScores;
  enabled: boolean;
}

/**
 * 获取状态显示名称
 */
export function getActivityStateName(state: ActivityState): string {
  switch (state) {
    case 'studying':
      return '学习中';
    case 'coding':
      return '编程中';
    case 'entertainment':
      return '娱乐中';
    case 'idle':
      return '空闲';
    case 'unknown':
    default:
      return '未知';
  }
}

/**
 * 获取状态图标
 */
export function getActivityStateIcon(state: ActivityState): string {
  switch (state) {
    case 'studying':
      return '📚';
    case 'coding':
      return '💻';
    case 'entertainment':
      return '🎮';
    case 'idle':
      return '😴';
    case 'unknown':
    default:
      return '❓';
  }
}

/**
 * 活动检测 Hook
 */
export const useActivity = (pollingInterval: number = 10000) => {
  const [analysis, setAnalysis] = useState<ActivityAnalysis | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [enabled, setEnabled] = useState(true);
  const intervalRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  /**
   * 获取活动分析
   */
  const fetchAnalysis = useCallback(async () => {
    if (!enabled) return;

    try {
      const data = await invoke('get_activity_analysis') as ActivityAnalysis;
      setAnalysis(data);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, [enabled]);

  /**
   * 获取最后一次分析（不触发新检测）
   */
  const getLastAnalysis = useCallback(async () => {
    try {
      const data = await invoke('get_last_activity') as ActivityAnalysis | null;
      return data;
    } catch (err) {
      console.error('获取活动分析失败:', err);
      return null;
    }
  }, []);

  /**
   * 启用/禁用活动检测
   */
  const setDetectionEnabled = useCallback(async (isEnabled: boolean) => {
    try {
      await invoke('set_activity_detection_enabled', { enabled: isEnabled });
      setEnabled(isEnabled);
      setError(null);
    } catch (err) {
      setError(String(err));
    }
  }, []);

  /**
   * 获取所有规则
   */
  const getRules = useCallback(async () => {
    try {
      const rules = await invoke('get_activity_rules') as ScoringRule[];
      return rules;
    } catch (err) {
      console.error('获取规则失败:', err);
      return [];
    }
  }, []);

  /**
   * 添加规则
   */
  const addRule = useCallback(async (rule: ScoringRule) => {
    try {
      const result = await invoke('add_activity_rule', { rule }) as boolean;
      return result;
    } catch (err) {
      console.error('添加规则失败:', err);
      return false;
    }
  }, []);

  /**
   * 删除规则
   */
  const removeRule = useCallback(async (ruleId: string) => {
    try {
      const result = await invoke('remove_activity_rule', { ruleId }) as boolean;
      return result;
    } catch (err) {
      console.error('删除规则失败:', err);
      return false;
    }
  }, []);

  // 轮询活动状态
  useEffect(() => {
    if (enabled) {
      fetchAnalysis();

      intervalRef.current = setInterval(() => {
        fetchAnalysis();
      }, pollingInterval);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [fetchAnalysis, pollingInterval, enabled]);

  return {
    analysis,
    loading,
    error,
    enabled,
    setDetectionEnabled,
    getRules,
    addRule,
    removeRule,
    getLastAnalysis,
    refresh: fetchAnalysis,
  };
};
