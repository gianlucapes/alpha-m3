// src-tauri/src/main.rs

// Dichiariamo che esistono queste cartelle (moduli)
mod math;
mod engine;
mod interface;

use tauri::Manager;
use std::sync::{Arc, atomic::AtomicBool};

// Nota come il percorso si è allungato: interface -> controller
use interface::controller::{start_simulation, stop_simulation, SimulationContext};

use crate::interface::controller;

fn main() {
    // Stato condiviso inizializzato a 'false' (simulazione ferma)
    let is_active = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .manage(SimulationContext { is_active })
        .invoke_handler(tauri::generate_handler![
            controller::start_simulation, 
            controller::stop_simulation
        ])
        .run(tauri::generate_context!())
        .expect("Errore critico durante l'esecuzione dell'applicazione Tauri");
}