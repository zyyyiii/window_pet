import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";

export function useTauriEvent<T>(
  event: string,
  handler: (payload: T) => void,
) {
  const handlerRef = useRef(handler);
  handlerRef.current = handler;

  useEffect(() => {
    const unlisten = listen(event, (event) => {
      handlerRef.current(event.payload as T);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [event]);
}