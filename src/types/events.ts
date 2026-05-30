export type PetEvent = 
  | { type: "TimeTick" }
  | { type: "UserInteract"; action: UserAction }
  | { type: "SystemEvent"; info: SystemInfo }
  | { type: "ReminderEvent"; reminder: Reminder }
  | { type: "AiDecision"; action: AiAction };

export type UserAction = 
  | { type: "Feed" }
  | { type: "Play" }
  | { type: "Pet" }
  | { type: "Talk" }
  | { type: "Command"; command: string };

export interface SystemInfo {
  cpu_usage: number;
  memory_usage: number;
  network_activity: boolean;
}

export interface Reminder {
  id: string;
  title: string;
  message: string;
  reminder_type: ReminderType;
}

export type ReminderType = 
  | { type: "Study" }
  | { type: "Break" }
  | { type: "Custom"; name: string };

export type AiAction = 
  | { type: "Wander" }
  | { type: "React"; reaction: string }
  | { type: "Suggest"; suggestion: string };