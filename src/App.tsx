import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

interface Vec2 {
  x: number;
  y: number;
}

interface Particle {
  pos: Vec2;
  vel: Vec2;
  radius: number;
  color: string;
}

interface SystemState {
  particles: Particle[];
}

function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [started, setStarted] = useState(false);

  const drawSystem = (state: SystemState) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    state.particles.forEach((p) => {
      ctx.beginPath();
      ctx.arc(p.pos.x, p.pos.y, p.radius, 0, Math.PI * 2);
      ctx.strokeStyle = "rgba(255, 255, 255, 0.8)"; 
      ctx.lineWidth = 1;
      ctx.stroke();
      ctx.fillStyle = p.color;
      ctx.fill();
    });
  };

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<SystemState>("update-physics", (event) => {
        drawSystem(event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const handleStart = () => {
    if (!started) {
      invoke("start_simulation");
      setStarted(true);
    }
  };


  // ==========================================
  // 5. NUOVA FUNZIONE DI RESET
  // ==========================================
  // Questa funzione chiama il comando Rust che alza la bandierina "should_reset"
  const handleReset = async () => {
    // 1. Diciamo a Rust di uccidere il thread
    await invoke("stop_simulation");
    
    // 2. Aggiorniamo lo stato locale per riabilitare il pulsante "Start"
    setStarted(false);

    // 3. Pulizia Visiva Immediata
    // Anche se il thread si ferma, l'ultimo disegno rimane sul canvas.
    // Dobbiamo cancellarlo manualmente per far capire all'utente che è "vuoto".
    const canvas = canvasRef.current;
    const ctx = canvas?.getContext("2d");
    if (canvas && ctx) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
    }
  };

  return (
    <div className="container">
      <h1>Alpha M3</h1>
      
      <div className="simulation-container">
        <canvas 
          ref={canvasRef} 
          width={500} 
          height={500}
          style={{ 
            borderRadius: "4px",
            boxShadow: "0 0 20px rgba(0, 0, 0, 0.58)",
            backgroundColor: "#1a1a1a"
          }}
        />
      </div>

      <div className="controls" style={{ marginTop: "20px", display: "flex", gap: "10px", justifyContent: "center" }}>
        
        {/* Pulsante START: Abilitato solo se NON è started */}
        <button onClick={handleStart} disabled={started}>
          {started ? "SIMULAZIONE IN CORSO..." : "LANCIA 100 PARTICELLE"}
        </button>

        {/* Pulsante RESET/STOP: Abilitato solo se è started */}
        <button 
          onClick={handleReset} 
          disabled={!started}
          style={{
            backgroundColor: !started ? "#555" : "#ff4444",
            color: "white",
            cursor: !started ? "not-allowed" : "pointer",
            border: "none",
            borderRadius: "8px",
            padding: "0.6em 1.2em"
          }}
        >
          STOP & RESET ⏹
        </button>

      </div>
    </div>
  );
}

export default App;