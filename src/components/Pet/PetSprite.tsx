import React from "react";
import { PetState } from "../../types/pet";

interface PetSpriteProps {
  state: PetState;
  name: string;
}

const PetSprite: React.FC<PetSpriteProps> = ({ state, name }) => {
  const getEmoji = (state: PetState): string => {
    switch (state) {
      case "idle":
        return "🐱";
      case "happy":
        return "😺";
      case "hungry":
        return "😿";
      case "sleepy":
        return "😴";
      case "playing":
        return "😸";
      case "studying":
        return "📚";
      case "monitoring":
        return "👀";
      case "talking":
        return "😼";
      default:
        return "🐱";
    }
  };

  return (
    <div className="pet-sprite">
      <div className="pet-emoji">{getEmoji(state)}</div>
      <div className="pet-name">{name}</div>
    </div>
  );
};

export default PetSprite;