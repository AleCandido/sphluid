use anyhow::Result;

pub fn create<P>(filepath: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    // Create a new file with default settings
    let mut file = netcdf::create(filepath)?;
    let nparticles = 1e6 as usize;

    // We must create a dimension which corresponds to our data
    file.add_dimension("particle", nparticles)?;
    // These dimensions can also be unlimited and will be resized when writing
    file.add_unlimited_dimension("time")?;

    // A variable can now be declared, and must be created from the dimension names.
    let mut var = file.add_variable::<f32>("x", &["time", "particle"])?;
    // Metadata can be added to the variable
    var.add_attribute("description", "position")?;

    // Data can then be created and added to the variable
    let data: Vec<f32> = vec![42.; nparticles];
    var.put_values(&data, Some(&[0, 0]), None)?;
    // (This puts data at offset (0, 0) until all the data has been consumed)

    // Values can be added along the unlimited dimension, which
    // resizes along the `time` axis
    var.put_values(&data, Some(&[1, 0]), None)?;

    let mut vary = file.add_variable::<f32>("y", &["time", "particle"])?;
    vary.put_values(&data, Some(&[0, 0]), None)?;

    println!("Written history.nc");

    Ok(())
}

pub fn append<P>(filepath: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    // Open an existing filea in append mode
    let mut file = netcdf::append(filepath)?;

    let mut var = file.variable_mut("crab_coolness_level").unwrap();

    let data: Vec<i32> = vec![42; 10];
    var.put_values(&data, Some(&[2, 0]), None)?;

    println!("Appended data to crabs.nc");

    Ok(())
}
