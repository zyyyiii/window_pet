import React from "react";
import { useInteraction } from "../../hooks/useInteraction";
import ChatBubble from "./ChatBubble";
import ContextMenu from "./ContextMenu";
import DialoguePanel from "./DialoguePanel";

interface InteractionContainerProps {
  children: React.ReactNode;
  onMenuAction?: (action: string) => void;
  className?: string;
}

const InteractionContainer: React.FC<InteractionContainerProps> = ({
  children,
  onMenuAction,
  className = "",
}) => {
  const {
    bubble,
    dialogue,
    menuItems,
    menuPosition,
    handleClick,
    handleContextMenu,
    closeMenu,
    closeDialogue,
    hideBubble,
  } = useInteraction({
    onMenuShow: (items) => {
      console.log("Menu shown:", items);
    },
    onMenuHide: () => {
      console.log("Menu hidden");
    },
    onDialogueStart: (dialogue) => {
      console.log("Dialogue started:", dialogue);
    },
    onDialogueEnd: () => {
      console.log("Dialogue ended");
    },
    onBubbleShow: (message) => {
      console.log("Bubble shown:", message);
    },
    onBubbleHide: () => {
      console.log("Bubble hidden");
    },
  });

  const handleMenuAction = (action: string) => {
    onMenuAction?.(action);
    closeMenu();
  };

  return (
    <div
      className={`interaction-container ${className}`}
      onClick={handleClick}
      onContextMenu={handleContextMenu}
    >
      {children}

      <ChatBubble message={bubble} onDismiss={hideBubble} />

      <ContextMenu
        items={menuItems || []}
        position={menuPosition}
        onAction={handleMenuAction}
        onClose={closeMenu}
      />

      <DialoguePanel
        dialogue={dialogue}
        onClose={closeDialogue}
      />
    </div>
  );
};

export default InteractionContainer;