import { create } from "zustand";
import { PetStatus } from "../types/pet";

interface PetStore {
  petStatus: PetStatus | null;
  isLoading: boolean;
  error: string | null;
  setPetStatus: (status: PetStatus) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const usePetStore = create<PetStore>((set) => ({
  petStatus: null,
  isLoading: true,
  error: null,
  setPetStatus: (status) => set({ petStatus: status, isLoading: false, error: null }),
  setLoading: (loading) => set({ isLoading: loading }),
  setError: (error) => set({ error, isLoading: false }),
}));