export type AnimationProviderType = 'sprite_sheet' | 'live2d' | 'spine';

export interface SpriteSheetConfig {
  type: AnimationProviderType;
  image: string;
  frame_width: number;
  frame_height: number;
  frame_count: number;
  frame_rate: number;
  loop: boolean;
  columns: number;
  offset_x?: number;
  offset_y?: number;
}

export interface TransitionConfig {
  from: string;
  to: string;
  duration: number;
}

export interface AnimationConfig {
  version: string;
  default_animation: string;
  sprite_sheets: Record<string, SpriteSheetConfig>;
  state_mapping: Record<string, string>;
  transitions: Record<string, TransitionConfig>;
}

export interface AnimationState {
  current_animation: string;
  frame_index: number;
  elapsed_time: number;
  is_playing: boolean;
  is_transitioning: boolean;
  transition_progress: number;
}

export interface AnimationUpdate {
  animation: string;
  frame_index: number;
  source_x: number;
  source_y: number;
  source_width: number;
  source_height: number;
  image_path: string;
}

export interface AnimationTransition {
  from_animation: string;
  to_animation: string;
  progress: number;
  from_frame: AnimationUpdate;
  to_frame: AnimationUpdate;
}