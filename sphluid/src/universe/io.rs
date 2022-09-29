use super::Universe;
use crate::numeric::Number;
use crate::particle::Particle;

use itertools::izip;
use ndarray::Axis;
use pyo3::prelude::*;

use std::path::PathBuf;

#[pymethods]
impl Universe {
    #[staticmethod]
    pub fn from_origin(path: PathBuf) -> Self {
        Self::from_time(path, 0)
    }

    #[staticmethod]
    pub fn from_time(path: PathBuf, time: usize) -> Self {
        let file = netcdf::open(&path).unwrap();

        let nparticles = file.dimension("particle").unwrap().len();
        let naxes = file.dimension("axis").unwrap().len();

        println!("{}", naxes);

        let pos = file
            .variable("positions")
            .unwrap()
            .values::<Number>(Some(&[time, 0, 0]), Some(&[1, nparticles, naxes]))
            .unwrap();
        let mom = file
            .variable("momenta")
            .unwrap()
            .values::<Number>(Some(&[time, 0, 0]), Some(&[1, nparticles, naxes]))
            .unwrap();
        let rad = file
            .variable("radii")
            .unwrap()
            .values::<Number>(Some(&[time, 0]), Some(&[1, nparticles]))
            .unwrap();

        let mut particles = Vec::new();

        for (x, p, r) in izip!(
            pos.index_axis(Axis(0), 0).rows(),
            mom.index_axis(Axis(0), 0).rows(),
            rad.index_axis(Axis(0), 0)
        ) {
            particles.push(Particle {
                x: x.clone().into_owned(),
                p: p.clone().into_owned(),
                r: r.clone(),
            })
        }

        Self {
            particles,
            time,
            path: Some(path),
        }
    }

    /// Save snapshot of current configuration
    pub fn dump(&self, filepath: PathBuf) {
        let mut file = netcdf::append(&filepath).unwrap();

        let naxes = self.naxes();

        let mut var_x = file.variable_mut("positions").unwrap();
        for i in 0..naxes {
            let axis_position: Vec<Number> = self.particles.iter().map(|p| p.x[i]).collect();

            var_x
                .put_values(
                    &axis_position,
                    Some(&[self.time, i, 0]),
                    Some(&[1, 1, self.nparticles()]),
                )
                .unwrap();
        }

        let mut var_p = file.variable_mut("momenta").unwrap();
        for i in 0..naxes {
            let axis_momentum: Vec<Number> = self.particles.iter().map(|p| p.p[i]).collect();
            var_p
                .put_values(
                    &axis_momentum,
                    Some(&[self.time, i, 0]),
                    Some(&[1, 1, self.nparticles()]),
                )
                .unwrap();
        }

        let mut var_r = file.variable_mut("radii").unwrap();
        let radii: Vec<Number> = self.particles.iter().map(|p| p.r).collect();
        var_r
            .put_values(&radii, Some(&[self.time, 0]), Some(&[1, self.nparticles()]))
            .unwrap();
    }
}
