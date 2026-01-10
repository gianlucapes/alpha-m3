use tauri::{Emitter, Manager, State, WebviewWindow};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time;
use rand::Rng;

mod vector;
use vector::Vec2;

#[derive(Clone, serde::Serialize)]
struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f64,
    color: String,
}

#[derive(Clone, serde::Serialize)]
struct SystemState {
    particles: Vec<Particle>,
}

// CAMBIAMENTO 1: Invece di 'should_reset', usiamo 'is_running'
struct SimulationControl {
    is_running: Arc<AtomicBool>,
}

// CAMBIAMENTO 2: Nuovo comando per FERMARE tutto
#[tauri::command]
fn stop_simulation(state: State<SimulationControl>) {
    // Impostiamo a FALSE. Il thread leggerà questo valore, uscirà dal loop e morirà.
    state.is_running.store(false, Ordering::Relaxed);
}

#[tauri::command]
fn start_simulation(window: WebviewWindow, state: State<SimulationControl>) {
    // 1. Prima di far partire un nuovo thread, diciamo che siamo "in esecuzione"
    state.is_running.store(true, Ordering::Relaxed);
    
    // Cloniamo il puntatore al flag per passarlo al thread
    let is_running = state.is_running.clone();

    thread::spawn(move || {
        let mut particles: Vec<Particle> = Vec::new();

        // Inizializzazione (Big Bang)
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            particles.push(Particle {
                pos: Vec2::new(rng.gen_range(50.0..450.0), rng.gen_range(50.0..200.0)),
                vel: Vec2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..0.0)),
                radius: rng.gen_range(6.0..12.0),
                color: format!("#{:02X}{:02X}FF", rng.gen_range(100..255), rng.gen_range(100..255)),
            });
        }

        let gravity = Vec2::new(0.0, 0.8);
        let damping = 0.9; 

        // CAMBIAMENTO 3: Il loop controlla se deve continuare a vivere
        loop {
            // Se 'is_running' è diventato FALSE (premuto Stop), usciamo dal ciclo
            if !is_running.load(Ordering::Relaxed) {
                break; // Il thread finisce qui.
            }

            // --- FISICA ---
            for p in particles.iter_mut() {
                p.vel += gravity;
                p.pos += p.vel;

                if p.pos.y > 480.0 {
                    p.pos.y = 480.0;
                    p.vel.y *= -1.0;
                    p.vel = p.vel * damping; 
                }
                if p.pos.x > 490.0 { p.pos.x = 490.0; p.vel.x *= -1.0; }
                if p.pos.x < 10.0 { p.pos.x = 10.0; p.vel.x *= -1.0; }
            }

            for i in 0..particles.len() {
                let (head, tail) = particles.split_at_mut(i + 1);
                let p1 = &mut head[i];
                for p2 in tail {
                    let dx = p2.pos.x - p1.pos.x;
                    let dy = p2.pos.y - p1.pos.y;
                    let dist_sq = dx*dx + dy*dy;
                    let min_dist = p1.radius + p2.radius;

                    if dist_sq < min_dist * min_dist {
                        let dist = dist_sq.sqrt();
                        let nx = dx / dist;
                        let ny = dy / dist;
                        let overlap = (min_dist - dist) * 0.5;
                        p1.pos.x -= nx * overlap; p1.pos.y -= ny * overlap;
                        p2.pos.x += nx * overlap; p2.pos.y += ny * overlap;

                        let dvx = p2.vel.x - p1.vel.x;
                        let dvy = p2.vel.y - p1.vel.y;
                        let vel_norm = dvx * nx + dvy * ny;

                        if vel_norm > 0.0 { continue; }

                        let impulse = -1.9 * vel_norm / 2.0; 
                        let impulse_vec = Vec2::new(nx * impulse, ny * impulse);
                        p1.vel += impulse_vec * -1.0; 
                        p2.vel += impulse_vec;
                    }
                }
            }

            if let Err(_) = window.emit("update-physics", SystemState { particles: particles.clone() }) {
                break;
            }

            thread::sleep(time::Duration::from_millis(16));
        }
    });
}

fn main() {
    // Inizializziamo il flag a false (simulazione ferma)
    let is_running = Arc::new(AtomicBool::new(false));

    tauri::Builder::default()
        .manage(SimulationControl { is_running: is_running.clone() })
        // Registriamo STOP e START
        .invoke_handler(tauri::generate_handler![stop_simulation, start_simulation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}