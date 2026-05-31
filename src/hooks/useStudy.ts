import { useState, useEffect, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  StudyMode,
  StudySnapshot,
  StudyReminderConfig,
  StudyDialogueType,
} from '../types/study';

/**
 * 学习系统 Hook
 */
export const useStudy = (pollingInterval: number = 5000) => {
  const [snapshot, setSnapshot] = useState<StudySnapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const intervalRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  /**
   * 获取学习模式快照
   */
  const fetchSnapshot = useCallback(async () => {
    try {
      const data = await invoke('get_study_snapshot') as StudySnapshot;
      setSnapshot(data);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * 设置学习模式
   */
  const setMode = useCallback(async (mode: StudyMode) => {
    try {
      const data = await invoke('set_study_mode', { mode }) as StudySnapshot;
      setSnapshot(data);
      setError(null);
      return data;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  /**
   * 检查是否需要提醒
   */
  const checkReminder = useCallback(async () => {
    try {
      const reminder = await invoke('check_study_reminder') as string | null;
      return reminder as StudyDialogueType | null;
    } catch (err) {
      console.error('检查提醒失败:', err);
      return null;
    }
  }, []);

  /**
   * 获取学习对话
   */
  const getDialogue = useCallback(async () => {
    try {
      const dialogue = await invoke('get_study_dialogue') as string | null;
      return dialogue;
    } catch (err) {
      console.error('获取对话失败:', err);
      return null;
    }
  }, []);

  /**
   * 获取学习时长
   */
  const getStudyDuration = useCallback(async () => {
    try {
      const duration = await invoke('get_study_duration') as number;
      return duration;
    } catch (err) {
      console.error('获取学习时长失败:', err);
      return 0;
    }
  }, []);

  /**
   * 获取休息时长
   */
  const getBreakDuration = useCallback(async () => {
    try {
      const duration = await invoke('get_break_duration') as number;
      return duration;
    } catch (err) {
      console.error('获取休息时长失败:', err);
      return 0;
    }
  }, []);

  /**
   * 更新计时器
   */
  const updateTimer = useCallback(async () => {
    try {
      await invoke('update_study_timer');
    } catch (err) {
      console.error('更新计时器失败:', err);
    }
  }, []);

  /**
   * 设置提醒配置
   */
  const setReminderConfig = useCallback(async (config: StudyReminderConfig) => {
    try {
      await invoke('set_study_reminder_config', {
        studyReminderInterval: config.studyReminderInterval,
        breakReminderInterval: config.breakReminderInterval,
        wordTestInterval: config.wordTestInterval,
        enableWordTest: config.enableWordTest,
      });
      setError(null);
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  /**
   * 获取提醒配置
   */
  const getReminderConfig = useCallback(async () => {
    try {
      const config = await invoke('get_study_reminder_config') as StudyReminderConfig;
      return config;
    } catch (err) {
      console.error('获取提醒配置失败:', err);
      return null;
    }
  }, []);

  // 轮询学习状态
  useEffect(() => {
    fetchSnapshot();

    intervalRef.current = setInterval(() => {
      fetchSnapshot();
      updateTimer();
    }, pollingInterval);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [fetchSnapshot, updateTimer, pollingInterval]);

  return {
    snapshot,
    loading,
    error,
    setMode,
    checkReminder,
    getDialogue,
    getStudyDuration,
    getBreakDuration,
    updateTimer,
    setReminderConfig,
    getReminderConfig,
    refresh: fetchSnapshot,
  };
};
