import AnimationRenderer from "./components/Animation/AnimationRenderer";
import InteractionContainer from "./components/Interaction/InteractionContainer";
import MoodIndicator from "./components/Mood/MoodIndicator";
import StatusPanel from "./components/Panels/StatusPanel";
import StudyIndicator from "./components/Study/StudyIndicator";
import { usePet } from "./hooks/usePet";
import { useMood } from "./hooks/useMood";
import { useStudy } from "./hooks/useStudy";
import "./styles/interaction.css";
import "./styles/mood.css";

function App() {
  const { petStatus, isLoading, error, feedPet, playWithPet } = usePet();
  const { snapshot: moodSnapshot } = useMood({
    onStateChange: (newState, oldState) => {
      console.log(`Mood changed: ${oldState} -> ${newState}`);
    },
  });
  const { setMode: setStudyMode } = useStudy();

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
      case "study_mode":
        setStudyMode("study");
        break;
      case "break_mode":
        setStudyMode("break");
        break;
      case "normal_mode":
        setStudyMode("normal");
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
              {moodSnapshot?.emoji || (currentState === "happy" ? "😺" : "🐱")}
            </div>
          }
        />
        <MoodIndicator snapshot={moodSnapshot} />
        <StudyIndicator />
        <StatusPanel status={petStatus} />
      </div>
    </InteractionContainer>
  );
}

export default App;