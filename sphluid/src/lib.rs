use pyo3::prelude::*;

pub mod numeric;
pub mod particle;
pub mod universe;

#[pymodule]
fn sphluid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<particle::Particle>()?;
    m.add_class::<universe::Universe>()?;
    m.add("version", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
