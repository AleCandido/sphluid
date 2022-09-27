use super::universe::Universe;

use anyhow::Result;
use num::Float;

pub fn create<P, F>(filepath: &P, uni: &Universe<F>) -> Result<()>
where
    P: AsRef<std::path::Path>,
    F: Float + netcdf::Numeric,
{
    // Create a new file with default settings
    let mut file = netcdf::create(filepath)?;

    let naxes = uni.naxes();

    println!("N: {}, particles: {}", naxes, uni.nparticles());
    file.add_unlimited_dimension("time")?;
    file.add_dimension("axis", naxes)?;
    file.add_dimension("particle", uni.nparticles())?;

    let mut var_x = file.add_variable::<F>("x", &["time", "axis", "particle"])?;
    var_x.add_attribute("description", "positions")?;
    let mut var_p = file.add_variable::<F>("p", &["time", "axis", "particle"])?;
    var_p.add_attribute("description", "momenta")?;
    let mut var_r = file.add_variable::<F>("r", &["time", "particle"])?;
    var_r.add_attribute("description", "radii")?;

    println!("created {}", filepath.as_ref().display());

    Ok(())
}
