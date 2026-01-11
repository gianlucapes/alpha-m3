// src/lib/renderer.ts
import { SystemState } from "../types/physics";

export class Renderer {
    // Funzione statica per pulire e disegnare un frame
    static drawFrame(
        canvas: HTMLCanvasElement, 
        state: SystemState, 
        showStrings: boolean
    ) {
        const ctx = canvas.getContext("2d");
        if (!ctx) return;

        // 1. Clear Screen
        ctx.fillStyle = "#1a1a1a";
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // 2. Disegno Particelle
        state.particles.forEach((p) => {
            
            // A. Filo (Se richiesto)
            if (showStrings) {
                ctx.beginPath();
                ctx.moveTo(p.pos.x, 0);
                ctx.lineTo(p.pos.x, p.pos.y);
                ctx.strokeStyle = "rgba(100, 100, 100, 0.5)";
                ctx.lineWidth = 1;
                ctx.setLineDash([2, 2]); 
                ctx.stroke();
                ctx.setLineDash([]);
            }

            // B. Corpo Particella
            ctx.beginPath();
            ctx.arc(p.pos.x, p.pos.y, p.radius, 0, Math.PI * 2);
            
            ctx.strokeStyle = "rgba(255, 255, 255, 0.8)";
            ctx.lineWidth = 1.5;
            ctx.stroke();

            ctx.fillStyle = p.color;
            ctx.fill();
        });

        // Opzionale: Debug info (FPS o numero particelle)
        /*
        ctx.fillStyle = "white";
        ctx.font = "12px monospace";
        ctx.fillText(`Time: ${state.timestamp.toFixed(2)}s`, 10, 20);
        */
    }
}