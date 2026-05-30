import React from "react";
import Panel from "../UI/Panel";
import { PetStatus } from "../../types/pet";

interface StatusPanelProps {
  status: PetStatus | null;
}

const StatusPanel: React.FC<StatusPanelProps> = ({ status }) => {
  if (!status) {
    return null;
  }

  const { attributes } = status;

  return (
    <Panel title="Status" className="status-panel">
      <div className="status-grid">
        <div className="status-item">
          <span className="status-label">State:</span>
          <span className="status-value">{status.state}</span>
        </div>
        <div className="status-item">
          <span className="status-label">Hunger:</span>
          <div className="status-bar">
            <div
              className="status-bar-fill"
              style={{ width: `${100 - attributes.hunger}%` }}
            />
          </div>
        </div>
        <div className="status-item">
          <span className="status-label">Mood:</span>
          <div className="status-bar">
            <div
              className="status-bar-fill"
              style={{ width: `${attributes.mood}%` }}
            />
          </div>
        </div>
        <div className="status-item">
          <span className="status-label">Energy:</span>
          <div className="status-bar">
            <div
              className="status-bar-fill"
              style={{ width: `${attributes.energy}%` }}
            />
          </div>
        </div>
        <div className="status-item">
          <span className="status-label">Cleanliness:</span>
          <div className="status-bar">
            <div
              className="status-bar-fill"
              style={{ width: `${attributes.cleanliness}%` }}
            />
          </div>
        </div>
        <div className="status-item">
          <span className="status-label">Health:</span>
          <div className="status-bar">
            <div
              className="status-bar-fill"
              style={{ width: `${attributes.health}%` }}
            />
          </div>
        </div>
      </div>
    </Panel>
  );
};

export default StatusPanel;