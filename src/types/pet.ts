export type PetState = 
  | "idle"
  | "happy"
  | "hungry"
  | "sleepy"
  | "playing"
  | "studying"
  | "monitoring"
  | "talking";

export interface PetAttributes {
  hunger: number;
  mood: number;
  energy: number;
  cleanliness: number;
  health: number;
}

export interface PetStatus {
  state: PetState;
  attributes: PetAttributes;
  name: string;
}

export interface PetModel {
  id: string;
  name: string;
  state: PetState;
  attributes: PetAttributes;
  created_at: string;
  last_updated: string;
}