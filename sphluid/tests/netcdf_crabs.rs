use anyhow::Result;
use std::{env, fs};

fn create<P>(filepath: P) -> Result<()>
where
    P: AsRef<std::path::Path>,
{
    // Create a new file with default settings
    let mut file = netcdf::create(filepath)?;

    // We must create a dimension which corresponds to our data
    file.add_dimension("ncrabs", 10)?;
    // These dimensions can also be unlimited and will be resized when writing
    file.add_unlimited_dimension("time")?;

    // A variable can now be declared, and must be created from the dimension names.
    let mut var = file.add_variable::<i32>("crab_coolness_level", &["time", "ncrabs"])?;
    // Metadata can be added to the variable
    var.add_attribute("units", "Kelvin")?;
    var.add_attribute("add_offset", 273.15_f32)?;

    // Data can then be created and added to the variable
    let data: Vec<i32> = vec![42; 10];
    var.put_values(&data, Some(&[0, 0]), None)?;
    // (This puts data at offset (0, 0) until all the data has been consumed)

    // Values can be added along the unlimited dimension, which
    // resizes along the `time` axis
    var.put_values(&data, Some(&[1, 0]), None)?;

    println!("Written crabs.nc");

    Ok(())
}

fn append<P>(filepath: P) -> Result<()>
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

#[test]
fn test_create() -> anyhow::Result<()> {
    let mut path = env::temp_dir();
    path.push("crabs.nc");
    create(&path)?;

    fs::remove_file(path)?;

    Ok(())
}

#[test]
fn test_append() -> anyhow::Result<()> {
    let mut path = env::temp_dir();
    path.push("crabs-append.nc");
    create(&path)?;
    append(&path)?;

    fs::remove_file(path)?;

    Ok(())
}
