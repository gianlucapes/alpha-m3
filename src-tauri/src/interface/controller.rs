use tauri::{State, WebviewWindow, Emitter};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use rand::Rng;

use crate::math::vector::Vec2;
use crate::engine::domain::{Particle, SystemState};
use crate::engine::solver;

// Lo stato ora contiene anche le particelle (protette da Mutex)
pub struct SimulationContext {
    pub is_active: Arc<AtomicBool>,
    pub particles: Arc<Mutex<Vec<Particle>>>, // <--- NUOVO: La "Memoria" del sistema
}

// 1. NUOVO COMANDO: Genera le particelle e le mostra, ma NON avvia il tempo
#[tauri::command]
pub fn init_simulation(window: WebviewWindow, state: State<SimulationContext>) {
    // Generiamo le particelle
    let initial_particles = initialize_system(100);

    // 1. Salviamo le particelle nello stato condiviso (nella cassaforte)
    // .lock().unwrap() serve ad aprire la cassaforte
    *state.particles.lock().unwrap() = initial_particles.clone();

    // 2. Le mandiamo subito al frontend per disegnarle ferme
    let state_payload = SystemState { 
        particles: initial_particles,
        timestamp: 0.0 
    };
    window.emit("update-physics", state_payload).unwrap();
}

#[tauri::command]
pub fn stop_simulation(state: State<SimulationContext>) {
    state.is_active.store(false, Ordering::Relaxed);
}

#[tauri::command]
pub fn start_simulation(window: WebviewWindow, state: State<SimulationContext>) {
    // Evita doppi avvii
    if state.is_active.load(Ordering::Relaxed) { return; }
    
    state.is_active.store(true, Ordering::Relaxed);
    let is_active = state.is_active.clone();

    // PRENDIAMO LE PARTICELLE DALLO STATO (CLONE)
    // Invece di crearne di nuove, prendiamo quelle che l'utente vede a schermo
    let mut particles = state.particles.lock().unwrap().clone();

    thread::spawn(move || {
        let mut timer: f64 = 0.0;

        loop {
            if !is_active.load(Ordering::Relaxed) { break; }

            // Calcolo Fisica
            solver::solve_step(&mut particles);
            timer += 0.016;

            // Invio Dati
            let state_payload = SystemState { 
                particles: particles.clone(),
                timestamp: timer 
            };

            if window.emit("update-physics", state_payload).is_err() {
                break; 
            }

            thread::sleep(Duration::from_millis(16));
        }
    });
}

// Helper per generare dati random (Configurazione "Appesa")
fn initialize_system(count: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    let mut list = Vec::new();
    
    for _ in 0..count {
        let radius = rng.gen_range(3.0..8.0); // Un po' più grandi per vederle bene
        
        // X: Sparpagliate su tutta la larghezza (con un po' di margine dai bordi)
        let x = rng.gen_range(20.0..480.0);
        
        // Y: Variano in altezza (lunghezza del filo diversa)
        // Le mettiamo tra 50px e 250px dal soffitto
        let y = rng.gen_range(50.0..250.0);

        list.push(Particle::new(
            Vec2::new(x, y),
            Vec2::new(0.0, 0.0), // VELOCITÀ ZERO: Sono ferme appese!
            radius,
            format!("#{:02X}{:02X}FF", rng.gen_range(100..255), rng.gen_range(100..255)),
        ));
    }
    list
}