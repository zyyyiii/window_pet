import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { PetStatus } from "../types/pet";
import { SystemInfo } from "../types/events";

export class TauriBridge {
  static async getPetStatus(): Promise<PetStatus> {
    return invoke("get_pet_status") as Promise<PetStatus>;
  }

  static async feedPet(): Promise<PetStatus> {
    return invoke("feed_pet") as Promise<PetStatus>;
  }

  static async playWithPet(): Promise<PetStatus> {
    return invoke("play_with_pet") as Promise<PetStatus>;
  }

  static async getSystemInfo(): Promise<SystemInfo> {
    return invoke("get_system_info") as Promise<SystemInfo>;
  }

  static async listenToPetStatusUpdates(
    callback: (status: PetStatus) => void,
  ): Promise<() => void> {
    return listen("pet_status_update", (event) => {
      callback(event.payload as PetStatus);
    });
  }

  static async listenToSystemUpdates(
    callback: (info: SystemInfo) => void,
  ): Promise<() => void> {
    return listen("system_update", (event) => {
      callback(event.payload as SystemInfo);
    });
  }
}