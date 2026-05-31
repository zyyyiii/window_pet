import React from 'react';
import { useStudy } from '../../hooks/useStudy';
import {
  formatDuration,
  getModeDisplayName,
  getModeIcon,
} from '../../types/study';

interface StudyIndicatorProps {
  className?: string;
}

/**
 * 学习状态指示器组件
 * 显示当前学习模式和计时
 */
export const StudyIndicator: React.FC<StudyIndicatorProps> = ({
  className = '',
}) => {
  const { snapshot, loading } = useStudy(2000);

  if (loading || !snapshot) {
    return null;
  }

  // 普通模式下不显示指示器
  if (snapshot.mode === 'normal') {
    return null;
  }

  const currentDuration = snapshot.currentSession?.durationSecs ?? 0;
  const modeIcon = getModeIcon(snapshot.mode);
  const modeName = getModeDisplayName(snapshot.mode);

  return (
    <div className={`study-indicator ${className}`}>
      <div className="study-indicator-content">
        <span className="study-mode-icon">{modeIcon}</span>
        <span className="study-mode-name">{modeName}</span>
        <span className="study-timer">{formatDuration(currentDuration)}</span>
      </div>
    </div>
  );
};

export default StudyIndicator;
