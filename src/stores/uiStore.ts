import { create } from "zustand";

interface UIStore {
  showStatusPanel: boolean;
  showDialogPanel: boolean;
  showSettingsPanel: boolean;
  showReminderPanel: boolean;
  toggleStatusPanel: () => void;
  toggleDialogPanel: () => void;
  toggleSettingsPanel: () => void;
  toggleReminderPanel: () => void;
  closeAllPanels: () => void;
}

export const useUIStore = create<UIStore>((set) => ({
  showStatusPanel: true,
  showDialogPanel: false,
  showSettingsPanel: false,
  showReminderPanel: false,
  toggleStatusPanel: () => set((state) => ({ showStatusPanel: !state.showStatusPanel })),
  toggleDialogPanel: () => set((state) => ({ showDialogPanel: !state.showDialogPanel })),
  toggleSettingsPanel: () => set((state) => ({ showSettingsPanel: !state.showSettingsPanel })),
  toggleReminderPanel: () => set((state) => ({ showReminderPanel: !state.showReminderPanel })),
  closeAllPanels: () => set({
    showStatusPanel: false,
    showDialogPanel: false,
    showSettingsPanel: false,
    showReminderPanel: false,
  }),
}));