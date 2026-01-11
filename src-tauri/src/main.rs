use std::sync::{Arc, Mutex, atomic::AtomicBool}; // Aggiungi Mutex qui
mod math;
mod engine;
mod interface;

use interface::controller;
use interface::controller::SimulationContext;

fn main() {
    // Inizializzazione dello stato globale
    let is_active = Arc::new(AtomicBool::new(false));
    let particles = Arc::new(Mutex::new(Vec::new())); // <--- NUOVO: Vettore vuoto protetto

    tauri::Builder::default()
        .manage(SimulationContext { is_active, particles }) // Passiamo entrambi
        .invoke_handler(tauri::generate_handler![
            controller::init_simulation,   // <--- NUOVO COMANDO
            controller::start_simulation, 
            controller::stop_simulation
        ])
        .run(tauri::generate_context!())
        .expect("Errore critico durante l'esecuzione dell'applicazione Tauri");
}