import { useState, useCallback, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  InteractionResponse,
  BubbleMessage,
  DialogueTree,
  MenuItem,
} from "../types/events";

interface UseInteractionOptions {
  onBubbleShow?: (message: BubbleMessage) => void;
  onBubbleHide?: () => void;
  onDialogueStart?: (dialogue: DialogueTree) => void;
  onDialogueEnd?: () => void;
  onMenuShow?: (items: MenuItem[]) => void;
  onMenuHide?: () => void;
  onTouch?: (x: number, y: number) => void;
}

export function useInteraction(options: UseInteractionOptions = {}) {
  const [bubble, setBubble] = useState<BubbleMessage | null>(null);
  const [dialogue, setDialogue] = useState<DialogueTree | null>(null);
  const [menuItems, setMenuItems] = useState<MenuItem[] | null>(null);
  const [menuPosition, setMenuPosition] = useState<{ x: number; y: number } | null>(null);
  
  const clickTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const clickCountRef = useRef(0);
  const bubbleTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handleBubbleTimer = useCallback(() => {
    if (bubbleTimerRef.current) {
      clearTimeout(bubbleTimerRef.current);
    }
    bubbleTimerRef.current = setTimeout(() => {
      setBubble(null);
      options.onBubbleHide?.();
    }, bubble?.duration || 3000);
  }, [bubble?.duration, options]);

  useEffect(() => {
    if (bubble) {
      handleBubbleTimer();
    }
    return () => {
      if (bubbleTimerRef.current) {
        clearTimeout(bubbleTimerRef.current);
      }
    };
  }, [bubble, handleBubbleTimer]);

  const processResponse = useCallback((response: InteractionResponse) => {
    if (response.bubble) {
      setBubble(response.bubble);
      options.onBubbleShow?.(response.bubble);
    }

    if (response.dialogue) {
      setDialogue(response.dialogue);
      options.onDialogueStart?.(response.dialogue);
    }

    if (response.menu_items) {
      setMenuItems(response.menu_items);
      options.onMenuShow?.(response.menu_items);
    }

    if (response.emotion) {
      // Trigger emotion change
    }
  }, [options]);

  const handleTouch = useCallback(async (x: number, y: number) => {
    try {
      const response = await invoke("handle_interaction", {
        interactionType: "touch",
        x,
        y,
      }) as InteractionResponse;
      processResponse(response);
      options.onTouch?.(x, y);
    } catch (error) {
      console.error("Touch interaction failed:", error);
    }
  }, [processResponse, options]);

  const handleDoubleTouch = useCallback(async (x: number, y: number) => {
    try {
      const response = await invoke("handle_interaction", {
        interactionType: "double_touch",
        x,
        y,
      }) as InteractionResponse;
      processResponse(response);
    } catch (error) {
      console.error("Double touch interaction failed:", error);
    }
  }, [processResponse]);

  const handleRightClick = useCallback(async (x: number, y: number) => {
    try {
      const response = await invoke("handle_interaction", {
        interactionType: "right_click",
        x,
        y,
      }) as InteractionResponse;
      setMenuPosition({ x, y });
      processResponse(response);
    } catch (error) {
      console.error("Right click interaction failed:", error);
    }
  }, [processResponse]);

  const handleClick = useCallback((e: React.MouseEvent) => {
    const x = e.clientX;
    const y = e.clientY;

    clickCountRef.current += 1;

    if (clickTimerRef.current) {
      clearTimeout(clickTimerRef.current);
    }

    clickTimerRef.current = setTimeout(() => {
      if (clickCountRef.current === 1) {
        handleTouch(x, y);
      } else if (clickCountRef.current >= 2) {
        handleDoubleTouch(x, y);
      }
      clickCountRef.current = 0;
    }, 250);
  }, [handleTouch, handleDoubleTouch]);

  const handleContextMenu = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    handleRightClick(e.clientX, e.clientY);
  }, [handleRightClick]);

  const closeMenu = useCallback(() => {
    setMenuItems(null);
    setMenuPosition(null);
    options.onMenuHide?.();
  }, [options]);

  const closeDialogue = useCallback(() => {
    setDialogue(null);
    options.onDialogueEnd?.();
  }, [options]);

  const hideBubble = useCallback(async () => {
    try {
      await invoke("hide_bubble");
      setBubble(null);
      options.onBubbleHide?.();
    } catch (error) {
      console.error("Hide bubble failed:", error);
    }
  }, [options]);

  const triggerRandomBubble = useCallback(async () => {
    try {
      const response = await invoke("get_random_bubble") as BubbleMessage | null;
      if (response) {
        setBubble(response);
        options.onBubbleShow?.(response);
      }
    } catch (error) {
      console.error("Trigger random bubble failed:", error);
    }
  }, [options]);

  return {
    bubble,
    dialogue,
    menuItems,
    menuPosition,
    handleClick,
    handleContextMenu,
    closeMenu,
    closeDialogue,
    hideBubble,
    triggerRandomBubble,
  };
}