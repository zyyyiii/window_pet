export type PetEvent = 
  | { type: "TimeTick" }
  | { type: "UserInteract"; action: UserAction }
  | { type: "SystemEvent"; info: SystemInfo }
  | { type: "ReminderEvent"; reminder: Reminder }
  | { type: "AiDecision"; action: AiAction }
  | { type: "InteractionEvent"; interaction: InteractionType };

export type UserAction = 
  | { type: "Feed" }
  | { type: "Play" }
  | { type: "Pet" }
  | { type: "Talk" }
  | { type: "Command"; command: string };

export type InteractionType = 
  | { type: "Touch"; x: number; y: number }
  | { type: "DoubleTouch"; x: number; y: number }
  | { type: "RightClick"; x: number; y: number }
  | { type: "BubbleTrigger" };

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

// Dialogue types
export interface DialogueOption {
  id: string;
  text: string;
  next_node?: string;
  action?: UserAction;
}

export interface DialogueNode {
  id: string;
  speaker: "pet" | "user";
  text: string;
  options?: DialogueOption[];
  auto_next?: string;
  emotion?: PetEmotion;
}

export interface DialogueTree {
  id: string;
  start_node: string;
  nodes: Record<string, DialogueNode>;
  conditions?: DialogueCondition[];
}

export interface DialogueCondition {
  type: "state" | "attribute" | "random";
  value: string;
  operator: "eq" | "gt" | "lt" | "gte" | "lte";
  threshold?: number;
  probability?: number;
}

export type PetEmotion = 
  | "neutral"
  | "happy"
  | "sad"
  | "excited"
  | "sleepy"
  | "hungry"
  | "curious";

// Bubble types
export interface BubbleMessage {
  id: string;
  text: string;
  duration: number;
  emotion?: PetEmotion;
  trigger: BubbleTrigger;
}

export type BubbleTrigger = 
  | { type: "random"; interval: number }
  | { type: "state_change"; state: string }
  | { type: "attribute_threshold"; attribute: string; threshold: number }
  | { type: "interaction"; interaction: string };

// Menu types
export interface MenuItem {
  id: string;
  label: string;
  icon?: string;
  action: MenuAction;
  disabled?: boolean;
}

export type MenuAction = 
  | { type: "feed" }
  | { type: "play" }
  | { type: "talk" }
  | { type: "settings" }
  | { type: "exit" }
  | { type: "custom"; command: string };

// Interaction response
export interface InteractionResponse {
  interaction_type: string;
  animation?: string;
  dialogue?: DialogueTree;
  bubble?: BubbleMessage;
  menu_items?: MenuItem[];
  emotion?: PetEmotion;
}