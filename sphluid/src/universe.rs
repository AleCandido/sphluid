use super::particle::Particle;

use anyhow::Result;
use num::Float;

use std::path::Path;

pub struct Universe<F: Float + Copy, const N: usize> {
    pub(crate) particles: Vec<Particle<F, N>>,
    pub(crate) time: usize,
}

impl<F: Float + Copy, const N: usize> Universe<F, N>
where
    F: netcdf::Numeric,
{
    pub fn new(n: usize) -> Self {
        let x = [num::zero(); N];
        let p = [num::zero(); N];
        let r = num::one();

        let particles = vec![Particle::new(&x, &p, r); n];
        Self { particles, time: 0 }
    }

    pub fn snapshot(&self, filepath: &Path) -> Result<()> {
        let mut file = netcdf::append(&filepath)?;

        let mut var = file.variable_mut("").unwrap();

        let data: Vec<F> = self.particles.iter().map(|p| p.x[0]).collect();
        var.put_values(&data, Some(&[self.time, 0]), None)?;

        Ok(())
    }
}
