import React from "react";
import { PetState } from "../../types/pet";

interface PetAnimationProps {
  state: PetState;
  children: React.ReactNode;
}

const PetAnimation: React.FC<PetAnimationProps> = ({ state, children }) => {
  const getAnimationClass = (state: PetState): string => {
    switch (state) {
      case "idle":
        return "animate-idle";
      case "happy":
        return "animate-happy";
      case "hungry":
        return "animate-hungry";
      case "sleepy":
        return "animate-sleepy";
      case "playing":
        return "animate-playing";
      case "studying":
        return "animate-studying";
      case "monitoring":
        return "animate-monitoring";
      case "talking":
        return "animate-talking";
      default:
        return "animate-idle";
    }
  };

  return (
    <div className={`pet-animation ${getAnimationClass(state)}`}>
      {children}
    </div>
  );
};

export default PetAnimation;