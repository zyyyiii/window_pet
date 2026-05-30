import { create } from "zustand";

interface AppConfig {
  petName: string;
  autoStart: boolean;
  alwaysOnTop: boolean;
  transparency: number;
  animationSpeed: number;
  updateInterval: number;
  enableAi: boolean;
  enableMonitoring: boolean;
  enableReminders: boolean;
}

interface ConfigStore {
  config: AppConfig;
  setConfig: (config: Partial<AppConfig>) => void;
  resetConfig: () => void;
}

const defaultConfig: AppConfig = {
  petName: "Pet",
  autoStart: true,
  alwaysOnTop: true,
  transparency: 0.9,
  animationSpeed: 1.0,
  updateInterval: 1000,
  enableAi: false,
  enableMonitoring: false,
  enableReminders: false,
};

export const useConfigStore = create<ConfigStore>((set) => ({
  config: defaultConfig,
  setConfig: (newConfig) => set((state) => ({
    config: { ...state.config, ...newConfig },
  })),
  resetConfig: () => set({ config: defaultConfig }),
}));