use crate::numeric::Number;

use super::Universe;

use anyhow::Result;

impl Universe {
    pub fn snapshot<P>(&self, filepath: &P) -> Result<()>
    where
        P: AsRef<std::path::Path>,
    {
        let mut file = netcdf::append(&filepath)?;

        let naxes = self.naxes();

        let mut var_x = file.variable_mut("x").unwrap();
        for i in 0..naxes {
            let axis_position: Vec<Number> = self.particles.iter().map(|p| p.x[i]).collect();

            var_x.put_values(
                &axis_position,
                Some(&[self.time, i, 0]),
                Some(&[1, 1, self.nparticles()]),
            )?;
        }

        let mut var_p = file.variable_mut("p").unwrap();
        for i in 0..naxes {
            let axis_momentum: Vec<Number> = self.particles.iter().map(|p| p.p[i]).collect();
            var_p.put_values(
                &axis_momentum,
                Some(&[self.time, i, 0]),
                Some(&[1, 1, self.nparticles()]),
            )?;
        }

        let mut var_r = file.variable_mut("r").unwrap();
        let radii: Vec<Number> = self.particles.iter().map(|p| p.r).collect();
        var_r.put_values(&radii, Some(&[self.time, 0]), Some(&[1, self.nparticles()]))?;

        Ok(())
    }
}
