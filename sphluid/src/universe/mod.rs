use super::particle::Particle;
use crate::numeric::Number;

use pyo3::prelude::*;

use std::path::PathBuf;

pub mod evolve;
pub mod io;

#[pyclass]
pub struct Universe {
    pub(crate) particles: Vec<Particle>,
    pub(crate) time: usize,
}

#[pymethods]
impl Universe {
    #[staticmethod]
    pub fn new(path: PathBuf) -> Self {
        Self::from_time(path, 0)
    }

    #[staticmethod]
    pub fn from_time(path: PathBuf, time: usize) -> Self {
        let file = netcdf::open(path).unwrap();

        let nparticles = file.dimension("particle").unwrap().len();
        let naxes = file.dimension("axis").unwrap().len();

        let pos = file
            .variable("positions")
            .unwrap()
            .values::<Number>(Some(&[time, 0, 0]), Some(&[1, naxes, nparticles]))
            .unwrap();
        let mom = file
            .variable("momenta")
            .unwrap()
            .values::<Number>(Some(&[time, 0, 0]), Some(&[1, naxes, nparticles]))
            .unwrap();
        let rad = file
            .variable("radii")
            .unwrap()
            .values::<Number>(Some(&[time, 0]), Some(&[1, nparticles]))
            .unwrap();

        let particles = Vec::new();

        // for x in pos.iter() {}

        Self { particles, time }
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }

    pub fn naxes(&self) -> usize {
        self.particles[0].x.len()
    }
}
