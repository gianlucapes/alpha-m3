import React, { RefObject } from "react";

interface Props {
    canvasRef: RefObject<HTMLCanvasElement>;
    width: number;
    height: number;
}

export const SimulationCanvas: React.FC<Props> = ({ canvasRef, width, height }) => {
    return (
        <div className="simulation-container" style={{ display: 'flex', justifyContent: 'center' }}>
            <canvas
                ref={canvasRef}
                width={width}
                height={height}
                style={{
                    borderRadius: "4px",
                    boxShadow: "0 0 20px rgba(0, 0, 0, 0.58)",
                    backgroundColor: "#1a1a1a",
                    cursor: "crosshair"
                }}
            />
        </div>
    );
};