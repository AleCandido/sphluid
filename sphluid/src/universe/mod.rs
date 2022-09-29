use crate::numeric::Number;
use crate::particle::Particle;

use itertools::izip;
use ndarray::{stack, Array1, Array2, Axis};
use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1, PyReadonlyArray2};
use pyo3::prelude::*;

use std::path::PathBuf;

pub mod evolve;
pub mod io;

#[pyclass]
pub struct Universe {
    pub(crate) particles: Vec<Particle>,
    pub(crate) time: usize,
    pub(crate) path: Option<PathBuf>,
}

#[pymethods]
impl Universe {
    #[new]
    pub fn new(
        positions: PyReadonlyArray2<Number>,
        momenta: PyReadonlyArray2<Number>,
        radii: PyReadonlyArray1<Number>,
    ) -> Self {
        let pos = positions.to_owned_array();
        let mom = momenta.to_owned_array();
        let rad = radii.to_owned_array();

        let mut particles = Vec::new();

        for (x, p, r) in izip!(pos.rows(), mom.rows(), rad) {
            particles.push(Particle {
                x: x.clone().into_owned(),
                p: p.clone().into_owned(),
                r: r.clone(),
            })
        }

        Self {
            particles,
            time: 0,
            path: None,
        }
    }

    pub fn nparticles(&self) -> usize {
        self.particles.len()
    }

    pub fn naxes(&self) -> usize {
        self.particles[0].x.len()
    }

    pub fn positions<'py>(&self, py: Python<'py>) -> &'py PyArray2<Number> {
        self.pos().into_pyarray(py)
    }

    pub fn momenta<'py>(&self, py: Python<'py>) -> &'py PyArray2<Number> {
        self.mom().into_pyarray(py)
    }

    pub fn radii<'py>(&self, py: Python<'py>) -> &'py PyArray1<Number> {
        self.rad().into_pyarray(py)
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone().unwrap()
    }
}

impl Universe {
    pub fn pos(&self) -> Array2<Number> {
        let views_vec = self
            .particles
            .iter()
            .map(|p| p.x.view())
            .collect::<Vec<_>>();

        stack(Axis(0), &views_vec).unwrap()
    }

    pub fn mom(&self) -> Array2<Number> {
        let views_vec = self
            .particles
            .iter()
            .map(|p| p.p.view())
            .collect::<Vec<_>>();

        stack(Axis(0), &views_vec).unwrap()
    }

    pub fn rad(&self) -> Array1<Number> {
        self.particles.iter().map(|p| p.r).collect()
    }
}
