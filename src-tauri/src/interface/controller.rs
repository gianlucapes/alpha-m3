use tauri::{State, WebviewWindow, Emitter};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use rand::Rng;

use crate::{engine::domain::{Particle, SystemState}, math::vector::Vec2};
use crate::engine::solver; // Importiamo il modulo solver

// Flag atomico per il controllo thread-safe
pub struct SimulationContext {
    pub is_active: Arc<AtomicBool>,
}

#[tauri::command]
pub fn stop_simulation(state: State<SimulationContext>) {
    state.is_active.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn start_simulation(window: WebviewWindow, state: State<SimulationContext>) {
    // Evita di lanciare doppi thread
    if state.is_active.load(Ordering::Relaxed) { return; }
    
    state.is_active.store(true, Ordering::Relaxed);
    let is_active = state.is_active.clone();

    thread::spawn(move || {
        // Configurazione Iniziale del Sistema
        let mut particles = initialize_system(100);
        let mut timer: f64 = 0.0;

        // --- CICLO DI SIMULAZIONE ---
        loop {
            // 1. Check condizione di arresto
            if !is_active.load(Ordering::Relaxed) { break; }

            // 2. Calcolo dello step fisico
            solver::solve_step(&mut particles);
            timer += 0.016; // Avanzamento temporale (dt fittizio)

            // 3. Serializzazione e Invio Dati (I/O)
            let state_payload = SystemState { 
                particles: particles.clone(),
                timestamp: timer 
            };

            if window.emit("update-physics", state_payload).is_err() {
                // Se la GUI è chiusa, interrompiamo il calcolo
                break; 
            }

            // 4. Sincronizzazione Temporale (Target ~60Hz)
            thread::sleep(Duration::from_millis(16));
        }
    });
}

// Generatore di Condizioni Iniziali (Monte Carlo base)
fn initialize_system(count: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut list = Vec::new();
    
    for _ in 0..count {
        let radius = rng.gen_range(2.0..6.0);
        list.push(Particle::new(
            Vec2::new(rng.gen_range(50.0..450.0), rng.gen_range(50.0..200.0)),
            Vec2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..0.0)),
            radius,
            format!("#{:02X}{:02X}FF", rng.gen_range(100..255), rng.gen_range(100..255)),
        ));
    }
    list
}