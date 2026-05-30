import React, { useState } from "react";
import Panel from "../UI/Panel";

interface Reminder {
  id: string;
  title: string;
  time: string;
  enabled: boolean;
}

interface ReminderPanelProps {
  onClose: () => void;
}

const ReminderPanel: React.FC<ReminderPanelProps> = ({ onClose }) => {
  const [reminders, setReminders] = useState<Reminder[]>([]);
  const [newTitle, setNewTitle] = useState("");
  const [newTime, setNewTime] = useState("");

  const addReminder = () => {
    if (!newTitle.trim() || !newTime.trim()) return;

    const reminder: Reminder = {
      id: Date.now().toString(),
      title: newTitle.trim(),
      time: newTime.trim(),
      enabled: true,
    };

    setReminders((prev) => [...prev, reminder]);
    setNewTitle("");
    setNewTime("");
  };

  const toggleReminder = (id: string) => {
    setReminders((prev) =>
      prev.map((r) => (r.id === id ? { ...r, enabled: !r.enabled } : r))
    );
  };

  const removeReminder = (id: string) => {
    setReminders((prev) => prev.filter((r) => r.id !== id));
  };

  return (
    <Panel title="Reminders" onClose={onClose} className="reminder-panel">
      <div className="reminder-list">
        {reminders.map((reminder) => (
          <div key={reminder.id} className="reminder-item">
            <input
              type="checkbox"
              checked={reminder.enabled}
              onChange={() => toggleReminder(reminder.id)}
            />
            <span className="reminder-title">{reminder.title}</span>
            <span className="reminder-time">{reminder.time}</span>
            <button onClick={() => removeReminder(reminder.id)}>×</button>
          </div>
        ))}
      </div>
      <div className="reminder-add">
        <input
          type="text"
          value={newTitle}
          onChange={(e) => setNewTitle(e.target.value)}
          placeholder="Reminder title"
        />
        <input
          type="time"
          value={newTime}
          onChange={(e) => setNewTime(e.target.value)}
        />
        <button onClick={addReminder}>Add</button>
      </div>
    </Panel>
  );
};

export default ReminderPanel;