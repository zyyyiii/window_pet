declare module "@tauri-apps/api/core" {
  export function invoke(cmd: string, args?: Record<string, unknown>): Promise<unknown>;
}

declare module "@tauri-apps/api/event" {
  export function listen<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void>;
  export function once<T>(event: string, handler: (event: { payload: T }) => void): Promise<() => void>;
  export function emit(event: string, payload?: unknown): Promise<void>;
}