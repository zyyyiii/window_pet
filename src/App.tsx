import { useState } from "react";
import PetContainer from "./components/Pet/PetContainer";
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

  return (
    <div className="app-container">
      <PetContainer 
        status={petStatus} 
        onFeed={feedPet}
        onPlay={playWithPet}
        onTalk={() => setShowDialog(true)}
      />
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