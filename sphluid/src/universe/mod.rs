use super::particle::Particle;

use num::Float;

use std::path::Path;

pub mod evolve;
pub mod init;
pub mod io;

pub struct Universe<F: Float + Copy, const N: usize> {
    pub(crate) particles: Vec<Particle<F, N>>,
    pub(crate) time: usize,
}

impl<F: Float + Copy, const N: usize> Universe<F, N> {
    pub fn new(path: &Path) -> Self {
        Self::from_time(path, 0)
    }

    pub fn from_time(path: &Path, time: usize) -> Self {
        let particles = vec![Particle::new(&x, &p, r); n];
        Self { particles, time }
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }
}
