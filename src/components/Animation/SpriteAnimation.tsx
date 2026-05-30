import React, { useEffect, useRef, useState } from "react";
import type { AnimationUpdate, SpriteSheetConfig } from "../../types/animation";

interface SpriteAnimationProps {
  config: SpriteSheetConfig;
  animationUpdate: AnimationUpdate | null;
  className?: string;
}

const SpriteAnimation: React.FC<SpriteAnimationProps> = ({
  config,
  animationUpdate,
  className = "",
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const imageRef = useRef<HTMLImageElement | null>(null);
  const [imageLoaded, setImageLoaded] = useState(false);

  useEffect(() => {
    const img = new Image();
    img.onload = () => {
      imageRef.current = img;
      setImageLoaded(true);
    };
    img.onerror = () => {
      console.error(`Failed to load sprite image: ${config.image}`);
    };
    img.src = `/animations/${config.image}`;
  }, [config.image]);

  useEffect(() => {
    if (!canvasRef.current || !imageRef.current || !animationUpdate || !imageLoaded) {
      return;
    }

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    canvas.width = config.frame_width;
    canvas.height = config.frame_height;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    ctx.drawImage(
      imageRef.current,
      animationUpdate.source_x,
      animationUpdate.source_y,
      animationUpdate.source_width,
      animationUpdate.source_height,
      0,
      0,
      config.frame_width,
      config.frame_height
    );
  }, [animationUpdate, config, imageLoaded]);

  return (
    <canvas
      ref={canvasRef}
      className={`sprite-animation ${className}`}
      style={{
        width: config.frame_width,
        height: config.frame_height,
        imageRendering: "pixelated",
      }}
    />
  );
};

export default SpriteAnimation;