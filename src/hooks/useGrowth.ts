import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { GrowthSnapshot, LevelInfo, InteractionType } from '../types/growth';

/**
 * 成长系统 Hook
 */
export function useGrowth() {
  const [snapshot, setSnapshot] = useState<GrowthSnapshot | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 获取成长快照
  const fetchSnapshot = useCallback(async () => {
    try {
      const data = await invoke('get_growth_snapshot') as GrowthSnapshot;
      setSnapshot(data);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  // 初始加载
  useEffect(() => {
    fetchSnapshot();
    // 每分钟更新一次陪伴时间
    const interval = setInterval(fetchSnapshot, 60000);
    return () => clearInterval(interval);
  }, [fetchSnapshot]);

  // 记录交互
  const recordInteraction = useCallback(async (type: InteractionType) => {
    try {
      const exp = await invoke('record_growth_interaction', {
        interactionType: type,
      }) as number;
      // 刷新快照
      await fetchSnapshot();
      return exp;
    } catch (err) {
      setError(String(err));
      return 0;
    }
  }, [fetchSnapshot]);

  // 添加学习积分
  const addLearningPoints = useCallback(async (amount: number) => {
    try {
      const total = await invoke('add_learning_points', { amount }) as number;
      await fetchSnapshot();
      return total;
    } catch (err) {
      setError(String(err));
      return 0;
    }
  }, [fetchSnapshot]);

  // 添加记忆
  const addMemory = useCallback(async (content: string, importance: number) => {
    try {
      const id = await invoke('add_growth_memory', { content, importance }) as string;
      await fetchSnapshot();
      return id;
    } catch (err) {
      setError(String(err));
      return null;
    }
  }, [fetchSnapshot]);

  // 获取等级信息
  const getLevelInfo = useCallback(async () => {
    try {
      return await invoke('get_level_info') as LevelInfo;
    } catch (err) {
      setError(String(err));
      return null;
    }
  }, []);

  // 保存数据
  const save = useCallback(async () => {
    try {
      await invoke('save_growth_data');
    } catch (err) {
      setError(String(err));
    }
  }, []);

  return {
    snapshot,
    loading,
    error,
    recordInteraction,
    addLearningPoints,
    addMemory,
    getLevelInfo,
    save,
    refresh: fetchSnapshot,
  };
}

/**
 * 格式化经验值显示
 */
export function formatExp(exp: number): string {
  if (exp >= 1000000) {
    return `${(exp / 1000000).toFixed(1)}M`;
  }
  if (exp >= 1000) {
    return `${(exp / 1000).toFixed(1)}K`;
  }
  return String(exp);
}

/**
 * 获取等级称号
 */
export function getLevelTitle(level: number): string {
  if (level >= 90) return '传奇伙伴';
  if (level >= 80) return '至尊挚友';
  if (level >= 70) return '亲密挚友';
  if (level >= 60) return '知心好友';
  if (level >= 50) return '挚友';
  if (level >= 40) return '好友';
  if (level >= 30) return '伙伴';
  if (level >= 20) return '朋友';
  if (level >= 10) return '熟人';
  if (level >= 5) return '相识';
  return '初见';
}
