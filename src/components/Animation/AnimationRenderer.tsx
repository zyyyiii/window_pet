import React, { useEffect, useState, useCallback } from "react";
import { AnimationService } from "../../services/animationService";
import SpriteAnimation from "./SpriteAnimation";
import type {
  AnimationConfig,
  AnimationUpdate,
  SpriteSheetConfig,
} from "../../types/animation";

interface AnimationRendererProps {
  state: string;
  fallback?: React.ReactNode;
  className?: string;
}

const AnimationRenderer: React.FC<AnimationRendererProps> = ({
  state,
  fallback,
  className = "",
}) => {
  const [config, setConfig] = useState<AnimationConfig | null>(null);
  const [currentUpdate, setCurrentUpdate] = useState<AnimationUpdate | null>(null);
  const [currentAnimation, setCurrentAnimation] = useState<string>("idle");
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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
    const animationName = animationService.getAnimationForState(state);
    if (animationName) {
      setCurrentAnimation(animationName);
      animationService.setStateAnimation(state);
    }
  }, [state]);

  useEffect(() => {
    const unsubscribe = animationService.onAnimationUpdate((update) => {
      setCurrentUpdate(update);
    });

    return unsubscribe;
  }, []);

  const getCurrentSpriteConfig = useCallback((): SpriteSheetConfig | null => {
    if (!config) return null;
    return config.sprite_sheets[currentAnimation] ?? null;
  }, [config, currentAnimation]);

  if (isLoading) {
    return <div className="animation-loading">Loading...</div>;
  }

  if (error) {
    return fallback ? <>{fallback}</> : <div className="animation-error">{error}</div>;
  }

  const spriteConfig = getCurrentSpriteConfig();

  if (!spriteConfig) {
    return fallback ? <>{fallback}</> : <div className="animation-error">No animation config</div>;
  }

  return (
    <div className={`animation-renderer ${className}`}>
      <SpriteAnimation
        config={spriteConfig}
        animationUpdate={currentUpdate}
      />
    </div>
  );
};

export default AnimationRenderer;