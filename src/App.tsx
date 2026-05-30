import { useState } from "react";
import AnimationRenderer from "./components/Animation/AnimationRenderer";
import StatusPanel from "./components/Panels/StatusPanel";
import DialogPanel from "./components/Panels/DialogPanel";
import { usePet } from "./hooks/usePet";

function App() {
  const { petStatus, isLoading, error, feedPet, playWithPet } = usePet();
  const [showDialog, setShowDialog] = useState(false);

  if (isLoading) {
    return <div className="loading">Loading...</div>;
  }

  if (error) {
    return <div className="error">Error: {error}</div>;
  }

  const currentState = petStatus?.state || "idle";

  return (
    <div className="app-container">
      <AnimationRenderer
        state={currentState}
        fallback={
          <div className="pet-emoji">
            {currentState === "happy" ? "😺" : "🐱"}
          </div>
        }
      />
      <div className="pet-controls">
        <button onClick={feedPet} title="Feed">
          🍕
        </button>
        <button onClick={playWithPet} title="Play">
          🎮
        </button>
        <button onClick={() => setShowDialog(true)} title="Talk">
          💬
        </button>
      </div>
      <StatusPanel status={petStatus} />
      {showDialog && (
        <DialogPanel
          onClose={() => setShowDialog(false)}
          petName={petStatus?.name || "Pet"}
        />
      )}
    </div>
  );
}

export default App;