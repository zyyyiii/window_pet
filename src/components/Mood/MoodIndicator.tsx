import React from "react";
import type { MoodSnapshot } from "../../types/mood";
import { MOOD_STATE_INFO } from "../../types/mood";

interface MoodIndicatorProps {
  snapshot: MoodSnapshot | null;
  showValue?: boolean;
  showLabel?: boolean;
  className?: string;
}

const MoodIndicator: React.FC<MoodIndicatorProps> = ({
  snapshot,
  showValue = true,
  showLabel = true,
  className = "",
}) => {
  if (!snapshot) return null;

  const info = MOOD_STATE_INFO[snapshot.state];

  return (
    <div className={`mood-indicator ${className}`}>
      <div className="mood-emoji" title={info.label}>
        {info.emoji}
      </div>
      {showLabel && (
        <div className="mood-label" style={{ color: info.color }}>
          {info.label}
        </div>
      )}
      {showValue && (
        <div className="mood-bar-container">
          <div
            className="mood-bar-fill"
            style={{
              width: `${snapshot.value}%`,
              backgroundColor: info.color,
            }}
          />
        </div>
      )}
    </div>
  );
};

export default MoodIndicator;