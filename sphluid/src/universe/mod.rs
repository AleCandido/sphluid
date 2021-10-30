use super::particle::Particle;

use num::Float;

pub mod evolve;
pub mod init;
pub mod io;

pub struct Universe<F: Float + Copy, const N: usize> {
    pub(crate) particles: Vec<Particle<F, N>>,
    pub(crate) time: usize,
}

impl<F: Float + Copy, const N: usize> Universe<F, N> {
    pub fn new(n: usize) -> Self {
        let x = [num::zero(); N];
        let p = [num::zero(); N];
        let r = num::one();

        let particles = vec![Particle::new(&x, &p, r); n];
        Self { particles, time: 0 }
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }
}
