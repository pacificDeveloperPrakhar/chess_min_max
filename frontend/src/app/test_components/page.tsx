"use client";
import { motion, useDragControls } from "motion/react";
import { useState, useRef } from "react";

export default function Draggable() {
  const controls = useDragControls();
  const [isInTarget, setIsInTarget] = useState(false);
  const targetRef = useRef(null);

  // Function to detect collision
  const checkCollision = (event) => {
    const chessPiece = event.target.getBoundingClientRect();
    const targetBox = targetRef.current.getBoundingClientRect();

    // Collision detection logic
    const isOverlapping =
      chessPiece.left < targetBox.right &&
      chessPiece.right > targetBox.left &&
      chessPiece.top < targetBox.bottom &&
      chessPiece.bottom > targetBox.top;

    if (isOverlapping) {
      setIsInTarget(true); // Update state
    } else {
      setIsInTarget(false); // Reset state
    }
  };

  return (
    <div style={{ display: "flex", gap: "2rem", padding: "2rem" }}>
      {/* Draggable Chess Piece */}
      <motion.div
        drag
        dragControls={controls}
        dragElastic={0.2}
        onDrag={checkCollision} // Detect collision during drag
        style={{
          width: "100px",
          height: "100px",
          backgroundColor: isInTarget ? "green" : "orange",
          borderRadius: "10px",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          cursor: "grab",
        }}
      >
        Chess Piece
      </motion.div>

      {/* Target Container */}
      <div
        ref={targetRef}
        style={{
          width: "150px",
          height: "150px",
          backgroundColor: "purple",
          borderRadius: "10px",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          color: "#fff",
          border: `3px solid ${isInTarget ? "yellow" : "transparent"}`,
        }}
      >
        Drop Zone
      </div>
    </div>
  );
}
