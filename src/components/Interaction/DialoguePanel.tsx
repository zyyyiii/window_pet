import React, { useState, useCallback } from "react";
import type { DialogueTree, DialogueNode, DialogueOption } from "../../types/events";

interface DialoguePanelProps {
  dialogue: DialogueTree | null;
  onClose: () => void;
  onOptionSelect?: (option: DialogueOption) => void;
}

const DialoguePanel: React.FC<DialoguePanelProps> = ({
  dialogue,
  onClose,
  onOptionSelect,
}) => {
  const [currentNodeId, setCurrentNodeId] = useState<string | null>(null);

  const currentNode: DialogueNode | undefined = dialogue
    ? dialogue.nodes[currentNodeId || dialogue.start_node]
    : undefined;

  const handleOptionClick = useCallback(
    (option: DialogueOption) => {
      if (option.next_node) {
        setCurrentNodeId(option.next_node);
      } else {
        onOptionSelect?.(option);
        onClose();
      }
    },
    [onOptionSelect, onClose]
  );

  if (!dialogue || !currentNode) return null;

  return (
    <div className="dialogue-panel">
      <div className="dialogue-header">
        <div className="dialogue-speaker">
          {currentNode.speaker === "pet" ? "🐱 宠物" : "👤 你"}
        </div>
        <button className="dialogue-close" onClick={onClose}>
          ×
        </button>
      </div>

      <div className="dialogue-content">
        <div className="dialogue-text">{currentNode.text}</div>
        {currentNode.emotion && (
          <div className="dialogue-emotion">
            {getEmotionEmoji(currentNode.emotion)}
          </div>
        )}
      </div>

      {currentNode.options && currentNode.options.length > 0 && (
        <div className="dialogue-options">
          {currentNode.options.map((option) => (
            <button
              key={option.id}
              className="dialogue-option"
              onClick={() => handleOptionClick(option)}
            >
              {option.text}
            </button>
          ))}
        </div>
      )}
    </div>
  );
};

function getEmotionEmoji(emotion: string): string {
  switch (emotion) {
    case "happy":
      return "😊";
    case "sad":
      return "😢";
    case "excited":
      return "🤩";
    case "sleepy":
      return "😴";
    case "hungry":
      return "😋";
    case "curious":
      return "🧐";
    default:
      return "😺";
  }
}

export default DialoguePanel;