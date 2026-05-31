import { useEffect, useRef } from "react";
import AnimationRenderer from "./components/Animation/AnimationRenderer";
import InteractionContainer from "./components/Interaction/InteractionContainer";
import MoodIndicator from "./components/Mood/MoodIndicator";
import StatusPanel from "./components/Panels/StatusPanel";
import StudyIndicator from "./components/Study/StudyIndicator";
import { usePet } from "./hooks/usePet";
import { useMood } from "./hooks/useMood";
import { useStudy } from "./hooks/useStudy";
import { useActivity, ActivityState } from "./hooks/useActivity";
import "./styles/interaction.css";
import "./styles/mood.css";

function App() {
  const { petStatus, isLoading, error, feedPet, playWithPet } = usePet();
  const { snapshot: moodSnapshot } = useMood({
    onStateChange: (newState, oldState) => {
      console.log(`Mood changed: ${oldState} -> ${newState}`);
    },
  });
  const { snapshot: studySnapshot, setMode: setStudyMode } = useStudy();
  const { analysis: activityAnalysis } = useActivity(10000);

  // 自动同步学习模式（基于活动检测）
  const lastAutoState = useRef<ActivityState | null>(null);

  useEffect(() => {
    if (!activityAnalysis) return;

    const activityState = activityAnalysis.state;

    // 只在状态变化时切换模式
    if (activityState === lastAutoState.current) return;
    lastAutoState.current = activityState;

    // 根据活动状态自动切换学习模式
    switch (activityState) {
      case "studying":
      case "coding":
        // 学习或编程时自动进入学习模式
        if (studySnapshot?.mode !== "study") {
          setStudyMode("study");
          console.log(`活动检测: 检测到 ${activityState}，自动切换到学习模式`);
        }
        break;
      case "idle":
        // 空闲时自动进入休息模式
        if (studySnapshot?.mode === "study") {
          setStudyMode("break");
          console.log("活动检测: 检测到空闲，自动切换到休息模式");
        }
        break;
      case "entertainment":
        // 娱乐时自动恢复普通模式
        if (studySnapshot?.mode === "study") {
          setStudyMode("normal");
          console.log("活动检测: 检测到娱乐，自动切换到普通模式");
        }
        break;
      // unknown 保持当前模式
    }
  }, [activityAnalysis, studySnapshot?.mode, setStudyMode]);

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