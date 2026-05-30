import AnimationRenderer from "./components/Animation/AnimationRenderer";
import InteractionContainer from "./components/Interaction/InteractionContainer";
import StatusPanel from "./components/Panels/StatusPanel";
import { usePet } from "./hooks/usePet";
import "./styles/interaction.css";

function App() {
  const { petStatus, isLoading, error, feedPet, playWithPet } = usePet();

  if (isLoading) {
    return <div className="loading">Loading...</div>;
  }

  if (error) {
    return <div className="error">Error: {error}</div>;
  }

  const currentState = petStatus?.state || "idle";

  const handleMenuAction = (action: string) => {
    switch (action) {
      case "feed":
        feedPet();
        break;
      case "play":
        playWithPet();
        break;
      case "talk":
        // Dialogue is handled by InteractionContainer
        break;
      case "settings":
        // TODO: Show settings
        break;
      case "exit":
        // TODO: Exit app
        break;
    }
  };

  return (
    <InteractionContainer onMenuAction={handleMenuAction}>
      <div className="app-container">
        <AnimationRenderer
          state={currentState}
          fallback={
            <div className="pet-emoji">
              {currentState === "happy" ? "😺" : "🐱"}
            </div>
          }
        />
        <StatusPanel status={petStatus} />
      </div>
    </InteractionContainer>
  );
}

export default App;