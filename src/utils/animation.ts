export type AnimationType =
  | "idle"
  | "happy"
  | "hungry"
  | "sleepy"
  | "playing"
  | "studying"
  | "monitoring"
  | "talking";

export const getAnimationDuration = (type: AnimationType): number => {
  switch (type) {
    case "idle":
      return 2000;
    case "happy":
      return 500;
    case "hungry":
      return 1000;
    case "sleepy":
      return 2000;
    case "playing":
      return 300;
    case "studying":
      return 1500;
    case "monitoring":
      return 1000;
    case "talking":
      return 500;
    default:
      return 1000;
  }
};

export const getAnimationClass = (type: AnimationType): string => {
  return `animate-${type}`;
};