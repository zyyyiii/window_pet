import { useState, useEffect, useCallback, useRef } from "react";
import { AnimationService } from "../services/animationService";
import type { AnimationConfig, AnimationUpdate } from "../types/animation";

export function useAnimation(state: string) {
  const [config, setConfig] = useState<AnimationConfig | null>(null);
  const [currentUpdate, setCurrentUpdate] = useState<AnimationUpdate | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const unsubscribeRef = useRef<(() => void) | null>(null);

  const animationService = AnimationService.getInstance();

  useEffect(() => {
    const loadConfig = async () => {
      try {
        const loadedConfig = await animationService.loadConfig();
        setConfig(loadedConfig);
        setIsLoading(false);
      } catch (err) {
        setError("Failed to load animation config");
        setIsLoading(false);
      }
    };

    loadConfig();
  }, []);

  useEffect(() => {
    const unsubscribe = animationService.onAnimationUpdate((update) => {
      setCurrentUpdate(update);
    });

    unsubscribeRef.current = unsubscribe;

    return () => {
      unsubscribe();
    };
  }, []);

  useEffect(() => {
    if (config) {
      animationService.setStateAnimation(state);
    }
  }, [state, config]);

  const getAnimationForState = useCallback(
    (stateName: string): string | null => {
      return animationService.getAnimationForState(stateName);
    },
    []
  );

  const getSpriteConfig = useCallback(
    (animationName: string) => {
      return animationService.getAnimationConfig(animationName);
    },
    []
  );

  return {
    config,
    currentUpdate,
    isLoading,
    error,
    getAnimationForState,
    getSpriteConfig,
  };
}