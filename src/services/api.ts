import { TauriBridge } from "./tauriBridge";
import { PetStatus } from "../types/pet";
import { SystemInfo } from "../types/events";

export const api = {
  pet: {
    getStatus: () => TauriBridge.getPetStatus(),
    feed: () => TauriBridge.feedPet(),
    play: () => TauriBridge.playWithPet(),
    listenToUpdates: (callback: (status: PetStatus) => void) =>
      TauriBridge.listenToPetStatusUpdates(callback),
  },
  system: {
    getInfo: () => TauriBridge.getSystemInfo(),
    listenToUpdates: (callback: (info: SystemInfo) => void) =>
      TauriBridge.listenToSystemUpdates(callback),
  },
};