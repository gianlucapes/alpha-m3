import "./App.css";
import { useSimulation } from "./hooks/useSimulation";
import { SimulationCanvas } from "./components/SimulationCanvas";
import { Controls } from "./components/Controls";

function App() {
  // 1. Usiamo l'hook per ottenere logica e riferimenti
  const { canvasRef, started, start, reset } = useSimulation();

  return (
    <div className="container">
      <h1>Alpha M3: Laboratorio</h1>
      
      {/* 2. Componente Visuale */}
      <SimulationCanvas 
        canvasRef={canvasRef} 
        width={500} 
        height={500} 
      />

      {/* 3. Componente Controlli */}
      <Controls 
        started={started} 
        onStart={start} 
        onReset={reset} 
      />
    </div>
  );
}

export default App;