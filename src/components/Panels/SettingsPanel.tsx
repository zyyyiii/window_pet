import React from "react";
import Panel from "../UI/Panel";
import { useConfigStore } from "../../stores/configStore";

interface SettingsPanelProps {
  onClose: () => void;
}

const SettingsPanel: React.FC<SettingsPanelProps> = ({ onClose }) => {
  const { config, setConfig, resetConfig } = useConfigStore();

  return (
    <Panel title="Settings" onClose={onClose} className="settings-panel">
      <div className="settings-group">
        <label>
          Pet Name:
          <input
            type="text"
            value={config.petName}
            onChange={(e) => setConfig({ petName: e.target.value })}
          />
        </label>
      </div>
      <div className="settings-group">
        <label>
          <input
            type="checkbox"
            checked={config.autoStart}
            onChange={(e) => setConfig({ autoStart: e.target.checked })}
          />
          Auto Start
        </label>
      </div>
      <div className="settings-group">
        <label>
          <input
            type="checkbox"
            checked={config.alwaysOnTop}
            onChange={(e) => setConfig({ alwaysOnTop: e.target.checked })}
          />
          Always on Top
        </label>
      </div>
      <div className="settings-group">
        <label>
          Transparency: {Math.round(config.transparency * 100)}%
          <input
            type="range"
            min="0"
            max="1"
            step="0.1"
            value={config.transparency}
            onChange={(e) => setConfig({ transparency: parseFloat(e.target.value) })}
          />
        </label>
      </div>
      <div className="settings-group">
        <label>
          Animation Speed: {config.animationSpeed}x
          <input
            type="range"
            min="0.5"
            max="2"
            step="0.1"
            value={config.animationSpeed}
            onChange={(e) => setConfig({ animationSpeed: parseFloat(e.target.value) })}
          />
        </label>
      </div>
      <div className="settings-group">
        <label>
          <input
            type="checkbox"
            checked={config.enableAi}
            onChange={(e) => setConfig({ enableAi: e.target.checked })}
          />
          Enable AI Behavior
        </label>
      </div>
      <div className="settings-group">
        <label>
          <input
            type="checkbox"
            checked={config.enableMonitoring}
            onChange={(e) => setConfig({ enableMonitoring: e.target.checked })}
          />
          Enable System Monitoring
        </label>
      </div>
      <div className="settings-group">
        <label>
          <input
            type="checkbox"
            checked={config.enableReminders}
            onChange={(e) => setConfig({ enableReminders: e.target.checked })}
          />
          Enable Reminders
        </label>
      </div>
      <div className="settings-actions">
        <button onClick={resetConfig}>Reset to Default</button>
      </div>
    </Panel>
  );
};

export default SettingsPanel;