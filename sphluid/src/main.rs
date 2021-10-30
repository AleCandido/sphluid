use anyhow::Result;

use sphluid::io::create;
use sphluid::universe::init;

fn main() -> Result<()> {
    let mut uni = init::random::<f64, 3>(1e6 as usize);

    let history_file = "history.nc";
    create(&history_file, &uni)?;

    uni.snapshot(&history_file)?;
    uni.trivial();
    uni.snapshot(&history_file)?;

    Ok(())
}
