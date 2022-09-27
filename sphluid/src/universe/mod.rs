use super::particle::Particle;

use netcdf::error::Result;
use num::Float;

use std::path::Path;

pub mod evolve;
pub mod io;

pub struct Universe<F: Float + Copy> {
    pub(crate) particles: Vec<Particle<F>>,
    pub(crate) time: usize,
}

impl<F: Float + Copy> Universe<F> {
    pub fn new(path: &Path) -> Result<Self> {
        Self::from_time(path, 0)
    }

    pub fn from_time(path: &Path, time: usize) -> Result<Self> {
        let file = netcdf::open(path)?;

        let nparticles = file.dimension("particle").unwrap().len();
        let naxes = file.dimension("axis").unwrap().len();

        let pos = file
            .variable("positions")
            .unwrap()
            .values(Some(&[time, 0, 0]), Some(&[1, naxes, nparticles]))?;
        let mom = file
            .variable("momenta")
            .unwrap()
            .values(Some(&[time, 0, 0]), Some(&[1, naxes, nparticles]))?;
        let rad = file
            .variable("radii")
            .unwrap()
            .values(Some(&[time, 0]), Some(&[1, nparticles]))?;

        Ok(Self { particles, time })
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }

    pub fn naxes(&self) -> usize {
        self.particles[0].x.len()
    }
}
