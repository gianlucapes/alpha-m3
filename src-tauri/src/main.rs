// src-tauri/src/main.rs

// Disabilita la console di comando su Windows quando l'app è in release
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// --- MODULI ---
// Dichiariamo il modulo 'vector' definito nel file vector.rs
mod vector;
use vector::Vec2;

// --- IMPORT ---
// Emitter: Fondamentale in Tauri v2 per inviare eventi (emit) al frontend.
// Window: Rappresenta la finestra dell'applicazione.
use tauri::{Emitter, Window};
use std::{thread, time};
use serde::Serialize;
use rand::Rng; // Libreria per la generazione di numeri casuali

// --- STRUTTURE DATI ---

/// Rappresenta una singola entità fisica nel sistema.
#[derive(Clone, Serialize)]
struct Particle {
    pos: Vec2,      // Posizione attuale (vettore)
    vel: Vec2,      // Velocità attuale (vettore)
    radius: f64,    // Raggio visivo (per le collisioni future)
    color: String,  // Colore esadecimale per il rendering
}

/// Rappresenta l'istantanea ("Snapshot") dell'intero sistema fisico.
/// Questo è l'oggetto che viene serializzato in JSON e spedito a React
/// 60 volte al secondo.
#[derive(Clone, Serialize)]
struct SystemState {
    particles: Vec<Particle>,
}

// --- COMANDI TAURI ---

/// Funzione invocabile dal Frontend per avviare la simulazione.
///
/// NOTA ARCHITETTURALE:
/// La simulazione viene lanciata in un `thread::spawn` separato.
/// Se facessimo girare il loop `while` o `loop` nel thread principale,
/// l'interfaccia grafica (GUI) si congelerebbe perché il processore
/// sarebbe impegnato al 100% nei calcoli e non potrebbe disegnare la finestra.
#[tauri::command]
fn start_simulation(window: Window) {
    // Spawna un nuovo thread (processo leggero) per la fisica
    std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let mut particles: Vec<Particle> = Vec::new();

        // 1. FASE DI INIZIALIZZAZIONE (Big Bang)
        // Creiamo 100 particelle con stati iniziali casuali
        for _ in 0..100 {
            particles.push(Particle {
                pos: Vec2::new(
                    rng.gen_range(50.0..450.0), // Posizione X casuale
                    rng.gen_range(50.0..200.0)  // Posizione Y casuale
                ),
                vel: Vec2::new(
                    rng.gen_range(-2.0..2.0),   // Velocità orizzontale
                    rng.gen_range(-2.0..0.0)    // Velocità verticale (verso l'alto inizialmente)
                ),
                radius: rng.gen_range(2.0..6.0),
                color: format!("#{:02X}{:02X}{:02X}", 
                    rng.gen_range(100..255), // Rosso vivido
                    rng.gen_range(100..255), // Verde vivido
                    255                      // Blu al massimo
                ), 
            });
        }

        // Costanti fisiche globali
        let gravity = Vec2::new(0.0, 0.2); // Accelerazione verso il basso
        let damping = 0.9;                 // Coefficiente di restituzione (perdita energia agli urti)

        // 2. LOOP DELLA FISICA (Game Loop)
        // Questo ciclo gira all'infinito finché l'app è aperta.
        loop {
            // Iteriamo su ogni particella in modo mutabile
            for p in particles.iter_mut() {
                // A. Integrazione di Eulero Semi-Implicita
                // 1. Aggiorna velocità: v_new = v_old + accelerazione
                p.vel += gravity;
                // 2. Aggiorna posizione: x_new = x_old + v_new
                p.pos += p.vel;

                // B. Rilevamento Collisioni (Boundary Check)
                
                // Collisione Pavimento
                if p.pos.y > 480.0 {
                    p.pos.y = 480.0;       // Correggi compenetrazione
                    p.vel.y *= -1.0;       // Inverti velocità (rimbalzo)
                    p.vel = p.vel * damping; // Applica attrito
                }
                
                // Collisione Muri Laterali
                if p.pos.x > 490.0 { 
                    p.pos.x = 490.0; 
                    p.vel.x *= -1.0; 
                }
                if p.pos.x < 10.0 { 
                    p.pos.x = 10.0;  
                    p.vel.x *= -1.0; 
                }
            }

            // 3. INVIO DATI AL FRONTEND
            // Impacchettiamo lo stato e lo spediamo via evento.
            // .unwrap() viene usato perché se il canale si rompe, vogliamo saperlo.
            window.emit("update-physics", SystemState { 
                particles: particles.clone() 
            }).unwrap();

            // 4. CLOCK RATE
            // Dormiamo per ~16ms per mirare a circa 60 Frame Per Secondo (1000ms / 60 = 16.6ms)
            thread::sleep(time::Duration::from_millis(16));
        }
    });
}

fn main() {
    // Configurazione standard di Tauri
    tauri::Builder::default()
        // Registra il comando in modo che JS possa chiamare invoke('start_simulation')
        .invoke_handler(tauri::generate_handler![start_simulation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}