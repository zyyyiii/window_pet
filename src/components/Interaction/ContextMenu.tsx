import React, { useEffect, useRef } from "react";
import type { MenuItem } from "../../types/events";

interface ContextMenuProps {
  items: MenuItem[];
  position: { x: number; y: number } | null;
  onAction: (action: string) => void;
  onClose: () => void;
  className?: string;
}

const ContextMenu: React.FC<ContextMenuProps> = ({
  items,
  position,
  onAction,
  onClose,
  className = "",
}) => {
  const menuRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        onClose();
      }
    };

    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleEscape);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    };
  }, [onClose]);

  if (!position) return null;

  const style: React.CSSProperties = {
    position: "fixed",
    left: position.x,
    top: position.y,
    zIndex: 1000,
  };

  return (
    <div
      ref={menuRef}
      className={`context-menu ${className}`}
      style={style}
    >
      {items.map((item) => (
        <button
          key={item.id}
          className={`context-menu-item ${item.disabled ? "disabled" : ""}`}
          onClick={() => {
            if (!item.disabled) {
              onAction(item.id);
              onClose();
            }
          }}
          disabled={item.disabled}
        >
          {item.icon && <span className="context-menu-icon">{item.icon}</span>}
          <span className="context-menu-label">{item.label}</span>
        </button>
      ))}
    </div>
  );
};

export default ContextMenu;