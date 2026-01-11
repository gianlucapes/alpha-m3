// src/types/physics.ts

export interface Vec2 {
    x: number;
    y: number;
}
  
export interface Particle {
    pos: Vec2;
    vel: Vec2;
    radius: number;
    color: string;
    mass: number; // Aggiunto per coerenza col nuovo backend
}
  
export interface SystemState {
    particles: Particle[];
    timestamp: number;
}