// src/hooks/useSimulation.ts
import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { SystemState } from "../types/physics";
import { Renderer } from "../lib/renderer";

export function useSimulation() {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);
    const [started, setStarted] = useState(false);

    // Gestione Eventi da Rust
    useEffect(() => {
        let unlisten: (() => void) | undefined;

        const setupListener = async () => {
            unlisten = await listen<SystemState>("update-physics", (event) => {
                const canvas = canvasRef.current;
                if (!canvas) return;

                // Logica: Disegna i fili solo se siamo al tempo 0
                const isInitialState = event.payload.timestamp === 0;
                
                // Chiamiamo il nostro renderer puro
                Renderer.drawFrame(canvas, event.payload, isInitialState);
            });

            // Init iniziale
            invoke("init_simulation");
        };

        setupListener();

        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    // Azioni Utente
    const start = async () => {
        if (!started) {
            await invoke("start_simulation");
            setStarted(true);
        }
    };

    const reset = async () => {
        await invoke("stop_simulation");
        setStarted(false);
        // Richiediamo il nuovo stato iniziale (fili appesi)
        await invoke("init_simulation");
    };

    return {
        canvasRef,
        started,
        start,
        reset
    };
}