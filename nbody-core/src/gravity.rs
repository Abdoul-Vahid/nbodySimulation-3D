use crate::bodies::Bodies;
use crate::vec3::Vec3;

pub fn compute_accelerations(b: &mut Bodies, g: f64, eps: f64) {
    let eps2 = eps * eps;
    // Destructure to get disjoint field borrows
    let Bodies { pos, acc, mass, .. } = b;

    for (i, acc_i) in acc.iter_mut().enumerate() {
        let pi = pos[i];
        let mut a = Vec3::ZERO;
        for (j, (&pj, &mj)) in pos.iter().zip(mass.iter()).enumerate() {
            if i == j { continue; }
            let d = pj - pi;
            let r2 = d.norm_sq() + eps2;
            let inv_r3 = 1.0 / (r2 * r2.sqrt());
            a += d * (g * mj * inv_r3);
        }
        *acc_i = a;
    }
}

pub fn kinetic_energy(b: &Bodies) -> f64 {
    b.vel.iter().zip(&b.mass).map(|(v, m)| 0.5 * m * v.norm_sq()).sum()
}

pub fn potential_energy(b: &Bodies, g: f64, eps: f64) -> f64 {
    let eps2 = eps * eps;
    let n = b.len();
    let mut u = 0.0;
    for i in 0..n {
        for j in (i + 1)..n {
            let r = ((b.pos[j] - b.pos[i]).norm_sq() + eps2).sqrt();
            u -= g * b.mass[i] * b.mass[j] / r;
        }
    }
    u
}

// This is useful to evaluate the corectness of the simulation.
pub fn angular_momentum(b: &Bodies) -> Vec3 {
    b.pos.iter().zip(&b.vel).zip(&b.mass)
        .map(|((&r, &v), &m)| r.cross(v) * m)
        .fold(Vec3::ZERO, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bodies::Bodies;
    use crate::vec3::Vec3;

    fn two_bodies() -> Bodies {
        let mut b = Bodies::new();
        b.push(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, 2.0);
        b.push(Vec3::new(1.0, 0.0, 0.0), Vec3::ZERO, 3.0);
        b
    }

    #[test]
    fn two_body_acceleration() {
        let mut b = two_bodies();
        compute_accelerations(&mut b, 1.0, 0.0);
        // a_0 = G*m_1/r^2 vers +x = 3 ; a_1 = G*m_0/r^2 vers -x = -2
        assert!((b.acc[0].x - 3.0).abs() < 1e-12);
        assert!((b.acc[1].x + 2.0).abs() < 1e-12);
        assert!(b.acc[0].y.abs() < 1e-12 && b.acc[0].z.abs() < 1e-12);
    }

    #[test]
    fn forces_sum_to_zero() {
        // 3e loi de Newton : somme des m_i * a_i = 0
        let mut b = Bodies::new();
        b.push(Vec3::new(0.0, 0.0, 0.0), Vec3::ZERO, 1.0);
        b.push(Vec3::new(1.0, 2.0, -1.0), Vec3::ZERO, 2.0);
        b.push(Vec3::new(-3.0, 0.5, 2.0), Vec3::ZERO, 0.5);
        b.push(Vec3::new(0.0, -1.0, 4.0), Vec3::ZERO, 3.0);
        compute_accelerations(&mut b, 1.0, 0.1);
        let mut total = Vec3::ZERO;
        for i in 0..b.len() {
            total += b.acc[i] * b.mass[i];
        }
        assert!(total.norm() < 1e-12);
    }

    #[test]
    fn softening_avoids_nan() {
        let mut b = Bodies::new();
        b.push(Vec3::ZERO, Vec3::ZERO, 1.0);
        b.push(Vec3::ZERO, Vec3::ZERO, 1.0); // même position
        compute_accelerations(&mut b, 1.0, 0.1);
        assert!(b.acc[0].norm().is_finite());
    }

    #[test]
    fn energies_two_bodies() {
        let mut b = two_bodies();
        assert!((potential_energy(&b, 1.0, 0.0) + 6.0).abs() < 1e-12); // -G m1 m2 / r
        b.vel[0] = Vec3::new(3.0, 4.0, 0.0);
        assert!((kinetic_energy(&b) - 25.0).abs() < 1e-12); // 1/2 * 2 * 25
    }

    #[test]
    fn angular_momentum_simple() {
        let mut b = Bodies::new();
        b.push(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), 3.0);
        // L = m (r x v) = 3 * (0, 0, 2)
        assert_eq!(angular_momentum(&b), Vec3::new(0.0, 0.0, 6.0));
    }
}