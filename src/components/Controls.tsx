import React from "react";

interface Props {
    started: boolean;
    onStart: () => void;
    onReset: () => void;
}

export const Controls: React.FC<Props> = ({ started, onStart, onReset }) => {
    return (
        <div className="controls" style={{ marginTop: "20px", display: "flex", gap: "10px", justifyContent: "center" }}>
            <button 
                onClick={onStart} 
                disabled={started}
                style={{ padding: "10px 20px", cursor: started ? "default" : "pointer" }}
            >
                {started ? "SIMULAZIONE IN CORSO..." : "LANCIA ESPERIMENTO"}
            </button>

            <button
                onClick={onReset}
                disabled={!started}
                style={{
                    backgroundColor: !started ? "#555" : "#ff4444",
                    color: "white",
                    cursor: !started ? "not-allowed" : "pointer",
                    border: "none",
                    borderRadius: "8px",
                    padding: "10px 20px"
                }}
            >
                STOP & RESET ⏹
            </button>
        </div>
    );
};