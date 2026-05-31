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
  console.log("App 组件开始渲染");

  const { petStatus, isLoading, error, feedPet, playWithPet } = usePet();
  const { snapshot: moodSnapshot } = useMood({
    onStateChange: (newState, oldState) => {
      console.log(`Mood changed: ${oldState} -> ${newState}`);
    },
  });
  const { snapshot: studySnapshot, setMode: setStudyMode } = useStudy();
  const { analysis: activityAnalysis } = useActivity(10000);

  console.log("App 状态:", { isLoading, error, petStatus });

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

  // 始终显示的基本内容，用于调试
  const debugContent = (
    <div style={{
      position: 'absolute',
      top: 10,
      right: 10,
      background: 'rgba(0,0,0,0.7)',
      color: 'white',
      padding: '4px 8px',
      borderRadius: 4,
      fontSize: 11,
      zIndex: 9999,
      maxWidth: 200,
      wordBreak: 'break-all'
    }}>
      调试: {isLoading ? '加载中' : error ? `错误: ${error}` : '正常'}
    </div>
  );

  if (isLoading) {
    return (
      <div className="loading" style={{ background: 'rgba(255,255,255,0.95)' }}>
        {debugContent}
        <div style={{ fontSize: 24, marginBottom: 10 }}>😺</div>
        <div>Loading...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="error" style={{ background: 'rgba(255,255,255,0.95)' }}>
        {debugContent}
        <div style={{ fontSize: 24, marginBottom: 10 }}>😿</div>
        <div>Error: {error}</div>
      </div>
    );
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
      <div className="app-container" data-tauri-drag-region>
        {/* 调试信息 */}
        <div style={{
          position: 'absolute',
          top: 5,
          left: 5,
          fontSize: 10,
          color: '#666',
          background: 'rgba(255,255,255,0.8)',
          padding: '2px 4px',
          borderRadius: 4,
          zIndex: 1000
        }}>
          状态: {currentState} | 加载: {isLoading ? '是' : '否'}
        </div>

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