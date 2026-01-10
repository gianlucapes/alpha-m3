import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

// 1. DEFINIZIONE DEI TIPI (Aggiornati per le 100 particelle)
interface Vec2 {
  x: number;
  y: number;
}

// Definiamo la singola particella con colore e raggio
interface Particle {
  pos: Vec2;
  vel: Vec2;
  radius: number;
  color: string;
}

// Definiamo lo stato completo del sistema (la lista)
interface SystemState {
  particles: Particle[];
}

function App() {
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const [started, setStarted] = useState(false);

  // 2. FUNZIONE DI DISEGNO (N-Body)
  const drawSystem = (state: SystemState) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // PULIZIA TOTALE (Per vedere la griglia CSS sotto)
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Disegno di ogni particella nel vettore
    state.particles.forEach((p) => {
      ctx.beginPath();
      ctx.arc(p.pos.x, p.pos.y, p.radius, 0, Math.PI * 2);
      
      // Bordo per contrasto
      ctx.strokeStyle = "rgba(255, 255, 255, 0.8)"; 
      ctx.lineWidth = 1;
      ctx.stroke();

      // Colore pieno
      ctx.fillStyle = p.color;
      ctx.fill();
    });
  };

  // 3. SETUP ASCOLTATORE
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      // Nota: Ascoltiamo <SystemState>, non più ParticleState
      unlisten = await listen<SystemState>("update-physics", (event) => {
        // Chiamiamo la funzione corretta drawSystem
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

  return (
    <div className="container">
      <h1>Alpha M3: Laboratorio Particelle</h1>
      
      <div className="simulation-container">
        <canvas 
          ref={canvasRef} 
          width={500} 
          height={500}
          // Ho rimosso 'background: #111' da qui per far funzionare la griglia nel CSS
          style={{ 
            borderRadius: "4px",
            boxShadow: "0 0 20px rgba(0,0,0,0.5)"
          }}
        />
      </div>

      <div className="controls" style={{ marginTop: "20px" }}>
        <button onClick={handleStart} disabled={started}>
          {started ? "Simulazione Attiva..." : "LANCIA 100 PARTICELLE"}
        </button>
      </div>
    </div>
  );
}

export default App;