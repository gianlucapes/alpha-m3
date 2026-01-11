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

  // MODIFICA 1: Aggiungiamo il parametro 'showStrings'
  const drawSystem = (state: SystemState, showStrings: boolean) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Pulizia e sfondo (grigio scuro "laboratorio")
    ctx.fillStyle = "#1a1a1a"; 
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    state.particles.forEach((p) => {
      
      // --- DISEGNO DEL FILO (Solo se richiesto) ---
      if (showStrings) {
        ctx.beginPath();
        ctx.moveTo(p.pos.x, 0);       // Punto di ancoraggio al soffitto
        ctx.lineTo(p.pos.x, p.pos.y); // Centro della particella
        ctx.strokeStyle = "rgba(100, 100, 100, 0.5)"; // Grigio sottile, semi-trasparente
        ctx.lineWidth = 1;
        // Effetto tratteggiato (opzionale, per stile tecnico)
        ctx.setLineDash([2, 2]); 
        ctx.stroke();
        ctx.setLineDash([]); // Resetta per i cerchi
      }

      // --- DISEGNO DELLA PARTICELLA ---
      ctx.beginPath();
      ctx.arc(p.pos.x, p.pos.y, p.radius, 0, Math.PI * 2);
      
      // Bordo
      ctx.strokeStyle = "rgba(255, 255, 255, 0.9)"; 
      ctx.lineWidth = 2;
      ctx.stroke();

      // Riempimento
      ctx.fillStyle = p.color;
      ctx.fill();
    });
  };

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setup = async () => {
      unlisten = await listen<SystemState>("update-physics", (event) => {
        // MODIFICA 2: Come facciamo a sapere qui se disegnare i fili?
        // Trucco: Se l'evento ha timestamp 0 (o molto basso), è l'init.
        // Ma la soluzione più pulita è usare una 'ref' per leggere lo stato 'started'
        // dentro questo listener asincrono. 
        // Per semplicità qui, useremo il timestamp dell'evento:
        // Se time < 0.1, consideriamole appese.
        const isInitialState = event.payload.timestamp === 0;
        drawSystem(event.payload, isInitialState);
      });

      invoke("init_simulation"); 
    };

    setup();

    return () => { if (unlisten) unlisten(); };
  }, []); // Dipendenze vuote

  
  const handleStart = () => {
    if (!started) {
      invoke("start_simulation");
      setStarted(true);
    }
  };

  const handleReset = async () => {
    await invoke("stop_simulation");
    setStarted(false);

    // INVECE DI PULIRE E BASTA:
    // Richiamiamo l'inizializzazione. 
    // Questo resetterà le posizioni e mostrerà le nuove particelle ferme.
    invoke("init_simulation");
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