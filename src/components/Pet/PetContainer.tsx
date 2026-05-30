import React from "react";
import PetSprite from "./PetSprite";
import PetAnimation from "./PetAnimation";
import { PetStatus } from "../../types/pet";

interface PetContainerProps {
  status: PetStatus | null;
  onFeed: () => void;
  onPlay: () => void;
  onTalk: () => void;
}

const PetContainer: React.FC<PetContainerProps> = ({
  status,
  onFeed,
  onPlay,
  onTalk,
}) => {
  if (!status) {
    return <div className="pet-container">Loading pet...</div>;
  }

  return (
    <div className="pet-container">
      <PetAnimation state={status.state}>
        <PetSprite state={status.state} name={status.name} />
      </PetAnimation>
      <div className="pet-controls">
        <button onClick={onFeed} title="Feed">
          🍕
        </button>
        <button onClick={onPlay} title="Play">
          🎮
        </button>
        <button onClick={onTalk} title="Talk">
          💬
        </button>
      </div>
    </div>
  );
};

export default PetContainer;