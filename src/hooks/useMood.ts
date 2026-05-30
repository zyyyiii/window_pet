import { useState, useEffect, useCallback, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { MoodSnapshot, MoodState } from "../types/mood";

interface UseMoodOptions {
  onUpdate?: (snapshot: MoodSnapshot) => void;
  onStateChange?: (newState: MoodState, oldState: MoodState) => void;
  pollInterval?: number;
}

export function useMood(options: UseMoodOptions = {}) {
  const [snapshot, setSnapshot] = useState<MoodSnapshot | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const previousStateRef = useRef<MoodState | null>(null);
  const pollIntervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const fetchSnapshot = useCallback(async () => {
    try {
      const result = await invoke("get_mood_snapshot") as MoodSnapshot;
      setSnapshot(result);
      setError(null);

      if (previousStateRef.current && previousStateRef.current !== result.state) {
        options.onStateChange?.(result.state, previousStateRef.current);
      }
      previousStateRef.current = result.state;
      options.onUpdate?.(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, [options]);

  useEffect(() => {
    fetchSnapshot();

    const interval = options.pollInterval || 5000;
    pollIntervalRef.current = setInterval(fetchSnapshot, interval);

    const unlisten = listen<MoodSnapshot>("mood_update", (event) => {
      setSnapshot(event.payload);
      options.onUpdate?.(event.payload);
    });

    return () => {
      if (pollIntervalRef.current) {
        clearInterval(pollIntervalRef.current);
      }
      unlisten.then(fn => fn());
    };
  }, [fetchSnapshot, options]);

  const setValue = useCallback(async (value: number) => {
    try {
      const result = await invoke("set_mood_value", { value }) as MoodSnapshot;
      setSnapshot(result);
      return result;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  const applyBoost = useCallback(async (amount?: number) => {
    try {
      const result = await invoke("apply_mood_interaction_boost", { amount }) as MoodSnapshot;
      setSnapshot(result);
      return result;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  const getAnimationHint = useCallback(async () => {
    try {
      return await invoke("get_mood_animation_hint") as string;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  const getEmoji = useCallback(async () => {
    try {
      return await invoke("get_mood_emoji") as string;
    } catch (err) {
      setError(String(err));
      throw err;
    }
  }, []);

  return {
    snapshot,
    isLoading,
    error,
    setValue,
    applyBoost,
    getAnimationHint,
    getEmoji,
    refresh: fetchSnapshot,
  };
}