import React, { useEffect, useState } from "react";
import type { BubbleMessage } from "../../types/events";

interface ChatBubbleProps {
  message: BubbleMessage | null;
  onDismiss?: () => void;
  className?: string;
}

const ChatBubble: React.FC<ChatBubbleProps> = ({
  message,
  onDismiss,
  className = "",
}) => {
  const [isVisible, setIsVisible] = useState(false);
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    if (message) {
      setIsAnimating(true);
      setTimeout(() => {
        setIsVisible(true);
        setIsAnimating(false);
      }, 50);
    } else {
      setIsVisible(false);
    }
  }, [message]);

  if (!message) return null;

  return (
    <div
      className={`chat-bubble ${isVisible ? "visible" : ""} ${isAnimating ? "animating" : ""} ${className}`}
      onClick={onDismiss}
    >
      <div className="chat-bubble-content">
        <div className="chat-bubble-text">{message.text}</div>
        {message.emotion && (
          <div className="chat-bubble-emotion">
            {getEmotionEmoji(message.emotion)}
          </div>
        )}
      </div>
      <div className="chat-bubble-tail" />
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

export default ChatBubble;