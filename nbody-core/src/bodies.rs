use crate::vec3::Vec3;

/// Structure of Arrays. Following invariant needs to be true: the vectors have all the same size.
#[derive(Debug, Clone, Default)]
pub struct Bodies {
    pub pos: Vec<Vec3>,
    pub vel: Vec<Vec3>,
    pub acc: Vec<Vec3>,
    pub mass: Vec<f64>,
}

// A new body is always pushed with pos, vel, acc and mass.
impl Bodies {
    pub fn new() -> Self { Self::default() }
    pub fn len(&self) -> usize { self.mass.len() }
    pub fn is_empty(&self) -> bool { self.mass.is_empty() }

    pub fn push(&mut self, pos: Vec3, vel: Vec3, mass: f64) {
        self.pos.push(pos);
        self.vel.push(vel);
        self.acc.push(Vec3::ZERO);
        self.mass.push(mass);
    }
}