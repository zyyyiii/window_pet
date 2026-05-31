import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import {
  DialogueRequest,
  DialogueResponse,
  ReminderType,
  ReminderResponse,
  BehaviorSuggestion,
  BehaviorManagerConfig,
  AIProvider,
  AIProviderConfig,
} from '../types/behavior';

/**
 * 行为系统 Hook
 */
export const useBehavior = () => {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * 生成对话
   */
  const generateDialogue = useCallback(
    async (
      request: DialogueRequest,
      context: {
        petName: string;
        petState: string;
        petHunger: number;
        petMoodValue: number;
        petEnergy: number;
        petCleanliness: number;
        petHealth: number;
        activityState: string;
        windowTitle: string;
        processName: string;
        idleSeconds: number;
        studyMode: string;
        studyDurationSecs: number;
        breakDurationSecs: number;
        sessionCount: number;
        totalStudyTime: number;
        moodState: string;
        moodValue: number;
      }
    ): Promise<DialogueResponse | null> => {
      setLoading(true);
      setError(null);
      try {
        const response = await invoke('generate_behavior_dialogue', {
          request,
          petName: context.petName,
          petState: context.petState,
          petHunger: context.petHunger,
          petMoodValue: context.petMoodValue,
          petEnergy: context.petEnergy,
          petCleanliness: context.petCleanliness,
          petHealth: context.petHealth,
          activityState: context.activityState,
          windowTitle: context.windowTitle,
          processName: context.processName,
          idleSeconds: context.idleSeconds,
          studyMode: context.studyMode,
          studyDurationSecs: context.studyDurationSecs,
          breakDurationSecs: context.breakDurationSecs,
          sessionCount: context.sessionCount,
          totalStudyTime: context.totalStudyTime,
          moodState: context.moodState,
          moodValue: context.moodValue,
        }) as DialogueResponse;
        return response;
      } catch (err) {
        setError(String(err));
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * 生成提醒
   */
  const generateReminder = useCallback(
    async (
      reminderType: ReminderType,
      context: {
        petName: string;
        petState: string;
        petHunger: number;
        petMoodValue: number;
        petEnergy: number;
        petCleanliness: number;
        petHealth: number;
        activityState: string;
        windowTitle: string;
        processName: string;
        idleSeconds: number;
        studyMode: string;
        studyDurationSecs: number;
        breakDurationSecs: number;
        sessionCount: number;
        totalStudyTime: number;
        moodState: string;
        moodValue: number;
      }
    ): Promise<ReminderResponse | null> => {
      setLoading(true);
      setError(null);
      try {
        const response = await invoke('generate_behavior_reminder', {
          reminderType,
          petName: context.petName,
          petState: context.petState,
          petHunger: context.petHunger,
          petMoodValue: context.petMoodValue,
          petEnergy: context.petEnergy,
          petCleanliness: context.petCleanliness,
          petHealth: context.petHealth,
          activityState: context.activityState,
          windowTitle: context.windowTitle,
          processName: context.processName,
          idleSeconds: context.idleSeconds,
          studyMode: context.studyMode,
          studyDurationSecs: context.studyDurationSecs,
          breakDurationSecs: context.breakDurationSecs,
          sessionCount: context.sessionCount,
          totalStudyTime: context.totalStudyTime,
          moodState: context.moodState,
          moodValue: context.moodValue,
        }) as ReminderResponse;
        return response;
      } catch (err) {
        setError(String(err));
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * 获取行为建议
   */
  const getBehaviorSuggestion = useCallback(
    async (context: {
      petName: string;
      petState: string;
      petHunger: number;
      petMoodValue: number;
      petEnergy: number;
      petCleanliness: number;
      petHealth: number;
      activityState: string;
      windowTitle: string;
      processName: string;
      idleSeconds: number;
      studyMode: string;
      studyDurationSecs: number;
      breakDurationSecs: number;
      sessionCount: number;
      totalStudyTime: number;
      moodState: string;
      moodValue: number;
    }): Promise<BehaviorSuggestion | null> => {
      setLoading(true);
      setError(null);
      try {
        const response = await invoke('get_behavior_suggestion', {
          petName: context.petName,
          petState: context.petState,
          petHunger: context.petHunger,
          petMoodValue: context.petMoodValue,
          petEnergy: context.petEnergy,
          petCleanliness: context.petCleanliness,
          petHealth: context.petHealth,
          activityState: context.activityState,
          windowTitle: context.windowTitle,
          processName: context.processName,
          idleSeconds: context.idleSeconds,
          studyMode: context.studyMode,
          studyDurationSecs: context.studyDurationSecs,
          breakDurationSecs: context.breakDurationSecs,
          sessionCount: context.sessionCount,
          totalStudyTime: context.totalStudyTime,
          moodState: context.moodState,
          moodValue: context.moodValue,
        }) as BehaviorSuggestion | null;
        return response;
      } catch (err) {
        setError(String(err));
        return null;
      } finally {
        setLoading(false);
      }
    },
    []
  );

  /**
   * 获取已注册的提供者列表
   */
  const getProviders = useCallback(async (): Promise<string[]> => {
    try {
      const providers = await invoke('get_behavior_providers') as string[];
      return providers;
    } catch (err) {
      console.error('获取提供者列表失败:', err);
      return [];
    }
  }, []);

  /**
   * 获取当前活跃提供者名称
   */
  const getActiveProvider = useCallback(async (): Promise<string> => {
    try {
      const provider = await invoke('get_active_behavior_provider') as string;
      return provider;
    } catch (err) {
      console.error('获取活跃提供者失败:', err);
      return 'rule';
    }
  }, []);

  /**
   * 切换活跃提供者
   */
  const switchProvider = useCallback(async (name: string): Promise<boolean> => {
    try {
      await invoke('switch_behavior_provider', { name });
      return true;
    } catch (err) {
      console.error('切换提供者失败:', err);
      return false;
    }
  }, []);

  /**
   * 注册 AI 提供者
   */
  const registerAIProvider = useCallback(
    async (provider: AIProvider, config: AIProviderConfig): Promise<boolean> => {
      try {
        await invoke('register_ai_provider', { provider, config });
        return true;
      } catch (err) {
        console.error('注册 AI 提供者失败:', err);
        return false;
      }
    },
    []
  );

  /**
   * 获取行为管理器配置
   */
  const getConfig = useCallback(async (): Promise<BehaviorManagerConfig | null> => {
    try {
      const config = await invoke('get_behavior_config') as BehaviorManagerConfig;
      return config;
    } catch (err) {
      console.error('获取配置失败:', err);
      return null;
    }
  }, []);

  /**
   * 更新行为管理器配置
   */
  const updateConfig = useCallback(async (config: BehaviorManagerConfig): Promise<boolean> => {
    try {
      await invoke('update_behavior_config', { config });
      return true;
    } catch (err) {
      console.error('更新配置失败:', err);
      return false;
    }
  }, []);

  return {
    loading,
    error,
    generateDialogue,
    generateReminder,
    getBehaviorSuggestion,
    getProviders,
    getActiveProvider,
    switchProvider,
    registerAIProvider,
    getConfig,
    updateConfig,
  };
};
