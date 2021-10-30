use anyhow::Result;
use sphluid::io::create;

fn main() -> Result<()> {
    create("history.nc")?;
    Ok(())
}
