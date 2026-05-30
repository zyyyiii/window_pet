import React from 'react';
import { useGrowth, formatExp, getLevelTitle } from '../../../hooks/useGrowth';
import './GrowthPanel.css';

interface GrowthPanelProps {
  onClose: () => void;
}

export const GrowthPanel: React.FC<GrowthPanelProps> = ({ onClose }) => {
  const { snapshot, loading, error, recordInteraction } = useGrowth();

  if (loading) {
    return (
      <div className="growth-panel">
        <div className="growth-loading">加载中...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="growth-panel">
        <div className="growth-error">加载失败: {error}</div>
      </div>
    );
  }

  if (!snapshot) {
    return null;
  }

  const { level_info, companion_days, learning_points, total_interactions, memory_count } = snapshot;

  return (
    <div className="growth-panel">
      <div className="growth-header">
        <h2>成长档案</h2>
        <button className="close-btn" onClick={onClose}>×</button>
      </div>

      <div className="growth-content">
        {/* 等级区域 */}
        <div className="level-section">
          <div className="level-badge">
            <span className="level-number">Lv.{level_info.current_level}</span>
            <span className="level-title">{getLevelTitle(level_info.current_level)}</span>
          </div>
          <div className="exp-bar-container">
            <div className="exp-bar" style={{ width: `${level_info.progress_percent}%` }} />
            <span className="exp-text">
              {formatExp(level_info.current_exp)} / {formatExp(level_info.exp_to_next_level)}
            </span>
          </div>
        </div>

        {/* 统计信息 */}
        <div className="stats-section">
          <div className="stat-item">
            <span className="stat-icon">📅</span>
            <span className="stat-label">陪伴天数</span>
            <span className="stat-value">{companion_days} 天</span>
          </div>
          <div className="stat-item">
            <span className="stat-icon">⭐</span>
            <span className="stat-label">学习积分</span>
            <span className="stat-value">{learning_points}</span>
          </div>
          <div className="stat-item">
            <span className="stat-icon">🤝</span>
            <span className="stat-label">互动次数</span>
            <span className="stat-value">{total_interactions}</span>
          </div>
          <div className="stat-item">
            <span className="stat-icon">💭</span>
            <span className="stat-label">记忆数量</span>
            <span className="stat-value">{memory_count}</span>
          </div>
        </div>

        {/* 快捷互动 */}
        <div className="interaction-section">
          <h3>互动</h3>
          <div className="interaction-buttons">
            <button onClick={() => recordInteraction('Feed')}>喂食 +10</button>
            <button onClick={() => recordInteraction('Play')}>玩耍 +15</button>
            <button onClick={() => recordInteraction('Talk')}>对话 +5</button>
            <button onClick={() => recordInteraction('Touch')}>摸摸 +2</button>
          </div>
        </div>
      </div>
    </div>
  );
};
