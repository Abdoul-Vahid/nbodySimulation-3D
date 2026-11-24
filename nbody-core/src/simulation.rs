use crate::bodies::Bodies;
use crate::gravity::{compute_accelerations, kinetic_energy, potential_energy};

pub struct Simulation {
    pub bodies: Bodies,
    pub g: f64,
    pub eps: f64,
    pub dt: f64,
}

impl Simulation {
    pub fn new(mut bodies: Bodies, g: f64, eps: f64, dt: f64) -> Self {
        // Invariant : `acc` is always coherent with `pos`
        compute_accelerations(&mut bodies, g, eps);
        Self { bodies, g, eps, dt }
    }

    /// One Kick-Drift-Kick step. Single force evaluation per step.
    pub fn step(&mut self) {
        let half_dt = 0.5 * self.dt;
        let b = &mut self.bodies;

        for (v, a) in b.vel.iter_mut().zip(&b.acc) { *v += *a * half_dt; }   // kick
        for (p, v) in b.pos.iter_mut().zip(&b.vel) { *p += *v * self.dt; }   // drift
        compute_accelerations(b, self.g, self.eps);                           // Momentum at t+dt
        for (v, a) in b.vel.iter_mut().zip(&b.acc) { *v += *a * half_dt; }   // kick
    }

    pub fn total_energy(&self) -> f64 {
        kinetic_energy(&self.bodies) + potential_energy(&self.bodies, self.g, self.eps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn circular_orbit_conserves_energy_and_closes() {
        let mut b = Bodies::new();
        b.push(Vec3::ZERO, Vec3::ZERO, 1.0);
        b.push(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1e-6);

        let dt = 1e-3;
        let mut sim = Simulation::new(b, 1.0, 0.0, dt);
        let e0 = sim.total_energy();

        let period = 2.0 * std::f64::consts::PI;
        let steps = (10.0 * period / dt).round() as usize; // 10 orbites
        let mut max_rel_err: f64 = 0.0;
        for _ in 0..steps {
            sim.step();
            max_rel_err = max_rel_err.max(((sim.total_energy() - e0) / e0).abs());
        }
        assert!(max_rel_err < 1e-6, "dérive d'énergie : {max_rel_err}");

        let d = sim.bodies.pos[1] - sim.bodies.pos[0];
        assert!((d.norm() - 1.0).abs() < 1e-3, "rayon = {}", d.norm());
    }
}