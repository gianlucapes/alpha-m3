use super::domain::Particle; 
use crate::math::vector::Vec2;

// Parametri globali della simulazione (Costanti ambientali)
const GRAVITY: Vec2 = Vec2 { x: 0.0, y: 0.8 }; // g (accelerazione)
const RESTITUTION_COEFF: f64 = 0.9;            // Dissipazione energia (urti anelastici)
const BOUNDARY_WIDTH: f64 = 500.0;
const BOUNDARY_HEIGHT: f64 = 500.0;

/// Esegue un singolo passo di integrazione temporale (Time Step).
pub fn solve_step(particles: &mut Vec<Particle>) {
    // 1. Integrazione delle Forze Esterne
    integrate_forces(particles);

    // 2. Risoluzione Interazioni Locali (Collisioni tra particelle)
    resolve_interactions(particles);

    // 3. Risoluzione Vincoli Geometrici (Pareti)
    apply_boundary_conditions(particles);

    // 4. Aggiornamento Posizione (Cinematica)
    integrate_motion(particles);
}

fn integrate_forces(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        // F = ma -> a = F/m (Qui assumiamo gravità costante per tutti)
        p.vel += GRAVITY;
    }
}

fn integrate_motion(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        p.pos += p.vel;
    }
}

fn apply_boundary_conditions(particles: &mut Vec<Particle>) {
    for p in particles.iter_mut() {
        // Vincolo Pavimento (Y max)
        if p.pos.y > BOUNDARY_HEIGHT - 20.0 { // 20.0 padding estetico
            p.pos.y = BOUNDARY_HEIGHT - 20.0;
            p.vel.y *= -1.0 * RESTITUTION_COEFF;
        }
        
        // Vincoli Pareti Laterali (X min/max)
        if p.pos.x > BOUNDARY_WIDTH - 10.0 { 
            p.pos.x = BOUNDARY_WIDTH - 10.0; 
            p.vel.x *= -1.0 * RESTITUTION_COEFF; 
        }
        if p.pos.x < 10.0 { 
            p.pos.x = 10.0;  
            p.vel.x *= -1.0 * RESTITUTION_COEFF; 
        }
    }
}

fn resolve_interactions(particles: &mut Vec<Particle>) {
    let len = particles.len();
    for i in 0..len {
        let (head, tail) = particles.split_at_mut(i + 1);
        let p1 = &mut head[i];

        for p2 in tail {
            // Distanza Euclidea
            let dx = p2.pos.x - p1.pos.x;
            let dy = p2.pos.y - p1.pos.y;
            let dist_sq = dx*dx + dy*dy;
            let min_dist = p1.radius + p2.radius;

            // Rilevamento Sovrapposizione
            if dist_sq < min_dist * min_dist {
                let dist = dist_sq.sqrt();
                let nx = dx / dist; // Versore Normale X
                let ny = dy / dist; // Versore Normale Y

                // A. Risoluzione Penetrazione (Position Correction)
                let overlap = min_dist - dist;
                let separation_factor = 0.5; // 50% a testa se masse uguali
                let corr_x = nx * overlap * separation_factor;
                let corr_y = ny * overlap * separation_factor;
                
                p1.pos.x -= corr_x; p1.pos.y -= corr_y;
                p2.pos.x += corr_x; p2.pos.y += corr_y;

                // B. Risoluzione Impulso (Conservazione Quantità di Moto)
                let dvx = p2.vel.x - p1.vel.x;
                let dvy = p2.vel.y - p1.vel.y;
                let vel_along_normal = dvx * nx + dvy * ny;

                // Se le velocità divergono, non c'è urto
                if vel_along_normal > 0.0 { continue; }

                // Calcolo scalare dell'impulso j
                // Formula fisica per urto 1D lungo la normale
                // j = -(1 + e) * v_rel / (1/m1 + 1/m2)
                let inv_mass1 = 1.0 / p1.mass;
                let inv_mass2 = 1.0 / p2.mass;
                
                let j = -(1.0 + RESTITUTION_COEFF) * vel_along_normal;
                let j = j / (inv_mass1 + inv_mass2);

                // Applicazione Impulso
                let impulse_x = j * nx;
                let impulse_y = j * ny;

                // v_new = v_old - (Impulso / massa)
                p1.vel.x -= impulse_x * inv_mass1;
                p1.vel.y -= impulse_y * inv_mass1;
                p2.vel.x += impulse_x * inv_mass2;
                p2.vel.y += impulse_y * inv_mass2;
            }
        }
    }
}