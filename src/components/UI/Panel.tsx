import React from "react";

interface PanelProps {
  children: React.ReactNode;
  title?: string;
  onClose?: () => void;
  className?: string;
}

const Panel: React.FC<PanelProps> = ({
  children,
  title,
  onClose,
  className = "",
}) => {
  return (
    <div className={`panel ${className}`}>
      {title && (
        <div className="panel-header">
          <h3 className="panel-title">{title}</h3>
          {onClose && (
            <button className="panel-close" onClick={onClose}>
              ×
            </button>
          )}
        </div>
      )}
      <div className="panel-content">{children}</div>
    </div>
  );
};

export default Panel;