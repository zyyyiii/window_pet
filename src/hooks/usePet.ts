import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { PetStatus } from "../types/pet";

export function usePet() {
  const [petStatus, setPetStatus] = useState<PetStatus | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchPetStatus = useCallback(async () => {
    try {
      const status = await invoke("get_pet_status") as PetStatus;
      setPetStatus(status);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setIsLoading(false);
    }
  }, []);

  const feedPet = useCallback(async () => {
    try {
      const status = await invoke("feed_pet") as PetStatus;
      setPetStatus(status);
    } catch (err) {
      setError(String(err));
    }
  }, []);

  const playWithPet = useCallback(async () => {
    try {
      const status = await invoke("play_with_pet") as PetStatus;
      setPetStatus(status);
    } catch (err) {
      setError(String(err));
    }
  }, []);

  useEffect(() => {
    fetchPetStatus();

    const unlisten = listen("pet_status_update", (event) => {
      setPetStatus(event.payload as PetStatus);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [fetchPetStatus]);

  return {
    petStatus,
    isLoading,
    error,
    feedPet,
    playWithPet,
    refreshStatus: fetchPetStatus,
  };
}