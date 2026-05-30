import React, { useState } from "react";
import Panel from "../UI/Panel";

interface DialogPanelProps {
  onClose: () => void;
  petName: string;
}

const DialogPanel: React.FC<DialogPanelProps> = ({ onClose, petName }) => {
  const [message, setMessage] = useState("");
  const [chatHistory, setChatHistory] = useState<
    { sender: "user" | "pet"; message: string }[]
  >([]);

  const handleSend = () => {
    if (!message.trim()) return;

    setChatHistory((prev) => [...prev, { sender: "user", message: message.trim() }]);

    // TODO: Implement actual AI response
    setTimeout(() => {
      setChatHistory((prev) => [
        ...prev,
        { sender: "pet", message: `Meow! I heard you say "${message.trim()}"` },
      ]);
    }, 500);

    setMessage("");
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      handleSend();
    }
  };

  return (
    <Panel title={`Chat with ${petName}`} onClose={onClose} className="dialog-panel">
      <div className="chat-history">
        {chatHistory.map((chat, index) => (
          <div key={index} className={`chat-message chat-${chat.sender}`}>
            <span className="chat-sender">
              {chat.sender === "user" ? "You" : petName}:
            </span>
            <span className="chat-text">{chat.message}</span>
          </div>
        ))}
      </div>
      <div className="chat-input">
        <input
          type="text"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Type a message..."
        />
        <button onClick={handleSend}>Send</button>
      </div>
    </Panel>
  );
};

export default DialogPanel;