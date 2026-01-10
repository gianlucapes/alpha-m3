// src-tauri/src/vector.rs

// Importiamo le macro per la serializzazione (trasformare dati in JSON)
// e i tratti per la matematica (Addizione, Moltiplicazione).
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul};

/// Rappresenta un vettore geometrico in uno spazio 2D.
///
/// Questa struct è la base di tutta la simulazione fisica.
/// Viene usata sia per la Posizione (x, y) che per la Velocità (vx, vy)
/// e l'Accelerazione (ax, ay).
///
/// # Attributi
/// * `derive(Serialize, Deserialize)`: Permette a Rust di convertire automaticamente
///   questa struttura in un oggetto JSON { "x": ..., "y": ... } leggibile da JavaScript.
/// * `derive(Clone, Copy)`: Indica che questo oggetto è leggero e può essere copiato
///   in memoria velocemente invece di essere spostato (fondamentale per le performance).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f64, // Coordinata X (virgola mobile a 64 bit per precisione)
    pub y: f64, // Coordinata Y
}

impl Vec2 {
    /// Costruttore: crea un nuovo Vettore 2D.
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

// --- IMPLEMENTAZIONE OPERATORI MATEMATICI ---
// Rust non sa come sommare due struct personalizzate.
// Qui sotto gli "insegniamo" come usare il simbolo '+' e '*' con i nostri vettori.

/// Implementa l'operatore `+` (Somma vettoriale).
/// Formula: V3 = (x1+x2, y1+y2)
impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implementa l'operatore `+=` (Somma e assegna).
/// Permette di scrivere `posizione += velocità` invece di `posizione = posizione + velocità`.
/// È zucchero sintattico che rende il codice fisico molto più leggibile.
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

/// Implementa l'operatore `*` (Moltiplicazione per scalare).
/// Utile per attrito o inversione (es. vettore * -1.0).
/// Formula: V2 = (x * k, y * k)
impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}