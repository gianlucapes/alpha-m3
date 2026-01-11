use serde::Serialize;
use crate::math::vector::Vec2;
/// Rappresenta una massa puntiforme con volume spaziale.
#[derive(Clone, Serialize)]
pub struct Particle {
    pub pos: Vec2,      // Posizione (m)
    pub vel: Vec2,      // Velocità (m/s)
    pub radius: f64,    // Raggio (m) - Approssimazione sferica
    pub mass: f64,      // Massa (kg) - Importante per la conservazione della q.d.m.
    pub color: String,  // Metadato per la visualizzazione
}

impl Particle {
    pub fn new(pos: Vec2, vel: Vec2, radius: f64, color: String) -> Self {
        // Per ora assumiamo densità costante: Massa proporzionale all'area (R^2)
        let mass = radius * radius; 
        Self { pos, vel, radius, mass, color }
    }
}

/// Snapshot dello stato dell'intero sistema fisico.
#[derive(Clone, Serialize)]
pub struct SystemState {
    pub particles: Vec<Particle>,
    pub timestamp: f64, // Tempo trascorso (utile per grafici futuri)
}