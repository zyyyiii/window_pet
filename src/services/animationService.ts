import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AnimationConfig, AnimationUpdate, AnimationState } from "../types/animation";

export class AnimationService {
  private static instance: AnimationService;
  private config: AnimationConfig | null = null;
  private listeners: ((update: AnimationUpdate) => void)[] = [];
  private stateListeners: ((state: AnimationState) => void)[] = [];

  private constructor() {}

  static getInstance(): AnimationService {
    if (!AnimationService.instance) {
      AnimationService.instance = new AnimationService();
    }
    return AnimationService.instance;
  }

  async loadConfig(): Promise<AnimationConfig> {
    try {
      const response = await fetch("/animations/animations.json");
      this.config = await response.json();
      return this.config!;
    } catch (error) {
      console.error("Failed to load animation config:", error);
      throw error;
    }
  }

  getConfig(): AnimationConfig | null {
    return this.config;
  }

  getAnimationConfig(name: string) {
    return this.config?.sprite_sheets[name] ?? null;
  }

  getStateMapping(): Record<string, string> {
    return this.config?.state_mapping ?? {};
  }

  getAnimationForState(state: string): string | null {
    return this.config?.state_mapping[state] ?? null;
  }

  async setAnimation(animationName: string): Promise<void> {
    try {
      await invoke("set_animation", { animationName });
    } catch (error) {
      console.error("Failed to set animation:", error);
    }
  }

  async setStateAnimation(stateName: string): Promise<void> {
    const animationName = this.getAnimationForState(stateName);
    if (animationName) {
      await this.setAnimation(animationName);
    }
  }

  onAnimationUpdate(callback: (update: AnimationUpdate) => void): () => void {
    this.listeners.push(callback);
    
    const unlisten = listen<AnimationUpdate>("animation_update", (event) => {
      callback(event.payload);
    });

    return () => {
      this.listeners = this.listeners.filter(l => l !== callback);
      unlisten.then(fn => fn());
    };
  }

  onStateUpdate(callback: (state: AnimationState) => void): () => void {
    this.stateListeners.push(callback);
    
    const unlisten = listen<AnimationState>("animation_state_update", (event) => {
      callback(event.payload);
    });

    return () => {
      this.stateListeners = this.stateListeners.filter(l => l !== callback);
      unlisten.then(fn => fn());
    };
  }

  async getAvailableAnimations(): Promise<string[]> {
    try {
      const result = await invoke("get_available_animations");
      return result as string[];
    } catch (error) {
      console.error("Failed to get available animations:", error);
      return [];
    }
  }
}